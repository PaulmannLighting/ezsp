use crate::ember::zigbee::Network;
use crate::frame::Parameters;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{Chain, Once};

pub const ID: u16 = 0x001B;

/// Reports that a network was found as a result of a prior call to startScan.
///
/// Gives the network parameters useful for deciding which network to join.
#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    network_found: Network,
    last_hop_lqi: u8,
    last_hop_rssi: i8,
}

impl Response {
    #[must_use]
    pub const fn new(network_found: Network, last_hop_lqi: u8, last_hop_rssi: i8) -> Self {
        Self {
            network_found,
            last_hop_lqi,
            last_hop_rssi,
        }
    }

    #[must_use]
    pub const fn network_found(&self) -> &Network {
        &self.network_found
    }

    #[must_use]
    pub const fn last_hop_lqi(&self) -> u8 {
        self.last_hop_lqi
    }

    #[must_use]
    pub const fn last_hop_rssi(&self) -> i8 {
        self.last_hop_rssi
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = Chain<
        Chain<
            Chain<
                Chain<
                    Chain<
                        Chain<
                            Chain<Once<Self::Item>, IntoIter<Self::Item, 2>>,
                            IntoIter<Self::Item, 8>,
                        >,
                        Once<Self::Item>,
                    >,
                    IntoIter<Self::Item, 1>,
                >,
                IntoIter<Self::Item, 1>,
            >,
            IntoIter<Self::Item, 1>,
        >,
        IntoIter<Self::Item, 1>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.network_found
            .into_iter()
            .chain(self.last_hop_lqi.to_be_bytes())
            .chain(self.last_hop_rssi.to_be_bytes())
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let network_found = Network::read_from(src)?;
        let mut buffer @ [last_hop_lqi, last_hop_rssi] = [0; 2];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            network_found,
            last_hop_lqi,
            last_hop_rssi: i8::from_be_bytes([last_hop_rssi]),
        })
    }
}
