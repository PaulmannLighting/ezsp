use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{once, Chain, Once};

#[derive(Debug, Eq, PartialEq)]
pub struct Data {
    channel: u8,
    lqi: u8,
    rssi: i8,
    depth: u8,
    nwk_update_id: u8,
    power: i8,
    parent_priority: i8,
    pan_id: u16,
    extended_pan_id: u64,
    sender: u16,
}

impl Data {
    #[must_use]
    pub const fn new(
        channel: u8,
        lqi: u8,
        rssi: i8,
        depth: u8,
        nwk_update_id: u8,
        power: i8,
        parent_priority: i8,
        pan_id: u16,
        extended_pan_id: u64,
        sender: u16,
    ) -> Self {
        Self {
            channel,
            lqi,
            rssi,
            depth,
            nwk_update_id,
            power,
            parent_priority,
            pan_id,
            extended_pan_id,
            sender,
        }
    }

    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }

    #[must_use]
    pub const fn lqi(&self) -> u8 {
        self.lqi
    }

    #[must_use]
    pub const fn rssi(&self) -> i8 {
        self.rssi
    }

    #[must_use]
    pub const fn depth(&self) -> u8 {
        self.depth
    }

    #[must_use]
    pub const fn nwk_update_id(&self) -> u8 {
        self.nwk_update_id
    }

    #[must_use]
    pub const fn power(&self) -> i8 {
        self.power
    }

    #[must_use]
    pub const fn parent_priority(&self) -> i8 {
        self.parent_priority
    }

    #[must_use]
    pub const fn pan_id(&self) -> u16 {
        self.pan_id
    }

    #[must_use]
    pub const fn extended_pan_id(&self) -> u64 {
        self.extended_pan_id
    }

    #[must_use]
    pub const fn sender(&self) -> u16 {
        self.sender
    }
}

impl Readable for Data {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let channel = src.read_num_le()?;
        let lqi = src.read_num_le()?;
        let rssi = src.read_num_le()?;
        let depth = src.read_num_le()?;
        let nwk_update_id = src.read_num_le()?;
        let power = src.read_num_le()?;
        let parent_priority = src.read_num_le()?;
        let pan_id = src.read_num_le()?;
        let extended_pan_id = src.read_num_le()?;
        let sender = src.read_num_le()?;
        Ok(Self {
            channel,
            lqi,
            rssi,
            depth,
            nwk_update_id,
            power,
            parent_priority,
            pan_id,
            extended_pan_id,
            sender,
        })
    }
}

impl IntoIterator for Data {
    type Item = u8;
    type IntoIter = Chain<
        Chain<
            Chain<
                Chain<
                    Chain<
                        Chain<
                            Chain<
                                Chain<
                                    Chain<Once<Self::Item>, IntoIter<Self::Item, 1>>,
                                    IntoIter<Self::Item, 1>,
                                >,
                                IntoIter<Self::Item, 1>,
                            >,
                            IntoIter<Self::Item, 1>,
                        >,
                        IntoIter<Self::Item, 1>,
                    >,
                    IntoIter<Self::Item, 1>,
                >,
                IntoIter<Self::Item, 2>,
            >,
            IntoIter<Self::Item, 8>,
        >,
        IntoIter<Self::Item, 2>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        once(self.channel)
            .chain(self.lqi.to_le_bytes())
            .chain(self.rssi.to_le_bytes())
            .chain(self.depth.to_le_bytes())
            .chain(self.nwk_update_id.to_le_bytes())
            .chain(self.power.to_le_bytes())
            .chain(self.parent_priority.to_le_bytes())
            .chain(self.pan_id.to_le_bytes())
            .chain(self.extended_pan_id.to_le_bytes())
            .chain(self.sender.to_le_bytes())
    }
}
