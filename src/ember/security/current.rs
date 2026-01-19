//! Ember current security state.

use std::fmt::Display;

use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

use crate::ember::types::Eui64;

/// Ember current security bitmask.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
#[repr(transparent)]
pub struct Bitmask(u16);

bitflags! {
    impl Bitmask: u16 {
        /// This denotes that the device is running in a network with Zigbee Standard Security.
        const STANDARD_SECURITY_MODE = 0x0000;
        /// This denotes that the device is running in a network without a centralized Trust Center.
        const DISTRIBUTED_TRUST_CENTER_MODE = 0x0002;
        /// This denotes that the device has a Global Link Key.
        ///
        /// The Trust Center Link Key is the same across multiple nodes.
        const GLOBAL_LINK_KEY = 0x0004;
        /// This denotes that the node has a Trust Center Link Key.
        const HAVE_TRUST_CENTER_LINK_KEY = 0x0010;
        /// This denotes that the Trust Center is using a hashed Link Key.
        const TRUST_CENTER_USES_HASHED_LINK_KEY = 0x0084;
    }
}

impl From<Bitmask> for u16 {
    fn from(bitmask: Bitmask) -> Self {
        bitmask.0
    }
}

impl From<u16> for Bitmask {
    fn from(value: u16) -> Self {
        Self::from_bits_truncate(value)
    }
}

/// The security options and information currently used by the stack.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct State {
    bitmask: u16,
    trust_center_long_address: Eui64,
}

impl State {
    /// Create a new current security state.
    #[must_use]
    pub const fn new(bitmask: Bitmask, trust_center_long_address: Eui64) -> Self {
        Self {
            bitmask: bitmask.0,
            trust_center_long_address,
        }
    }

    /// Return the bitmask indicating the security options currently in use
    /// by a device joined in the network.
    ///
    /// # Errors
    /// Returns the [`u16`] value of the bitmask if it is not a valid [`Bitmask`].
    #[must_use]
    pub fn bitmask(&self) -> Bitmask {
        Bitmask::from(self.bitmask)
    }

    /// Return the IEEE Address of the Trust Center device.
    #[must_use]
    pub const fn trust_center_long_address(&self) -> Eui64 {
        self.trust_center_long_address
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Bitmask:")?;
        for (name, bitmask) in self.bitmask().iter_names() {
            writeln!(f, "\t{name}: {bitmask:#06X}")?;
        }

        writeln!(
            f,
            "Trust Center Long Address: {}",
            self.trust_center_long_address
        )
    }
}
