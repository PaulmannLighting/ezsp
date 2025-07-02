use std::array::TryFromSliceError;

use crate::ezsp::value::{EmberVersion, Id};
use crate::{Configuration, Error};

/// Extensions trait for the `GetValue` command.
///
/// This trait provides an additional method to read certain values from the NCP,
pub trait GetValueExt {
    /// Reads a value from the NCP but passes an extra argument specific to the value being retrieved.
    fn get_ember_version(
        &mut self,
    ) -> impl Future<Output = Result<Result<EmberVersion, TryFromSliceError>, Error>> + Send;
}

impl<T> GetValueExt for T
where
    T: Configuration + Send,
{
    async fn get_ember_version(
        &mut self,
    ) -> Result<Result<EmberVersion, TryFromSliceError>, Error> {
        self.get_value(Id::VersionInfo)
            .await
            .map(EmberVersion::try_from)
    }
}
