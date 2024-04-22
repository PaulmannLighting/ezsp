use crate::ember::{
    Certificate283k1Data, CertificateData, MessageDigest, PublicKey283k1Data, SignatureData,
};
use crate::types::ByteSizedVec;
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

    /// Clears the temporary data associated with CBKE and the key establishment,
    /// most notably the ephemeral public/private  key pair.
    /// If storeLinKey is true it moves the unverified link key stored in temporary storage
    /// into the link key table.
    /// Otherwise it discards the key.
    fn clear_temporary_data_maybe_store_link_key283k1(
        &self,
        store_link_key: bool,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// This functionality has been replaced by a single bit in the EmberApsFrame, `EMBER_APS_OPTION_DSA_SIGN`.
    ///
    /// Devices wishing to send signed messages should use that as it requires fewer function calls and message buffering.
    /// The dsaSignHandler response is still called when `EMBER_APS_OPTION_DSA_SIGN` is used.
    /// However, this function is still supported.
    ///
    /// This function begins the process of signing the passed message contained within the messageContents array.
    /// If no other ECC operation is going on, it will immediately return with
    /// [`Status::OperationInProgress`](ember::Status::OperationInProgress) to indicate the start of ECC operation.
    /// It will delay a period of time to let APS retries take place,
    /// but then it will shut down the radio and consume the CPU processing until the signing is complete.
    /// This may take up to 1 second. The signed message will be returned in the dsaSignHandler response.
    ///
    /// Note that the last byte of the messageContents passed to this function has special significance.
    /// As the typical use case for DSA signing is to sign the ZCL payload of a DRLC Report Event Status message in SE 1.0,
    /// there is often both a signed portion (ZCL payload) and an unsigned portion (ZCL header).
    /// The last byte in the content of messageToSign is therefore used as a special indicator to signify
    /// how many bytes of leading data in the array should be excluded from consideration during the signing process.
    /// If the signature needs to cover the entire array (all bytes except last one), the caller should ensure that the last byte of messageContents is 0x00.
    /// When the signature operation is complete, this final byte will be replaced by the signature type indicator (0x01 for ECDSA signatures),
    /// and the actual signature will be appended to the original contents after this byte.
    #[deprecated]
    fn dsa_sign(&self, message: ByteSizedVec<u8>)
        -> impl Future<Output = Result<(), Error>> + Send;

    /// Verify that signature of the associated message digest was signed by the private key of the associated certificate.
    fn dsa_verify(
        &self,
        digest: MessageDigest,
        signer_certificate: CertificateData,
        received_sig: SignatureData,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}
