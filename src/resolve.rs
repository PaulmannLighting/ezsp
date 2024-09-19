use crate::{Error, Result};

pub trait Resolve {
    type Result;

    /// Resolve to a result type.
    ///
    /// # Errors
    /// Return [`Error`] in case of errors.
    fn resolve(self) -> Result<Self::Result>;
}

impl Resolve for std::result::Result<siliconlabs::Status, u32> {
    type Result = ();

    fn resolve(self) -> Result<Self::Result> {
        match self {
            Ok(status) => {
                if status == siliconlabs::Status::Ok {
                    Ok(())
                } else {
                    Err(Error::Siliconlabs(status))
                }
            }
            Err(error) => Err(Error::InvalidSiliconlabsStatus(error)),
        }
    }
}
