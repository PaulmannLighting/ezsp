use crate::{Error, Transport};
use std::future::Future;

pub trait Bootloader: Transport {
    fn aes_encrypt(
        &mut self,
        plaintext: [u8; 16],
        key: [u8; 16],
    ) -> impl Future<Output = Result<[u8; 16], Error>> + Send;
}
