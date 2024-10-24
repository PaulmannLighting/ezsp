use le_stream::derive::FromLeStream;
use num_traits::FromPrimitive;

use crate::ember::gp::{Address, KeyType, SecurityLevel};
use crate::ember::Status;
use crate::frame::Identified;
use crate::types::ByteSizedVec;
use crate::Error;

const ID: u16 = 0x00C5;

/// A callback invoked by the Zigbee GP stack when a GPDF is received.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    status: u8,
    payload: Payload,
}

impl Identified for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl TryFrom<Handler> for Payload {
    type Error = Error;

    fn try_from(handler: Handler) -> Result<Self, Self::Error> {
        match Status::from_u8(handler.status).ok_or(handler.status) {
            Ok(Status::Success) => Ok(handler.payload),
            other => Err(other.into()),
        }
    }
}

/// The payload of the GPDF receive.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Payload {
    gpd_link: u8,
    sequence_number: u8,
    addr: Address,
    gpdf_security_level: SecurityLevel,
    gpdf_security_key_type: KeyType,
    auto_commissioning: bool,
    bidirectional_info: u8,
    gpd_security_frame_counter: u32,
    gpd_command_id: u8,
    mic: u32,
    proxy_table_index: u8,
    #[allow(clippy::struct_field_names)]
    gpd_command_payload: ByteSizedVec<u8>,
}

impl Payload {
    /// The gpdLink value of the received GPDF.
    #[must_use]
    pub const fn gpd_link(&self) -> u8 {
        self.gpd_link
    }

    /// The GPDF sequence number.
    #[must_use]
    pub const fn sequence_number(&self) -> u8 {
        self.sequence_number
    }

    /// The address of the source GPD.
    #[must_use]
    pub const fn addr(&self) -> &Address {
        &self.addr
    }

    /// The security level of the received GPDF.
    #[must_use]
    pub const fn gpdf_security_level(&self) -> SecurityLevel {
        self.gpdf_security_level
    }

    /// The securityKeyType used to decrypt/authenticate the incoming GPDF.
    #[must_use]
    pub const fn gpdf_security_key_type(&self) -> KeyType {
        self.gpdf_security_key_type
    }

    /// Whether the incoming GPDF had the auto-commissioning bit set.
    #[must_use]
    pub const fn auto_commissioning(&self) -> bool {
        self.auto_commissioning
    }

    /// Bidirectional information represented in bitfields, where bit0 holds the rxAfterTx of
    /// incoming gpdf and bit1 holds if tx queue is available for outgoing gpdf.
    #[must_use]
    pub const fn bidirectional_info(&self) -> u8 {
        self.bidirectional_info
    }

    /// The security frame counter of the incoming GDPF.
    #[must_use]
    pub const fn gpd_security_frame_counter(&self) -> u32 {
        self.gpd_security_frame_counter
    }

    /// The gpdCommandId of the incoming GPDF.
    #[must_use]
    pub const fn gpd_command_id(&self) -> u8 {
        self.gpd_command_id
    }

    /// The received MIC of the GPDF.
    #[must_use]
    pub const fn mic(&self) -> u32 {
        self.mic
    }

    /// The proxy table index of the corresponding proxy table entry to the incoming GPDF.
    #[must_use]
    pub const fn proxy_table_index(&self) -> u8 {
        self.proxy_table_index
    }

    /// The GPD command payload.
    #[must_use]
    pub const fn gpd_command_payload(&self) -> &ByteSizedVec<u8> {
        &self.gpd_command_payload
    }
}
