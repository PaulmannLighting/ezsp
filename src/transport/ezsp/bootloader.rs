use crate::{Error, Transport};
use std::future::Future;

pub trait Bootloader: Transport {
    /// Perform AES encryption on `plaintext` using `key`.
    fn aes_encrypt(
        &self,
        plaintext: [u8; 16],
        key: [u8; 16],
    ) -> impl Future<Output = Result<[u8; 16], Error>> + Send;
}
