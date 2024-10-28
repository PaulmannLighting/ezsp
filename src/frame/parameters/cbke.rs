//! Certificate-Based Key Exchange (CBKE) Frames

pub mod calculate_smacs;
pub mod calculate_smacs283k1;
pub mod clear_temporary_data_maybe_store_link_key;
pub mod clear_temporary_data_maybe_store_link_key283k1;
pub mod dsa_sign;
pub mod dsa_verify;
pub mod dsa_verify283k1;
pub mod generate_cbke_keys;
pub mod generate_cbke_keys283k1;
pub mod get_certificate;
pub mod get_certificate283k1;
pub mod handler;
pub mod save_preinstalled_cbke_data283k1;
pub mod set_preinstalled_cbke_data;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    CalculateSmacs(calculate_smacs::Response),
    CalculateSmacs283k1(calculate_smacs283k1::Response),
    ClearTemporaryDataMaybeStoreLinkKey(clear_temporary_data_maybe_store_link_key::Response),
    ClearTemporaryDataMaybeStoreLinkKey283k1(
        clear_temporary_data_maybe_store_link_key283k1::Response,
    ),
    DsaSign(dsa_sign::Response),
    DsaVerify(dsa_verify::Response),
    DsaVerify283k1(dsa_verify283k1::Response),
    GenerateCbkeKeys(generate_cbke_keys::Response),
    GenerateCbkeKeys283k1(generate_cbke_keys283k1::Response),
    GetCertificate(get_certificate::Response),
    SavePreinstalledCbkeData283k1(save_preinstalled_cbke_data283k1::Response),
    SetPreinstalledCbkeData(set_preinstalled_cbke_data::Response),
}
