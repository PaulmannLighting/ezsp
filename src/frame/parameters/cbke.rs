//! Certificate-Based Key Exchange (CBKE) Frames

use le_stream::FromLeStream;

use crate::error::Decode;
use crate::frame::parsable::Parsable;
use crate::frame::Parameter;

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
    /// Calculate Secure Message Authentication Codes (SMACs).
    CalculateSmacsHandler(handler::CalculateSmacs),
    CalculateSmacs283k1(calculate_smacs283k1::Response),
    /// Calculate Secure Message Authentication Codes (SMACs) for 283k1.
    CalculateSmacs283k1Handler(handler::CalculateSmacs283k1),
    ClearTemporaryDataMaybeStoreLinkKey(clear_temporary_data_maybe_store_link_key::Response),
    ClearTemporaryDataMaybeStoreLinkKey283k1(
        clear_temporary_data_maybe_store_link_key283k1::Response,
    ),
    DsaSign(dsa_sign::Response),
    /// Digital Signature Algorithm (DSA) sign.
    DsaSignHandler(handler::DsaSign),
    DsaVerify(dsa_verify::Response),
    /// Digital Signature Algorithm (DSA) verify.
    DsaVerifyHandler(handler::DsaVerify),
    DsaVerify283k1(dsa_verify283k1::Response),
    GenerateCbkeKeys(generate_cbke_keys::Response),
    /// Generate CBKE keys.
    GenerateCbkeKeysHandler(handler::GenerateCbkeKeys),
    GenerateCbkeKeys283k1(generate_cbke_keys283k1::Response),
    /// Generate CBKE keys for 283k1.
    GenerateCbkeKeys283k1Handler(handler::GenerateCbkeKeys283k1),
    GetCertificate(get_certificate::Response),
    SavePreinstalledCbkeData283k1(save_preinstalled_cbke_data283k1::Response),
    SetPreinstalledCbkeData(set_preinstalled_cbke_data::Response),
    Handler(handler::Handler),
}

impl Parsable for Response {
    fn parse_from_le_stream<T>(id: u16, stream: T) -> Result<Self, Decode>
    where
        T: Iterator<Item = u8>,
    {
        match id {
            <calculate_smacs::Response as Parameter>::ID => Ok(Self::CalculateSmacs(
                calculate_smacs::Response::from_le_stream_exact(stream)?,
            )),
            <handler::CalculateSmacs as Parameter>::ID => Ok(Self::CalculateSmacsHandler(
                handler::CalculateSmacs::from_le_stream_exact(stream)?,
            )),
            <calculate_smacs283k1::Response as Parameter>::ID => Ok(Self::CalculateSmacs283k1(
                calculate_smacs283k1::Response::from_le_stream_exact(stream)?,
            )),

            <handler::CalculateSmacs283k1 as Parameter>::ID => {
                Ok(Self::CalculateSmacs283k1Handler(
                    handler::CalculateSmacs283k1::from_le_stream_exact(stream)?,
                ))
            }
            <clear_temporary_data_maybe_store_link_key::Response as Parameter>::ID => {
                Ok(Self::ClearTemporaryDataMaybeStoreLinkKey(
                    clear_temporary_data_maybe_store_link_key::Response::from_le_stream_exact(
                        stream,
                    )?,
                ))
            }
            <clear_temporary_data_maybe_store_link_key283k1::Response as Parameter>::ID => {
                Ok(Self::ClearTemporaryDataMaybeStoreLinkKey283k1(
                    clear_temporary_data_maybe_store_link_key283k1::Response::from_le_stream_exact(
                        stream,
                    )?,
                ))
            }
            <dsa_sign::Response as Parameter>::ID => Ok(Self::DsaSign(
                dsa_sign::Response::from_le_stream_exact(stream)?,
            )),
            <handler::DsaSign as Parameter>::ID => Ok(Self::DsaSignHandler(
                handler::DsaSign::from_le_stream_exact(stream)?,
            )),
            <dsa_verify::Response as Parameter>::ID => Ok(Self::DsaVerify(
                dsa_verify::Response::from_le_stream_exact(stream)?,
            )),
            <handler::DsaVerify as Parameter>::ID => Ok(Self::DsaVerifyHandler(
                handler::DsaVerify::from_le_stream_exact(stream)?,
            )),
            <dsa_verify283k1::Response as Parameter>::ID => Ok(Self::DsaVerify283k1(
                dsa_verify283k1::Response::from_le_stream_exact(stream)?,
            )),
            <generate_cbke_keys::Response as Parameter>::ID => Ok(Self::GenerateCbkeKeys(
                generate_cbke_keys::Response::from_le_stream_exact(stream)?,
            )),
            <handler::GenerateCbkeKeys as Parameter>::ID => Ok(Self::GenerateCbkeKeysHandler(
                handler::GenerateCbkeKeys::from_le_stream_exact(stream)?,
            )),
            <generate_cbke_keys283k1::Response as Parameter>::ID => {
                Ok(Self::GenerateCbkeKeys283k1(
                    generate_cbke_keys283k1::Response::from_le_stream_exact(stream)?,
                ))
            }
            <handler::GenerateCbkeKeys283k1 as Parameter>::ID => {
                Ok(Self::GenerateCbkeKeys283k1Handler(
                    handler::GenerateCbkeKeys283k1::from_le_stream_exact(stream)?,
                ))
            }
            <get_certificate::Response as Parameter>::ID => Ok(Self::GetCertificate(
                get_certificate::Response::from_le_stream_exact(stream)?,
            )),
            <save_preinstalled_cbke_data283k1::Response as Parameter>::ID => {
                Ok(Self::SavePreinstalledCbkeData283k1(
                    save_preinstalled_cbke_data283k1::Response::from_le_stream_exact(stream)?,
                ))
            }
            <set_preinstalled_cbke_data::Response as Parameter>::ID => {
                Ok(Self::SetPreinstalledCbkeData(
                    set_preinstalled_cbke_data::Response::from_le_stream_exact(stream)?,
                ))
            }
            _ => Err(Decode::InvalidFrameId(id)),
        }
    }
}
