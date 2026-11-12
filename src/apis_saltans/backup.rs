//! Complete-backup trait dispatch and fail-closed firmware qualification.
//!
//! An explicit [`OfflineTokenProfile`] enables complete token-image capture and
//! offline storage restoration. No real firmware profile is bundled or certified.
//! Without a configured profile, operations retain the original qualification
//! errors. Successful storage readback does not prove persistence across reset;
//! the application owns reset, reconnection, and later network activation.

use apis_saltans_hw::{
    BackupError, Error, NetworkBackup, NetworkDump, NetworkRestore, RestoreFailureState,
    RestoreOptions, RestoreReport,
};

pub use self::plan::RestorePlan;
pub use self::profile::{
    DeviceTokenLayout, LinkKeyTokenLayout, NetworkTokenLayout, OfflineTokenProfile,
    SecurityTokenLayout, TokenField,
};
pub use self::security::{
    EMBER_BACKUP_NAMESPACE, EMBER_BACKUP_STACK, EMBER_SECURITY_EXTENSION, EMBER_SECURITY_VERSION,
    EmberSecurityState,
};
use crate::api::backup_inspection::inspect;
use crate::ember::network::Status;
use crate::ezsp::config::Id;
use crate::ezsp::value;
use crate::{
    BACKUP_INSPECTION_PROTOCOL_VERSION, Configuration, GetValueExt, Identity, Ncp, Networking,
    TokenInterface, Utilities,
};

mod image;
mod offline;
mod plan;
mod profile;
mod security;
#[cfg(test)]
mod simulation;
#[cfg(test)]
mod tests;

const CLASSIC_KEY_STORAGE: u8 = 0;
const ERASED_EUI64: [u8; 8] = [u8::MAX; 8];
const SNAPSHOT_BLOCKER: &str = "firmware has no qualified consistent complete snapshot mechanism";
const RESTORE_BLOCKER: &str =
    "persistent replay-state restoration and offline reset/readback are not qualified";

impl Ncp {
    /// Select an explicit offline token-image firmware contract.
    ///
    /// No real profile is bundled or hardware-certified. This opt-in asserts the
    /// documented persistence, completeness, portability, and quiescence assumptions
    /// of `OfflineTokenProfile`. It never stops or starts the network automatically.
    #[must_use]
    pub fn with_offline_token_backup(mut self, profile: OfflineTokenProfile) -> Self {
        self.backup_profile = Some(profile);
        self
    }
}

impl NetworkDump for Ncp {
    async fn dump_network(&mut self) -> Result<NetworkBackup, Error> {
        let protocol = self.connection.desired_version.get();
        if let Some(profile) = &self.backup_profile {
            check_protocol(protocol)?;
            let mut session = self.connection.maintenance().await;
            return offline::capture(&mut session, profile).await;
        }
        dump(&mut self.connection, protocol).await
    }
}

impl NetworkRestore for Ncp {
    async fn validate_restore(
        &mut self,
        backup: &NetworkBackup,
        options: RestoreOptions,
    ) -> Result<(), Error> {
        let plan = RestorePlan::from_prepared(backup, options)?;
        let protocol = self.connection.desired_version.get();
        if let Some(profile) = &self.backup_profile {
            check_protocol(protocol)?;
            let mut session = self.connection.maintenance().await;
            return offline::validate(&mut session, profile, &plan, options)
                .await
                .map(drop);
        }
        validate_target(&mut self.connection, protocol, &plan).await
    }

    async fn restore_network(
        &mut self,
        backup: NetworkBackup,
        options: RestoreOptions,
    ) -> Result<RestoreReport, Error> {
        if let Some(profile) = &self.backup_profile {
            let plan = RestorePlan::from_prepared(&backup, options).map_err(|error| {
                BackupError::restore_failed(RestoreFailureState::Unchanged, error)
            })?;
            check_protocol(self.connection.desired_version.get()).map_err(|error| {
                BackupError::restore_failed(RestoreFailureState::Unchanged, error)
            })?;
            let mut session = self.connection.maintenance().await;
            return offline::restore(&mut session, profile, &plan, options).await;
        }
        // Revalidate even for direct trait calls or if another connection changed
        // the target after validation. No write is attempted on an unqualified profile.
        self.validate_restore(&backup, options)
            .await
            .map_err(|error| {
                Error::from(BackupError::restore_failed(
                    RestoreFailureState::Unchanged,
                    error,
                ))
            })?;
        Err(BackupError::restore_failed(
            RestoreFailureState::Unchanged,
            BackupError::Incompatible(RESTORE_BLOCKER),
        )
        .into())
    }
}

async fn dump<T>(target: &mut T, protocol: u8) -> Result<NetworkBackup, Error>
where
    T: Configuration + Networking + TokenInterface + Send,
{
    check_protocol(protocol)?;
    let inspection = inspect(target, protocol).await?;
    if inspection.network_state != Status::JoinedNetwork {
        return Err(BackupError::InvalidState.into());
    }
    if inspection.key_storage_version != CLASSIC_KEY_STORAGE {
        return Err(BackupError::Incompatible("key storage export/import is not qualified").into());
    }
    Err(BackupError::Incompatible(SNAPSHOT_BLOCKER).into())
}

async fn validate_target<T>(target: &mut T, protocol: u8, plan: &RestorePlan) -> Result<(), Error>
where
    T: Configuration + Networking + Utilities + Send,
{
    check_protocol(protocol)?;
    if target.network_state().await? != Status::NoNetwork {
        return Err(BackupError::InvalidState.into());
    }
    let storage = target.get_value(value::Id::KeyStorageVersion).await?;
    if storage.as_slice() != [CLASSIC_KEY_STORAGE] {
        return Err(BackupError::Incompatible("key storage export/import is not qualified").into());
    }
    let version = target
        .get_ember_version()
        .await?
        .map_err(|_| BackupError::Incompatible("malformed Ember firmware version"))?;
    let backup = plan.backup();
    if backup.source().firmware() != version.to_string() {
        return Err(BackupError::Incompatible("source and target firmware differ").into());
    }
    let address = target.get_eui64().await?;
    if address != backup.network().coordinator_ieee().into() {
        let token = target.get_custom_eui64().await?;
        if token.into_array() != ERASED_EUI64 && token != backup.network().coordinator_ieee().into()
        {
            return Err(
                BackupError::Incompatible("custom EUI64 token cannot be overwritten").into(),
            );
        }
    }
    let keys = target.get_configuration_value(Id::KeyTableSize).await?;
    let children = target
        .get_configuration_value(Id::MaxEndDeviceChildren)
        .await?;
    if backup
        .devices()
        .iter()
        .filter(|device| device.link_key().is_some())
        .count()
        > usize::from(keys)
        || backup
            .devices()
            .iter()
            .filter(|device| device.is_child())
            .count()
            > usize::from(children)
    {
        return Err(BackupError::Incompatible("target key or child table is too small").into());
    }
    Err(BackupError::Incompatible(RESTORE_BLOCKER).into())
}

const fn check_protocol(protocol: u8) -> Result<(), BackupError> {
    if protocol != BACKUP_INSPECTION_PROTOCOL_VERSION {
        return Err(BackupError::Incompatible(
            "complete backup layout requires EZSP v13",
        ));
    }
    Ok(())
}
