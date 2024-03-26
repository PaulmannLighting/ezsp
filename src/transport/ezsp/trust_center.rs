use crate::types::ByteSizedVec;
use crate::{ember, Error, Transport};
use std::future::Future;

pub trait TrustCenter: Transport {
    fn broadcast_next_network_key(
        &self,
        key: ember::key::Data,
    ) -> impl Future<Output = Result<(), Error>>;

    fn broadcast_network_key_switch(&self) -> impl Future<Output = Result<(), Error>> + Send;

    fn aes_mmo_hash(
        &self,
        context: ember::aes::MmoHashContext,
        finalize: bool,
        data: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<ember::aes::MmoHashContext, Error>> + Send;
}
