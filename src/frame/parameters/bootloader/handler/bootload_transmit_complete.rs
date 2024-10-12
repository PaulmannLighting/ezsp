use le_stream::derive::FromLeStream;

use crate::ember::Status;
use crate::frame::Parameter;
use crate::resolve::Resolve;
use crate::types::ByteSizedVec;
use crate::Error;

const ID: u16 = 0x0093;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    status: u8,
    message: ByteSizedVec<u8>,
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Handler {
    type Output = ByteSizedVec<u8>;

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|_| self.message)
    }
}
