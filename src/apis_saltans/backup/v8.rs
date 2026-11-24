//! Catalog-free EZSP v8 network recovery using standard stack commands.
//!
//! Recovery forms a running network. Only the coordinator identity, active network credentials,
//! persistent partner keys and outgoing counter floors are applied. Child and address records
//! help describe the original network. Firmware rediscovers them as devices rejoin.

use apis_saltans_hw::{BackupError, Error, NetworkBackup, RestoreFailureState, RestoreOptions};

use super::{EMBER_BACKUP_NAMESPACE, EMBER_SECURITY_EXTENSION, EmberSecurityState};
use crate::ember::security::initial;
use crate::ember::{self, key};
use crate::ezsp::config;
use crate::frame::parameters::security::get_key_table_entry;
use crate::{Communicate, Configuration, Connection};

mod capture;
mod restore;
#[cfg(test)]
mod tests;

const VERSION: u8 = 8;
const STACK: &str = "emberznet-ezsp-v8-recovery";
const INVALID_KEY: &str = "invalid or unreadable EZSP v8 key";
const INVALID_TABLE_SIZE: &str = "EZSP v8 table exceeds its index range";
const BASE_SECURITY_FLAGS: initial::Bitmask = initial::Bitmask::HAVE_NETWORK_KEY
    .union(initial::Bitmask::HAVE_PRECONFIGURED_KEY)
    .union(initial::Bitmask::HAVE_TRUST_CENTER_EUI64)
    .union(initial::Bitmask::REQUIRE_ENCRYPTED_KEY)
    .union(initial::Bitmask::TRUST_CENTER_GLOBAL_LINK_KEY)
    .union(initial::Bitmask::NO_FRAME_COUNTER_RESET);

/// Network recovery data captured from an EZSP v8 coordinator.
///
/// Serialize the original snapshot using the optional `apis-saltans-hw` serde support.
/// Its keys are secret and redacted from debug output. Keep the snapshot for future recovery
/// with fresh counter advances. Its source identifier is specific to this workflow.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecoveryBackup(NetworkBackup);

impl RecoveryBackup {
    /// Validate a previously saved v8 recovery snapshot.
    ///
    /// # Errors
    /// Returns an error for a different backup kind, malformed security state, or invalid keys.
    pub fn from_snapshot(snapshot: NetworkBackup) -> Result<Self, BackupError> {
        snapshot.validate()?;
        if snapshot.source().stack() != STACK || !snapshot.additional_network_keys().is_empty() {
            return Err(BackupError::Incompatible(
                "expected an EZSP v8 recovery snapshot",
            ));
        }
        Self::security(&snapshot)?;
        let coordinator: ember::Eui64 = snapshot.network().coordinator_ieee().into();
        if !valid_ieee(coordinator) {
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
        Ok(Self(snapshot))
    }

    /// Access the original snapshot for inspection or serialization.
    #[must_use]
    pub const fn snapshot(&self) -> &NetworkBackup {
        &self.0
    }

    fn security(snapshot: &NetworkBackup) -> Result<EmberSecurityState, BackupError> {
        let [extension] = snapshot.extensions() else {
            return Err(BackupError::Incompatible(
                "expected one EZSP v8 security extension",
            ));
        };
        if extension.namespace() != EMBER_BACKUP_NAMESPACE
            || extension.name() != EMBER_SECURITY_EXTENSION
        {
            return Err(BackupError::Incompatible(
                "expected an Ember security extension",
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
        Ok(security)
    }
}

/// Verified results from forming the recovered v8 network.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct RecoveryReport {
    /// Active network key's observed outgoing counter.
    pub nwk_tx_counter: u32,
    /// Shared outgoing APS counter observed on the recovered Trust Center profile.
    pub aps_tx_counter: u32,
    /// Number of partner keys whose material and IEEE addresses were verified.
    pub restored_link_keys: usize,
    /// Network update ID requested by the snapshot.
    pub requested_nwk_update_id: u8,
    /// Network update ID actually reported by the NCP. v8 firmware may reset it on formation.
    pub observed_nwk_update_id: u8,
}

impl Connection {
    /// Capture recoverable state from a running EZSP v8 coordinator.
    ///
    /// The NCP supplies partner IEEE addresses, child membership and short address hints. Keep
    /// joining and network key changes paused during capture. Commands from other clones of this
    /// connection are serialized, while the stack can still process radio traffic.
    ///
    /// # Errors
    /// Returns an error for the wrong protocol, an unreadable key, a different network role,
    /// inconsistent state, or failed NCP commands.
    pub async fn capture_v8_recovery(&self) -> Result<RecoveryBackup, Error> {
        self.check_v8()?;
        capture::capture(&mut self.maintenance().await).await
    }

    /// Form an EZSP v8 network from an original recovery snapshot.
    ///
    /// Use an idle, empty target with the original coordinator IEEE address already effective,
    /// security level configured, and the source coordinator offline. Keep the transport running
    /// and drain its callbacks. Run with a Tokio time driver. Supply counter advances exceeding usage since capture, including
    /// previous recoveries, and select a transmit power supported by the target.
    ///
    /// On success the target is running. After a failure or cancellation once writes have begun,
    /// inspect the target before retrying. Formation may already have started the recovered
    /// network. The original snapshot remains available.
    ///
    /// # Errors
    /// Returns a restore error with the mutation boundary if validation, a command, or readback
    /// fails. Live readback verifies network identity, key material and the outgoing counters.
    pub async fn restore_v8_recovery(
        &self,
        backup: &RecoveryBackup,
        options: RestoreOptions,
        radio_tx_power: i8,
    ) -> Result<RecoveryReport, Error> {
        self.check_v8().map_err(unchanged)?;
        restore::restore(
            &mut self.maintenance().await,
            backup,
            options,
            radio_tx_power,
        )
        .await
    }

    const fn check_v8(&self) -> Result<(), BackupError> {
        if self.desired_version.get() != VERSION {
            return Err(BackupError::Incompatible("recovery requires EZSP v8"));
        }
        Ok(())
    }
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
