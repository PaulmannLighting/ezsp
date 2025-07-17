//! Ember node type.

use core::fmt::Display;

use num_derive::FromPrimitive;

/// Ember node type.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Type {
    /// Device is not joined.
    UnknownDevice = 0x00,
    /// Will relay messages and can act as a parent to other nodes.
    Coordinator = 0x01,
    /// Will relay messages and can act as a parent to other nodes.
    Router = 0x02,
    /// Communicates only with its parent and will not relay messages.
    EndDevice = 0x03,
    /// An end device whose radio can be turned off to save power.
    ///
    /// The application must poll to receive messages.
    SleepyEndDevice = 0x04,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownDevice => write!(f, "Unknown Device"),
            Self::Coordinator => write!(f, "Coordinator"),
            Self::Router => write!(f, "Router"),
            Self::EndDevice => write!(f, "End Device"),
            Self::SleepyEndDevice => write!(f, "Sleepy End Device"),
        }
    }
}

impl From<Type> for u8 {
    fn from(typ: Type) -> Self {
        typ as Self
    }
}
