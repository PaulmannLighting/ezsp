use crate::Error;

pub trait Resolvable {
    type Result;

    fn resolve(self) -> Result<Self::Result, Error>;
}
