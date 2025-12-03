use le_stream::{FromLeStream, ToLeStream};

use crate::ember::key::Data;
use crate::ember::zll::KeyIndex;

/// Describes the initial security features and requirements
/// that will be used when forming or joining ZLL networks.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct InitialSecurityState {
    bitmask: u32,
    key_index: u8,
    encryption_key: Data,
    preconfigured_key: Data,
}

impl InitialSecurityState {
    /// Create a new initial security state.
    #[must_use]
    pub fn new(
        bitmask: u32,
        key_index: KeyIndex,
        encryption_key: Data,
        preconfigured_key: Data,
    ) -> Self {
        Self {
            bitmask,
            key_index: key_index.into(),
            encryption_key,
            preconfigured_key,
        }
    }

    /// Return the bitmask.
    ///
    /// This is currently unused and reserved for future use.
    #[must_use]
    pub const fn bitmask(&self) -> u32 {
        self.bitmask
    }

    /// Return the key encryption algorithm advertised by the application.
    ///
    /// # Errors
    /// Returns the [`u8`] value of the key index if it is not a valid [`KeyIndex`].
    pub fn key_index(&self) -> Result<KeyIndex, u8> {
        KeyIndex::try_from(self.key_index)
    }

    /// Return the encryption key for use by algorithms that require it.
    #[must_use]
    pub const fn encryption_key(&self) -> &Data {
        &self.encryption_key
    }

    /// Return the pre-configured link key used during classical Zigbee commissioning.
    #[must_use]
    pub const fn preconfigured_key(&self) -> &Data {
        &self.preconfigured_key
    }
}
