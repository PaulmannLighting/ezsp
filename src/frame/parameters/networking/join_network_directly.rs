use crate::ember::beacon::Data;
use crate::ember::node::Type;
use crate::ember::Status;
use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{once, Chain, Once};

pub const ID: u16 = 0x003B;

/// Causes the stack to associate with the network using the specified network parameters in the beacon parameter.
///
/// It can take several seconds for the stack to associate with the local network.
/// Do not send messages until the stackStatusHandler callback informs you that the stack is up.
/// Unlike ::emberJoinNetwork(), this function does not issue an active scan before joining.
/// Instead, it will cause the local node to issue a MAC Association Request directly to the specified target node.
/// It is assumed that the beacon parameter is an artifact after issuing an active scan.
/// (For more information, see emberGetBestBeacon and emberGetNextBeacon.)
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    local_node_type: Type,
    beacon: Data,
    radio_tx_power: i8,
    clear_beacons_after_network_up: bool,
}

impl Command {
    #[must_use]
    pub const fn new(
        local_node_type: Type,
        beacon: Data,
        radio_tx_power: i8,
        clear_beacons_after_network_up: bool,
    ) -> Self {
        Self {
            local_node_type,
            beacon,
            radio_tx_power,
            clear_beacons_after_network_up,
        }
    }

    #[must_use]
    pub const fn local_node_type(&self) -> Type {
        self.local_node_type
    }

    #[must_use]
    pub const fn beacon(&self) -> &Data {
        &self.beacon
    }

    #[must_use]
    pub const fn radio_tx_power(&self) -> i8 {
        self.radio_tx_power
    }

    #[must_use]
    pub const fn clear_beacons_after_network_up(&self) -> bool {
        self.clear_beacons_after_network_up
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = Chain<
        Chain<Chain<Once<Self::Item>, <Data as IntoIterator>::IntoIter>, IntoIter<Self::Item, 1>>,
        Once<Self::Item>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        once(self.local_node_type.into())
            .chain(self.beacon)
            .chain(self.radio_tx_power.to_be_bytes())
            .chain(once(self.clear_beacons_after_network_up.into()))
    }
}

impl Readable for Command {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let local_node_type = src.read_num_be::<u8, 1>()?;
        let beacon = Data::read_from(src)?;
        let radio_tx_power = src.read_num_be()?;
        let clear_beacons_after_network_up = src.read_bool()?;
        Ok(Self {
            local_node_type: local_node_type.try_into()?,
            beacon,
            radio_tx_power,
            clear_beacons_after_network_up,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    status: Status,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.status.into())
    }
}

impl Readable for Response {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        Ok(Self {
            status: src.read_num_be::<u8, 1>()?.try_into()?,
        })
    }
}
