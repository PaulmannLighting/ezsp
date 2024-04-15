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

    /// Clears the temporary data associated with CBKE and the key establishment,
    /// most notably the ephemeral public/private key pair.
    /// If storeLinKey is true it moves the unverified link key stored in temporary storage
    /// into the link key table. Otherwise it discards the key.
    fn clear_temporary_data_maybe_store_link_key(
        &self,
        store_link_key: bool,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}
