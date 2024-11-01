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

/// CBKE response parameters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    /// Response parameters to the `calculate_smacs` command.
    CalculateSmacs(calculate_smacs::Response),
    /// Response parameters to the `calculate_smacs283k1` command.
    CalculateSmacs283k1(calculate_smacs283k1::Response),
    /// Response parameters to the `clear_temporary_data_maybe_store_link_key` command.
    ClearTemporaryDataMaybeStoreLinkKey(clear_temporary_data_maybe_store_link_key::Response),
    /// Response parameters to the `clear_temporary_data_maybe_store_link_key283k1` command.
    ClearTemporaryDataMaybeStoreLinkKey283k1(
        clear_temporary_data_maybe_store_link_key283k1::Response,
    ),
    /// Response parameters to the `dsa_sign` command.
    DsaSign(dsa_sign::Response),
    /// Response parameters to the `dsa_verify` command.
    DsaVerify(dsa_verify::Response),
    /// Response parameters to the `dsa_verify283k1` command.
    DsaVerify283k1(dsa_verify283k1::Response),
    /// Response parameters to the `generate_cbke_keys` command.
    GenerateCbkeKeys(generate_cbke_keys::Response),
    /// Response parameters to the `generate_cbke_keys283k1` command.
    GenerateCbkeKeys283k1(generate_cbke_keys283k1::Response),
    /// Response parameters to the `get_certificate` command.
    GetCertificate(get_certificate::Response),
    /// Response parameters to the `get_certificate283k1` command.
    GetCertificate283k1(get_certificate283k1::Response),
    /// Response parameters to the `save_preinstalled_cbke_data283k1` command.
    SavePreinstalledCbkeData283k1(save_preinstalled_cbke_data283k1::Response),
    /// Response parameters to the `set_preinstalled_cbke_data` command.
    SetPreinstalledCbkeData(set_preinstalled_cbke_data::Response),
}
