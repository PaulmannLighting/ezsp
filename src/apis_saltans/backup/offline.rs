//! Successful capture and restore for an explicitly selected offline token profile.

use std::collections::BTreeSet;

use apis_saltans_hw::{
    BackupError, Error, NetworkBackup, RestoreFailureState, RestoreOptions, RestoreReport,
};

use super::image::{TOKEN_EXTENSION, TokenImage, TokenRecord};
use super::profile::{OfflineTokenProfile, incompatible};
use super::{EMBER_BACKUP_NAMESPACE, EmberSecurityState, RestorePlan};
use crate::ember::network::Status;
use crate::ezsp::config;
use crate::ezsp::value::Id;
use crate::{
    Configuration, GetValueExt, Identity, IdentityChange, Networking, TokenInterface, Utilities,
};

const CLASSIC_KEY_STORAGE: &[u8] = &[0];
const ERASED_EUI64: [u8; 8] = [u8::MAX; 8];

pub(super) async fn capture<T>(
    target: &mut T,
    profile: &OfflineTokenProfile,
) -> Result<NetworkBackup, Error>
where
    T: Configuration + Networking + Utilities + TokenInterface + Send,
{
    check_target(target, profile).await?;
    let ieee = target.get_eui64().await?;
    let image = read_image(target, profile).await?;
    let verification = read_image(target, profile).await?;
    if image != verification || target.get_eui64().await? != ieee {
        return Err(incompatible("offline token state changed during capture").into());
    }
    check_target(target, profile).await?;
    Ok(profile.snapshot(&image, ieee.into())?)
}

pub(super) async fn validate<T>(
    target: &mut T,
    profile: &OfflineTokenProfile,
    plan: &RestorePlan,
    options: RestoreOptions,
) -> Result<TokenImage, Error>
where
    T: Configuration + Networking + Utilities + TokenInterface + Send,
{
    // Decode, advance every embedded counter, and check all duplicate normalized
    // state before any target I/O, let alone persistent writes.
    let image = prepare_image(profile, plan, options)?;
    check_target(target, profile).await?;
    let keys = target
        .get_configuration_value(config::Id::KeyTableSize)
        .await?;
    let children = target
        .get_configuration_value(config::Id::MaxEndDeviceChildren)
        .await?;
    if plan
        .backup()
        .devices()
        .iter()
        .filter(|device| device.link_key().is_some())
        .count()
        > usize::from(keys)
        || plan
            .backup()
            .devices()
            .iter()
            .filter(|device| device.is_child())
            .count()
            > usize::from(children)
    {
        return Err(incompatible("target key or child table is too small").into());
    }
    let wanted = plan.backup().network().coordinator_ieee().into();
    if target.get_eui64().await? != wanted {
        let custom = target.get_custom_eui64().await?;
        if custom.into_array() != ERASED_EUI64 {
            return Err(incompatible(
                "target identity is occupied or requires a caller-owned reset",
            )
            .into());
        }
    }
    Ok(image)
}

pub(super) async fn restore<T>(
    target: &mut T,
    profile: &OfflineTokenProfile,
    plan: &RestorePlan,
    options: RestoreOptions,
) -> Result<RestoreReport, Error>
where
    T: Configuration + Networking + Utilities + TokenInterface + Send,
{
    let image = validate(target, profile, plan, options)
        .await
        .map_err(|error| failure(false, error))?;
    let mut modified = false;
    let result = apply(target, profile, plan, &image, &mut modified).await;
    result.map_err(|error| failure(modified, error))
}

async fn apply<T>(
    target: &mut T,
    profile: &OfflineTokenProfile,
    plan: &RestorePlan,
    image: &TokenImage,
    modified: &mut bool,
) -> Result<RestoreReport, Error>
where
    T: Configuration + Networking + Utilities + TokenInterface + Send,
{
    let wanted = plan.backup().network().coordinator_ieee().into();
    if target.get_eui64().await? != wanted {
        // Validation checked an erased custom token. Treat even a lost response
        // as possible mutation, and never silently restart a pending identity.
        *modified = true;
        match target
            .set_custom_eui64(wanted)
            .await
            .map_err(Error::backend)?
        {
            IdentityChange::Unchanged | IdentityChange::Applied => {}
            IdentityChange::RestartRequired => {
                return Err(
                    incompatible("restored IEEE address requires a caller-owned reset").into(),
                );
            }
        }
    }
    for record in &image.records {
        *modified = true;
        target
            .set_token_data(record.key, record.index, record.data.clone())
            .await?;
    }
    let verification = read_image(target, profile).await?;
    if verification != *image {
        return Err(incompatible("persistent token readback differs after restoration").into());
    }
    check_target(target, profile).await?;
    if target.get_eui64().await? != wanted {
        return Err(incompatible("operational IEEE address differs after restoration").into());
    }
    let verified = profile.snapshot(&verification, wanted.into())?;
    check_normalized(&verified, plan)?;
    Ok(RestoreReport::from_verified_backup(plan.backup()))
}

