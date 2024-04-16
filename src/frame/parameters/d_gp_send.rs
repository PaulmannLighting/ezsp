use crate::ember::gp::Address;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x00C6;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    action: bool,
    use_cca: bool,
    addr: Address,
    gpd_command_id: u8,
    gpd_asdu: ByteSizedVec<u8>,
    gpep_handle: u8,
    gp_tx_queue_entry_lifetime_ms: u16,
}

impl Command {
    #[must_use]
    pub const fn new(
        action: bool,
        use_cca: bool,
        addr: Address,
        gpd_command_id: u8,
        gpd_asdu: ByteSizedVec<u8>,
        gpep_handle: u8,
        gp_tx_queue_entry_lifetime_ms: u16,
    ) -> Self {
        Self {
            action,
            use_cca,
            addr,
            gpd_command_id,
            gpd_asdu,
            gpep_handle,
            gp_tx_queue_entry_lifetime_ms,
        }
    }

    #[must_use]
    pub const fn action(&self) -> bool {
        self.action
    }

    #[must_use]
    pub const fn use_cca(&self) -> bool {
        self.use_cca
    }

    #[must_use]
    pub const fn addr(&self) -> &Address {
        &self.addr
    }

    #[must_use]
    pub const fn gpd_command_id(&self) -> u8 {
        self.gpd_command_id
    }

    #[must_use]
    pub const fn gpd_asdu(&self) -> &ByteSizedVec<u8> {
        &self.gpd_asdu
    }

    #[must_use]
    pub const fn gpep_handle(&self) -> u8 {
        self.gpep_handle
    }

    #[must_use]
    pub const fn gp_tx_queue_entry_lifetime_ms(&self) -> u16 {
        self.gp_tx_queue_entry_lifetime_ms
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
}

impl Response {
    #[must_use]
    pub fn new(status: Status) -> Self {
        Self {
            status: status.into(),
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
