use crate::{ember, Error, Transport};
use std::future::Future;

pub trait CertificateBasedKeyExchange: Transport {
    fn calculate_smacs(
        &self,
        am_initiator: bool,
        partner_certificate: ember::CertificateData,
        partner_ephemeral_public_key: ember::PublicKeyData,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}