async fn check_target<T>(target: &mut T, profile: &OfflineTokenProfile) -> Result<(), Error>
where
    T: Configuration + Networking + TokenInterface + Send,
{
    if target.network_state().await? != Status::NoNetwork {
        return Err(BackupError::InvalidState.into());
    }
    if target.get_value(Id::KeyStorageVersion).await?.as_slice() != CLASSIC_KEY_STORAGE {
        return Err(incompatible("offline token backup requires classic key storage").into());
    }
    let firmware = target
        .get_ember_version()
        .await?
        .map_err(|_| incompatible("malformed firmware version"))?;
    if firmware.to_string() != profile.firmware {
        return Err(incompatible("firmware differs from the offline token profile").into());
    }
    if usize::from(target.get_token_count().await?) != profile.tokens.len() {
        return Err(incompatible("target token count differs from the profile").into());
    }
    for (index, expected) in profile.tokens.iter().enumerate() {
        let index = u8::try_from(index).map_err(|_| incompatible("too many tokens"))?;
        if target.get_token_info(index).await? != *expected {
            return Err(incompatible("target token catalog differs from the profile").into());
        }
    }
    Ok(())
}

async fn read_image<T>(target: &mut T, profile: &OfflineTokenProfile) -> Result<TokenImage, Error>
where
    T: TokenInterface + Send,
{
    let mut records = Vec::new();
    for token in &profile.tokens {
        for index in 0..u32::from(token.array_size()) {
            let data = target.get_token_data(token.nvm3_key(), index).await?;
            if data.size() != u32::from(token.size()) {
                return Err(incompatible("token data size differs from the catalog").into());
            }
            records.push(TokenRecord {
                key: token.nvm3_key(),
                index,
                data,
            });
        }
    }
    let image = TokenImage {
        id: profile.id.clone(),
        records,
    };
    image.validate(profile)?;
    Ok(image)
}

fn prepare_image(
    profile: &OfflineTokenProfile,
    plan: &RestorePlan,
    options: RestoreOptions,
) -> Result<TokenImage, BackupError> {
    let backup = plan.backup();
    if backup.source().firmware() != profile.firmware {
        return Err(incompatible(
            "source firmware differs from the offline token profile",
        ));
    }
    let extension = backup
        .extensions()
        .iter()
        .find(|extension| {
            extension.namespace() == EMBER_BACKUP_NAMESPACE && extension.name() == TOKEN_EXTENSION
        })
        .ok_or(incompatible("complete token image extension is required"))?;
    let mut image = TokenImage::decode(extension)?;
    image.validate(profile)?;
    let original = profile.snapshot(&image, backup.network().coordinator_ieee())?;
    let expected = RestorePlan::prepare(&original, options)?;
    if expected.backup() != backup {
        return Err(incompatible(
            "normalized backup and embedded token image disagree",
        ));
    }
    image.advance(
        profile.network.tx_counter,
        options.network_counter_advance().get(),
    )?;
    let mut counters = BTreeSet::new();
    counters.insert(profile.security.tx_counter);
    for device in &profile.devices {
        let address: crate::ember::Eui64 = image.read(device.ieee)?;
        if address == crate::ember::Eui64::default() || address.into_array() == ERASED_EUI64 {
            continue;
        }
        if let Some(key) = &device.link_key {
            counters.insert(key.tx_counter);
        }
    }
    for field in counters {
        image.advance(field, options.link_counter_advance().get())?;
    }
    check_normalized(
        &profile.snapshot(&image, backup.network().coordinator_ieee())?,
        plan,
    )?;
    Ok(image)
}

fn check_normalized(actual: &NetworkBackup, plan: &RestorePlan) -> Result<(), BackupError> {
    let expected = plan.backup();
    if actual.network() != expected.network()
        || actual.network_key() != expected.network_key()
        || actual.devices() != expected.devices()
        || actual.additional_network_keys() != expected.additional_network_keys()
        || actual.source() != expected.source()
        || EmberSecurityState::from_extension(&actual.extensions()[0])? != *plan.security()
    {
        return Err(incompatible(
            "restored normalized state differs from the prepared snapshot",
        ));
    }
    Ok(())
}

fn failure(modified: bool, error: Error) -> Error {
    BackupError::restore_failed(
        if modified {
            RestoreFailureState::PossiblyModified
        } else {
            RestoreFailureState::Unchanged
        },
        error,
    )
    .into()
}
