//! Constants used in the Ember library.

use core::time::Duration;

/// The `EMBER_NULL_NODE_ID` constant.
pub const NULL_NODE_ID: u16 = 0xFFFF;

/// A placeholder giving the number of Ember counter types.
///
/// Contrary to the documentation (`40`), the actual number of counter types is `41`.
pub const COUNTER_TYPE_COUNT: usize = 41;

/// The maximum number of children that an end device can have.
pub const MAX_END_DEVICE_CHILDREN: usize = 32;

/// The default amount of time that the MAC will hold a
/// message for indirect transmission to a child.
pub const INDIRECT_TRANSMISSION_TIMEOUT: Duration = Duration::from_millis(3_000);

/// The maximum amount of time that the MAC  will hold a
/// message for indirect transmission to a child.
pub const MAX_INDIRECT_TRANSMISSION_TIMEOUT: Duration = Duration::from_millis(30_000);
