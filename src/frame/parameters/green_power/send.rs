use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::gp::Address;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use crate::{Error, ValueError};

const ID: u16 = 0x00C6;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
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
}

impl Parameter for Command {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}

impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        Status::from_u8(response.status)
            .ok_or_else(|| ValueError::Ember(response.status).into())
            .and_then(|status| {
                if status == Status::Success {
                    Ok(())
                } else {
                    Err(status.into())
                }
            })
    }
}
