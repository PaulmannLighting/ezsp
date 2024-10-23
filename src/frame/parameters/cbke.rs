//! Certificate-Based Key Exchange (CBKE) Frames

pub(crate) mod calculate_smacs;
pub(crate) mod calculate_smacs283k1;
pub(crate) mod clear_temporary_data_maybe_store_link_key;
pub(crate) mod clear_temporary_data_maybe_store_link_key283k1;
pub(crate) mod dsa_sign;
pub(crate) mod dsa_verify;
pub(crate) mod dsa_verify283k1;
pub(crate) mod generate_cbke_keys;
pub(crate) mod generate_cbke_keys283k1;
pub(crate) mod get_certificate;
pub(crate) mod get_certificate283k1;
pub mod handler;
pub(crate) mod save_preinstalled_cbke_data283k1;
pub(crate) mod set_preinstalled_cbke_data;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
    CalculateSmacs(calculate_smacs::Command),
    CalculateSmacs283k1(calculate_smacs283k1::Command),
    ClearTemporaryDataMaybeStoreLinkKey(clear_temporary_data_maybe_store_link_key::Command),
    ClearTemporaryDataMaybeStoreLinkKey283k1(
        clear_temporary_data_maybe_store_link_key283k1::Command,
    ),
    DsaSign(dsa_sign::Command),
    DsaVerify(dsa_verify::Command),
    DsaVerify283k1(dsa_verify283k1::Command),
    GenerateCbkeKeys(generate_cbke_keys::Command),
    GenerateCbkeKeys283k1(generate_cbke_keys283k1::Command),
    GetCertificate(get_certificate::Command),
    SavePreinstalledCbkeData283k1(save_preinstalled_cbke_data283k1::Command),
    SetPreinstalledCbkeData(set_preinstalled_cbke_data::Command),
}

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
    Handler(handler::Handler),
}
