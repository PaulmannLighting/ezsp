//! Parameters for the [`Zll::get_tokens`](crate::Zll::get_tokens) command.

use crate::ember::zll::{DataToken, SecurityToken};

crate::frame::parameters::frame!(0x00BC, {}, { data: DataToken, security: SecurityToken });

impl Response {
    /// Returns the token data.
    #[must_use]
    pub const fn data(&self) -> &DataToken {
        &self.data
    }

    /// Returns the token security.
    #[must_use]
    pub const fn security(&self) -> &SecurityToken {
        &self.security
    }
}
