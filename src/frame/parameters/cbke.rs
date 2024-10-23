//! Certificate-Based Key Exchange (CBKE) Frames

use le_stream::FromLeStream;

use crate::error::Decode;
use crate::frame::parsable::Parsable;
use crate::frame::Parameter;

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
            <calculate_smacs283k1::Response as Parameter>::ID => Ok(Self::CalculateSmacs283k1(
                calculate_smacs283k1::Response::from_le_stream_exact(stream)?,
            )),
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
            <dsa_verify::Response as Parameter>::ID => Ok(Self::DsaVerify(
                dsa_verify::Response::from_le_stream_exact(stream)?,
            )),
            <dsa_verify283k1::Response as Parameter>::ID => Ok(Self::DsaVerify283k1(
                dsa_verify283k1::Response::from_le_stream_exact(stream)?,
            )),
            <generate_cbke_keys::Response as Parameter>::ID => Ok(Self::GenerateCbkeKeys(
                generate_cbke_keys::Response::from_le_stream_exact(stream)?,
            )),
            <generate_cbke_keys283k1::Response as Parameter>::ID => {
                Ok(Self::GenerateCbkeKeys283k1(
                    generate_cbke_keys283k1::Response::from_le_stream_exact(stream)?,
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
            <handler::CalculateSmacs as Parameter>::ID => {
                Ok(Self::Handler(handler::Handler::CalculateSmacs(
                    handler::CalculateSmacs::from_le_stream_exact(stream)?,
                )))
            }
            <handler::CalculateSmacs283k1 as Parameter>::ID => {
                Ok(Self::Handler(handler::Handler::CalculateSmacs283k1(
                    handler::CalculateSmacs283k1::from_le_stream_exact(stream)?,
                )))
            }
            <handler::DsaSign as Parameter>::ID => Ok(Self::Handler(handler::Handler::DsaSign(
                handler::DsaSign::from_le_stream_exact(stream)?,
            ))),
            <handler::DsaVerify as Parameter>::ID => Ok(Self::Handler(
                handler::Handler::DsaVerify(handler::DsaVerify::from_le_stream_exact(stream)?),
            )),
            <handler::GenerateCbkeKeys as Parameter>::ID => {
                Ok(Self::Handler(handler::Handler::GenerateCbkeKeys283k1(
                    handler::GenerateCbkeKeys283k1::from_le_stream_exact(stream)?,
                )))
            }
            <handler::GenerateCbkeKeys283k1 as Parameter>::ID => {
                Ok(Self::Handler(handler::Handler::GenerateCbkeKeys283k1(
                    handler::GenerateCbkeKeys283k1::from_le_stream_exact(stream)?,
                )))
            }
            _ => Err(Decode::InvalidFrameId(id)),
        }
    }
}
