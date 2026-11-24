//! Catalog-free EZSP v8 network recovery using standard stack commands.
//!
//! Recovery forms a running network. Only the coordinator identity, active network credentials,
//! persistent partner keys and outgoing counter floors are applied. Child and address records
//! help describe the original network. Firmware rediscovers them as devices rejoin.

use apis_saltans_hw::{BackupError, Error, NetworkBackup, RestoreFailureState};

pub(super) use self::capture::capture;
pub(super) use self::restore::{restore, validate};
use super::{EMBER_BACKUP_NAMESPACE, EMBER_SECURITY_EXTENSION, EmberSecurityState};
use crate::ember::security::initial;
use crate::ember::{self, key};
use crate::ezsp::config;
use crate::frame::parameters::security::get_key_table_entry;
use crate::{Communicate, Configuration};

mod capture;
mod restore;
#[cfg(test)]
mod tests;

pub(super) const VERSION: u8 = 8;
const STACK: &str = "emberznet-ezsp-v8-recovery";
const INVALID_KEY: &str = "invalid or unreadable EZSP v8 key";
const INVALID_TABLE_SIZE: &str = "EZSP v8 table exceeds its index range";
const BASE_SECURITY_FLAGS: initial::Bitmask = initial::Bitmask::HAVE_NETWORK_KEY
    .union(initial::Bitmask::HAVE_PRECONFIGURED_KEY)
    .union(initial::Bitmask::HAVE_TRUST_CENTER_EUI64)
    .union(initial::Bitmask::REQUIRE_ENCRYPTED_KEY)
    .union(initial::Bitmask::TRUST_CENTER_GLOBAL_LINK_KEY)
    .union(initial::Bitmask::NO_FRAME_COUNTER_RESET);

fn validate_snapshot(snapshot: &NetworkBackup) -> Result<EmberSecurityState, BackupError> {
    snapshot.validate()?;
    if snapshot.source().stack() != STACK || !snapshot.additional_network_keys().is_empty() {
        return Err(BackupError::Incompatible(
            "expected an EZSP v8 recovery snapshot",
        ));
    }
    let mut extensions = snapshot.backend_extensions();
    let extension = extensions.next().ok_or(BackupError::Incompatible(
        "expected an Ember security extension",
    ))?;
    if extension.namespace() != EMBER_BACKUP_NAMESPACE
        || extension.name() != EMBER_SECURITY_EXTENSION
        || extensions.next().is_some()
    {
        return Err(BackupError::Incompatible(
            "unsupported EZSP v8 backup extensions",
        ));
    }
    let security = EmberSecurityState::from_extension(extension)?;
    if security.flags() != BASE_SECURITY_FLAGS
        && security.flags()
            != BASE_SECURITY_FLAGS | initial::Bitmask::TRUST_CENTER_USES_HASHED_LINK_KEY
    {
        return Err(BackupError::Incompatible(
            "unsupported EZSP v8 restore security flags",
        ));
    }
    validate_key(security.trust_center_key().expose_secret())?;
    if !valid_ieee(snapshot.network().coordinator_ieee().into()) {
        return Err(BackupError::InvalidNetworkParameters);
    }
    validate_key(snapshot.network_key().key().expose_secret())?;
    for device in snapshot.devices() {
        if !valid_ieee(device.ieee_address().into()) {
            return Err(BackupError::Incompatible("invalid EZSP v8 device address"));
        }
        if let Some(key) = device.link_key() {
            validate_key(key.key().expose_secret())?;
        }
    }
    Ok(security)
}

fn unchanged<E>(error: E) -> Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    BackupError::restore_failed(RestoreFailureState::Unchanged, error).into()
}

fn valid_ieee(ieee: ember::Eui64) -> bool {
    valid_bytes(&ieee.into_array())
}

fn validate_key(key: &key::Data) -> Result<(), BackupError> {
    if !valid_bytes(key) {
        return Err(BackupError::Incompatible(INVALID_KEY));
    }
    Ok(())
}

fn valid_bytes(bytes: &[u8]) -> bool {
    bytes.iter().any(|byte| *byte != 0) && bytes.iter().any(|byte| *byte != u8::MAX)
}

async fn table_indices<T>(
    target: &mut T,
    id: config::Id,
) -> Result<impl Iterator<Item = u8> + use<T>, Error>
where
    T: Configuration + Send,
{
    let size = target.get_configuration_value(id).await?;
    if size > u16::from(u8::MAX) + 1 {
        return Err(BackupError::Incompatible(INVALID_TABLE_SIZE).into());
    }
    Ok((u8::MIN..=u8::MAX).take(usize::from(size)))
}

async fn read_link_keys<T>(target: &mut T) -> Result<Vec<key::Struct>, Error>
where
    T: Communicate,
{
    let mut keys = Vec::new();
    for index in table_indices(target, config::Id::KeyTableSize).await? {
        let entry: Result<Option<key::Struct>, _> = target
            .communicate(get_key_table_entry::Command::new(index))
            .await?
            .try_into();
        match entry {
            Ok(Some(key)) => keys.push(key),
            Ok(None) => (),
            Err(crate::Error::Status(crate::error::Status::Ember(Ok(
                ember::Status::IndexOutOfRange,
            )))) => break,
            Err(error) => return Err(error.into()),
        }
    }
    Ok(keys)
}
