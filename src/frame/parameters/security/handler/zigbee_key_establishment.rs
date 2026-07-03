use crate::ember::Eui64;
use crate::ember::key::{Status, TrustCenter};

crate::frame::parameters::handler!(
    0x009B,
    { partner: Eui64, status: u8 },
    impl {
        impl Handler {
            /// This is the IEEE address of the partner that the device successfully established a key with.
            ///
            /// # Errors
            ///
            /// Returns an error if the key establishment was not successful, containing either the status
            /// indicating what was established or why the key establishment failed.
            pub fn partner(&self) -> Result<Eui64, Result<Status, u8>> {
                match self.status() {
                    Ok(
                        Status::AppLinkKeyEstablished
                        | Status::TrustCenterLinkKeyEstablished
                        | Status::TrustCenter(TrustCenter::RespondedToKeyRequest),
                    ) => Ok(self.partner),
                    Ok(other) => Err(Ok(other)),
                    Err(invalid) => Err(Err(invalid)),
                }
            }

            /// This is the status indicating what was established or why the key establishment failed.
            ///
            /// # Errors
            ///
            /// Returns an error if the status is invalid.
            pub fn status(&self) -> Result<Status, u8> {
                Status::try_from(self.status)
            }
        }
    }
);
