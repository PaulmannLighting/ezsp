use crate::ember::{Certificate283k1Data, PublicKey283k1Data};
use crate::{ember, Error, Transport};
use std::future::Future;

pub trait CertificateBasedKeyExchange: Transport {
    fn calculate_smacs(
        &self,
        am_initiator: bool,
        partner_certificate: ember::CertificateData,
        partner_ephemeral_public_key: ember::PublicKeyData,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn calculate_smacs283k1(
        &self,
        am_initiator: bool,
        partner_certificate: Certificate283k1Data,
        partner_ephemeral_public_key: PublicKey283k1Data,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}
