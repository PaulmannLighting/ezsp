use crate::ember::join_method::JoinMethod;
use rw_exact_ext::ReadExactExt;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{once, Chain, Once};

#[derive(Debug, Eq, PartialEq)]
pub struct Parameters {
    extended_pan_id: u64,
    pan_id: u16,
    radio_tx_power: u8,
    radio_channel: u8,
    join_method: JoinMethod,
    nwk_manager_id: u16,
    nwk_update_id: u8,
    channels: u32,
}

impl Parameters {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        extended_pan_id: u64,
        pan_id: u16,
        radio_tx_power: u8,
        radio_channel: u8,
        join_method: JoinMethod,
        nwk_manager_id: u16,
        nwk_update_id: u8,
        channels: u32,
    ) -> Self {
        Self {
            extended_pan_id,
            pan_id,
            radio_tx_power,
            radio_channel,
            join_method,
            nwk_manager_id,
            nwk_update_id,
            channels,
        }
    }

    #[must_use]
    pub const fn extended_pan_id(&self) -> u64 {
        self.extended_pan_id
    }

    #[must_use]
    pub const fn pan_id(&self) -> u16 {
        self.pan_id
    }

    #[must_use]
    pub const fn radio_tx_power(&self) -> u8 {
        self.radio_tx_power
    }

    #[must_use]
    pub const fn radio_channel(&self) -> u8 {
        self.radio_channel
    }

    #[must_use]
    pub const fn join_method(&self) -> JoinMethod {
        self.join_method
    }

    #[must_use]
    pub const fn nwk_manager_id(&self) -> u16 {
        self.nwk_manager_id
    }

    #[must_use]
    pub const fn nwk_update_id(&self) -> u8 {
        self.nwk_update_id
    }

    #[must_use]
    pub const fn channels(&self) -> u32 {
        self.channels
    }

    /// Read [`Parameters`] from a reader.
    ///
    /// # Errors
    /// Returns an [`anyhow::Error`] on read errors.
    pub fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let extended_pan_id = src.read_num_be()?;
        let pan_id = src.read_num_be()?;
        let radio_tx_power = src.read_num_be()?;
        let radio_channel = src.read_num_be()?;
        let join_method: u8 = src.read_num_be()?;
        let nwk_manager_id = src.read_num_be()?;
        let nwk_update_id = src.read_num_be()?;
        let channels = src.read_num_be()?;
        Ok(Self {
            extended_pan_id,
            pan_id,
            radio_tx_power,
            radio_channel,
            join_method: join_method.try_into()?,
            nwk_manager_id,
            nwk_update_id,
            channels,
        })
    }
}

impl IntoIterator for Parameters {
    type Item = u8;
    type IntoIter = Chain<
        Chain<
            Chain<
                Chain<
                    Chain<
                        Chain<
                            Chain<IntoIter<Self::Item, 8>, IntoIter<Self::Item, 2>>,
                            Once<Self::Item>,
                        >,
                        Once<Self::Item>,
                    >,
                    Once<Self::Item>,
                >,
                IntoIter<Self::Item, 2>,
            >,
            Once<Self::Item>,
        >,
        IntoIter<Self::Item, 4>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.extended_pan_id
            .to_be_bytes()
            .into_iter()
            .chain(self.pan_id.to_be_bytes())
            .chain(once(self.radio_tx_power))
            .chain(once(self.radio_channel))
            .chain(once(self.join_method.into()))
            .chain(self.nwk_manager_id.to_be_bytes())
            .chain(once(self.nwk_update_id))
            .chain(self.channels.to_be_bytes())
    }
}
