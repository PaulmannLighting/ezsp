use crate::error::Invalid;
use crate::Error;

/// Resolve types into a result of a certain output type.
pub trait Resolve {
    /// Output type.
    type Output;

    /// Resolve to a result type.
    ///
    /// Returns `Ok(Self::Output)` if the resolution is successful.
    ///
    /// # Errors
    /// Return [`Error`] in case of errors.
    fn resolve(self) -> Result<Self::Output, Error>;
}

impl Resolve for Result<siliconlabs::Status, u32> {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        match self {
            Ok(status) => {
                if status == siliconlabs::Status::Ok {
                    Ok(())
                } else {
                    Err(Error::Siliconlabs(status))
                }
            }
            Err(error) => Err(Error::Invalid(Invalid::Siliconlabs(error))),
        }
    }
}
