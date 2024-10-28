use std::future::Future;

use crate::ember::{
    Certificate283k1Data, CertificateData, MessageDigest, PrivateKeyData, PublicKey283k1Data,
    PublicKeyData, Signature283k1Data, SignatureData,
};
use crate::frame::parameters::cbke::{
    calculate_smacs, calculate_smacs283k1, clear_temporary_data_maybe_store_link_key,
    clear_temporary_data_maybe_store_link_key283k1, dsa_sign, dsa_verify, dsa_verify283k1,
    generate_cbke_keys, generate_cbke_keys283k1, get_certificate, get_certificate283k1,
    save_preinstalled_cbke_data283k1, set_preinstalled_cbke_data,
};
use crate::types::ByteSizedVec;
use crate::{Error, Transport};

/// The `Cbke` trait provides an interface for the Certificate Based Key Exchange features.
pub trait Cbke {
    // TODO: Where is `ezspGenerateKeysRetrieveCert()` defined?
    /// Calculates the SMAC verification keys for both the initiator and responder roles of
    /// CBKE using the passed parameters and the stored public/private key pair previously
    /// generated with `ezspGenerateKeysRetrieveCert()`.
    /// It also stores the unverified link key data in temporary storage on the NCP until the key
    /// establishment is complete.
    fn calculate_smacs(
        &mut self,
        am_initiator: bool,
        partner_certificate: CertificateData,
        partner_ephemeral_public_key: PublicKeyData,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    // TODO: Where is `ezspGenerateKeysRetrieveCert283k1()` defined?
    /// Calculates the SMAC verification keys for both the initiator and responder roles of
    /// CBKE for the 283k1 ECC curve using the passed parameters and the stored public/private
    /// key pair previously generated with `ezspGenerateKeysRetrieveCert283k1()`.
    ///
    /// It also stores the unverified link key data in temporary storage on the NCP until the
    /// key establishment is complete.
    fn calculate_smacs283k1(
        &mut self,
        am_initiator: bool,
        partner_certificate: Certificate283k1Data,
        partner_ephemeral_public_key: PublicKey283k1Data,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Clears the temporary data associated with CBKE and the key establishment,
    /// most notably the ephemeral public/private key pair.
    ///
    /// If storeLinKey is true it moves the unverified link key stored in temporary storage
    /// into the link key table. Otherwise, it discards the key.
    fn clear_temporary_data_maybe_store_link_key(
        &mut self,
        store_link_key: bool,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Clears the temporary data associated with CBKE and the key establishment,
    /// most notably the ephemeral public/private  key pair.
    ///
    /// If storeLinKey is true it moves the unverified link key stored in temporary storage
    /// into the link key table. Otherwise, it discards the key.
    fn clear_temporary_data_maybe_store_link_key283k1(
        &mut self,
        store_link_key: bool,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// This functionality has been replaced by a single bit in the `EmberApsFrame`,
    /// `EMBER_APS_OPTION_DSA_SIGN`.
    ///
    /// Devices wishing to send signed messages should use that as it requires fewer function calls
    /// and message buffering.
    /// The dsaSignHandler response is still called when `EMBER_APS_OPTION_DSA_SIGN` is used.
    /// However, this function is still supported.
    ///
    /// This function begins the process of signing the passed message contained within the
    /// `messageContents` array.
    /// If no other ECC operation is going on, it will immediately return with
    /// [`Status::OperationInProgress`](crate::ember::Status::OperationInProgress) to indicate the
    /// start of ECC operation.
    /// It will delay a period of time to let APS retries take place, but then it will shut down the
    /// radio and consume the CPU processing until the signing is complete.
    /// This may take up to 1 second. The signed message will be returned in the dsaSignHandler
    /// response.
    ///
    /// Note that the last byte of the messageContents passed to this function has special
    /// significance.
    /// As the typical use case for DSA signing is to sign the ZCL payload of a `DRLC` Report Event
    /// Status message in SE 1.0,
    /// there is often both a signed portion (ZCL payload) and an unsigned portion (ZCL header).
    /// The last byte in the content of messageToSign is therefore used as a special indicator to
    /// signify how many bytes of leading data in the array should be excluded from consideration
    /// during the signing process.
    /// If the signature needs to cover the entire array (all bytes except last one), the caller
    /// should ensure that the last byte of messageContents is 0x00.
    /// When the signature operation is complete, this final byte will be replaced by the signature
    /// type indicator (0x01 for ECDSA signatures),  and the actual signature will be appended to
    /// the original contents after this byte.
    #[deprecated]
    fn dsa_sign(
        &mut self,
        message: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Verify that signature of the associated message digest was signed by the private key of the associated certificate.
    fn dsa_verify(
        &mut self,
        digest: MessageDigest,
        signer_certificate: CertificateData,
        received_sig: SignatureData,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Verify that signature of the associated message digest was signed by the private key of the associated certificate.
    fn dsa_verify283k1(
        &mut self,
        digest: MessageDigest,
        signer_certificate: Certificate283k1Data,
        received_sig: Signature283k1Data,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// This call starts the generation of the ECC Ephemeral Public/Private key pair.
    ///
    /// When complete it stores the private key.
    /// The results are returned via [`GenerateCbkeKeys`](crate::frame::parameters::cbke::handler::GenerateCbkeKeys).
    fn generate_cbke_keys(&mut self) -> impl Future<Output = Result<(), Error>> + Send;

    /// This call starts the generation of the ECC 283k1 curve Ephemeral Public/Private key pair.
    ///
    /// When complete it stores the private key.
    /// The results are returned via [`GenerateCbkeKeys283k1`](crate::frame::parameters::cbke::handler::GenerateCbkeKeys283k1).
    fn generate_cbke_keys283k1(&mut self) -> impl Future<Output = Result<(), Error>> + Send;

    /// Retrieves the certificate installed on the NCP.
    fn get_certificate(&mut self) -> impl Future<Output = Result<CertificateData, Error>> + Send;

    /// Retrieves the 283k certificate installed on the NCP.
    fn get_certificate283k1(
        &mut self,
    ) -> impl Future<Output = Result<Certificate283k1Data, Error>> + Send;

    /// Sets the device's 283k1 curve CA public key, local certificate,
    /// and static private key on the NCP associated with this node.
    fn save_preinstalled_cbke_data283k1(
        &mut self,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sets the device's CA public key, local certificate,
    /// and static private key on the NCP associated with this node.
    fn set_preinstalled_cbke_data(
        &mut self,
        ca_public: PublicKeyData,
        my_cert: CertificateData,
        my_key: PrivateKeyData,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl<T> Cbke for T
where
    T: Transport,
{
    async fn calculate_smacs(
        &mut self,
        am_initiator: bool,
        partner_certificate: CertificateData,
        partner_ephemeral_public_key: PublicKeyData,
    ) -> Result<(), Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, calculate_smacs::Response>(calculate_smacs::Command::new(
            am_initiator,
            partner_certificate,
            partner_ephemeral_public_key,
        ))
        .await?
        .try_into()
    }

    async fn calculate_smacs283k1(
        &mut self,
        am_initiator: bool,
        partner_certificate: Certificate283k1Data,
        partner_ephemeral_public_key: PublicKey283k1Data,
    ) -> Result<(), Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, calculate_smacs283k1::Response>(calculate_smacs283k1::Command::new(
            am_initiator,
            partner_certificate,
            partner_ephemeral_public_key,
        ))
        .await?
        .try_into()
    }

    async fn clear_temporary_data_maybe_store_link_key(
        &mut self,
        store_link_key: bool,
    ) -> Result<(), Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, clear_temporary_data_maybe_store_link_key::Response>(
            clear_temporary_data_maybe_store_link_key::Command::new(store_link_key),
        )
        .await?
        .try_into()
    }

    async fn clear_temporary_data_maybe_store_link_key283k1(
        &mut self,
        store_link_key: bool,
    ) -> Result<(), Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, clear_temporary_data_maybe_store_link_key283k1::Response>(
            clear_temporary_data_maybe_store_link_key283k1::Command::new(store_link_key),
        )
        .await?
        .try_into()
    }

    async fn dsa_sign(&mut self, message: ByteSizedVec<u8>) -> Result<(), Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, dsa_sign::Response>(dsa_sign::Command::new(message))
            .await
            .map(drop)
    }

    async fn dsa_verify(
        &mut self,
        digest: MessageDigest,
        signer_certificate: CertificateData,
        received_sig: SignatureData,
    ) -> Result<(), Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, dsa_verify::Response>(dsa_verify::Command::new(
            digest,
            signer_certificate,
            received_sig,
        ))
        .await?
        .try_into()
    }

    async fn dsa_verify283k1(
        &mut self,
        digest: MessageDigest,
        signer_certificate: Certificate283k1Data,
        received_sig: Signature283k1Data,
    ) -> Result<(), Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, dsa_verify283k1::Response>(dsa_verify283k1::Command::new(
            digest,
            signer_certificate,
            received_sig,
        ))
        .await?
        .try_into()
    }

    async fn generate_cbke_keys(&mut self) -> Result<(), Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, generate_cbke_keys::Response>(generate_cbke_keys::Command)
            .await?
            .try_into()
    }

    async fn generate_cbke_keys283k1(&mut self) -> Result<(), Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, generate_cbke_keys283k1::Response>(generate_cbke_keys283k1::Command)
            .await?
            .try_into()
    }

    async fn get_certificate(&mut self) -> Result<CertificateData, Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, get_certificate::Response>(get_certificate::Command)
            .await?
            .try_into()
    }

    async fn get_certificate283k1(&mut self) -> Result<Certificate283k1Data, Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, get_certificate283k1::Response>(get_certificate283k1::Command)
            .await?
            .try_into()
    }

    async fn save_preinstalled_cbke_data283k1(&mut self) -> Result<(), Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, save_preinstalled_cbke_data283k1::Response>(
            save_preinstalled_cbke_data283k1::Command,
        )
        .await?
        .try_into()
    }

    async fn set_preinstalled_cbke_data(
        &mut self,
        ca_public: PublicKeyData,
        my_cert: CertificateData,
        my_key: PrivateKeyData,
    ) -> Result<(), Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, set_preinstalled_cbke_data::Response>(
            set_preinstalled_cbke_data::Command::new(ca_public, my_cert, my_key),
        )
        .await?
        .try_into()
    }
}
