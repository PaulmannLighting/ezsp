//! Versioned, required Ember Trust Center security extension.

use apis_saltans_hw::{BackupError, BackupExtension, NetworkKey};
use le_stream::{FromLeStream, ToLeStream};

use crate::ember::security::initial::Bitmask;

/// Source stack identifier accepted by the Ember restore planner.
pub const EMBER_BACKUP_STACK: &str = "emberznet";
/// Namespace of the required Ember security extension.
pub const EMBER_BACKUP_NAMESPACE: &str = "org.silabs.emberznet";
/// Name of the required Ember security extension.
pub const EMBER_SECURITY_EXTENSION: &str = "security";
/// Current Ember security extension schema version.
pub const EMBER_SECURITY_VERSION: u32 = 1;

const KEY_LENGTH: usize = 16;
const PAYLOAD_LENGTH: usize = size_of::<u16>() + KEY_LENGTH + size_of::<u32>();

/// Trust Center key or hash seed, initial-security flags, and global outgoing APS counter.
///
/// The version-one payload is exactly 22 bytes: little-endian flags, 16 secret
/// key/seed bytes, and a little-endian APS counter. It is a required part of a
/// complete snapshot, not a replacement for child records, replay state, policies,
/// or token state. Debug output redacts the key; memory is not zeroized.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmberSecurityState {
    flags: Bitmask,
    trust_center_key: NetworkKey,
    aps_tx_counter: u32,
}

impl EmberSecurityState {
    /// Construct the security extension, preserving global versus hashed-key mode.
    ///
    /// # Errors
    /// Rejects distributed security, absent key flags, and install-code-derived
    /// initialization, which cannot be reproduced from this key/seed alone.
    pub fn new(
        flags: Bitmask,
        trust_center_key: NetworkKey,
        aps_tx_counter: u32,
    ) -> Result<Self, BackupError> {
        let required = Bitmask::HAVE_NETWORK_KEY | Bitmask::HAVE_PRECONFIGURED_KEY;
        if Bitmask::from_bits(flags.bits()).is_none()
            || !flags.contains(required)
            || !flags.contains(Bitmask::TRUST_CENTER_GLOBAL_LINK_KEY)
            || flags.intersects(
                Bitmask::DISTRIBUTED_TRUST_CENTER_MODE
                    | Bitmask::GET_PRECONFIGURED_KEY_FROM_INSTALL_CODE,
            )
        {
            return Err(BackupError::Incompatible(
                "unsupported Ember initial security flags",
            ));
        }
        Ok(Self {
            flags,
            trust_center_key,
            aps_tx_counter,
        })
    }

    /// Initial security flags, including whether the key is a hashed-key seed.
    #[must_use]
    pub const fn flags(&self) -> Bitmask {
        self.flags
    }

    /// Secret Trust Center key or seed with redacted debugging.
    #[must_use]
    pub const fn trust_center_key(&self) -> &NetworkKey {
        &self.trust_center_key
    }

    /// Global outgoing APS security counter, independent of per-device incoming counters.
    #[must_use]
    pub const fn aps_tx_counter(&self) -> u32 {
        self.aps_tx_counter
    }

    /// Encode the required namespaced extension, including secret key material.
    ///
    /// # Errors
    /// Forwards extension identifier validation errors.
    pub fn to_extension(&self) -> Result<BackupExtension, BackupError> {
        let payload = self
            .flags
            .bits()
            .to_le_stream()
            .chain(self.trust_center_key.expose_secret().iter().copied())
            .chain(self.aps_tx_counter.to_le_stream())
            .collect();
        BackupExtension::new(
            EMBER_BACKUP_NAMESPACE.into(),
            EMBER_SECURITY_EXTENSION.into(),
            EMBER_SECURITY_VERSION,
            payload,
        )
    }

    /// Decode a required extension without accepting unknown versions, flags, or trailing bytes.
    ///
    /// # Errors
    /// Rejects incompatible identifiers, versions, sizes, and security flags.
    pub fn from_extension(extension: &BackupExtension) -> Result<Self, BackupError> {
        if extension.namespace() != EMBER_BACKUP_NAMESPACE
            || extension.name() != EMBER_SECURITY_EXTENSION
            || extension.version() != EMBER_SECURITY_VERSION
            || extension.payload().len() != PAYLOAD_LENGTH
        {
            return Err(BackupError::Incompatible(
                "unknown or malformed Ember security extension",
            ));
        }
        let mut bytes = extension.payload().iter().copied();
        let flags = u16::from_le_stream(&mut bytes)
            .and_then(Bitmask::from_bits)
            .ok_or(BackupError::Incompatible("unknown Ember security flags"))?;
        let key = <[u8; KEY_LENGTH]>::from_le_stream(&mut bytes)
            .ok_or(BackupError::Incompatible("missing Trust Center key"))?;
        let counter = u32::from_le_stream(&mut bytes)
            .ok_or(BackupError::Incompatible("missing APS counter"))?;
        Self::new(flags, NetworkKey::new(key), counter)
    }

    pub(super) fn advance(&mut self, amount: u32) -> Result<(), BackupError> {
        self.aps_tx_counter = self
            .aps_tx_counter
            .checked_add(amount)
            .filter(|value| *value < u32::MAX)
            .ok_or(BackupError::CounterExhausted)?;
        Ok(())
    }
}
