//! Parameters for the [`Utilities::echo`](crate::Utilities::echo) command.

use crate::types::ByteSizedVec;

crate::frame::parameters::frame!(0x0081, { data: ByteSizedVec<u8> }, { echo: ByteSizedVec<u8> });

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub const fn new(data: ByteSizedVec<u8>) -> Self {
        Self { data }
    }
}

impl Response {
    /// Returns the echoed data.
    #[must_use]
    pub fn echo(self) -> ByteSizedVec<u8> {
        self.echo
    }
}
