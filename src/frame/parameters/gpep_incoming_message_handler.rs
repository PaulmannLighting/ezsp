use crate::ember::gp::{Address, KeyType, SecurityLevel};
use crate::ember::Status;
use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x00C5;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
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
    gpd_command_payload: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub fn new(
        status: Status,
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
        gpd_command_payload: ByteSizedVec<u8>,
    ) -> Self {
        Self {
            status: status.into(),
            gpd_link,
            sequence_number,
            addr,
            gpdf_security_level,
            gpdf_security_key_type,
            auto_commissioning,
            bidirectional_info,
            gpd_security_frame_counter,
            gpd_command_id,
            mic,
            proxy_table_index,
            gpd_command_payload,
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn gpd_link(&self) -> u8 {
        self.gpd_link
    }

    #[must_use]
    pub const fn sequence_number(&self) -> u8 {
        self.sequence_number
    }

    #[must_use]
    pub const fn addr(&self) -> &Address {
        &self.addr
    }

    #[must_use]
    pub const fn gpdf_security_level(&self) -> SecurityLevel {
        self.gpdf_security_level
    }

    #[must_use]
    pub const fn gpdf_security_key_type(&self) -> KeyType {
        self.gpdf_security_key_type
    }

    #[must_use]
    pub const fn auto_commissioning(&self) -> bool {
        self.auto_commissioning
    }

    #[must_use]
    pub const fn bidirectional_info(&self) -> u8 {
        self.bidirectional_info
    }

    #[must_use]
    pub const fn gpd_security_frame_counter(&self) -> u32 {
        self.gpd_security_frame_counter
    }

    #[must_use]
    pub const fn gpd_command_id(&self) -> u8 {
        self.gpd_command_id
    }

    #[must_use]
    pub const fn mic(&self) -> u32 {
        self.mic
    }

    #[must_use]
    pub const fn proxy_table_index(&self) -> u8 {
        self.proxy_table_index
    }

    #[must_use]
    pub const fn gpd_command_payload(&self) -> &ByteSizedVec<u8> {
        &self.gpd_command_payload
    }
}
