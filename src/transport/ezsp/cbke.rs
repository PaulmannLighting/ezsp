use crate::ember::{Certificate283k1Data, PublicKey283k1Data};
use crate::{ember, Error, Transport};
use std::future::Future;

pub trait CertificateBasedKeyExchange: Transport {
    /// Calculates the SMAC verification keys for both the initiator and responder roles of
    /// CBKE using the passed parameters and the stored public/private key pair previously
    /// generated with ezspGenerateKeysRetrieveCert().
    /// It also stores the unverified link key data in temporary storage on the NCP until the key
    /// establishment is complete.
    fn calculate_smacs(
        &self,
        am_initiator: bool,
        partner_certificate: ember::CertificateData,
        partner_ephemeral_public_key: ember::PublicKeyData,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Calculates the SMAC verification keys for both the initiator and responder roles of
    /// CBKE for the 283k1 ECC curve using the passed parameters and the stored public/private
    /// key pair previously generated with ezspGenerateKeysRetrieveCert283k1().
    /// It also stores the unverified link key data in temporary storage on the NCP until the
    /// key establishment is complete.
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
