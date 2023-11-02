use crate::ember::node::Type;
use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{once, Chain, Once};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Data {
    eui64: u64,
    typ: Type,
    id: u16,
    phy: u8,
    power: u8,
    timeout: u8,
    gpd_ieee_address: u64,
    source_id: u32,
    application_id: u8,
    endpoint: u8,
}

impl Data {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        eui64: u64,
        typ: Type,
        id: u16,
        phy: u8,
        power: u8,
        timeout: u8,
        gpd_ieee_address: u64,
        source_id: u32,
        application_id: u8,
        endpoint: u8,
    ) -> Self {
        Self {
            eui64,
            typ,
            id,
            phy,
            power,
            timeout,
            gpd_ieee_address,
            source_id,
            application_id,
            endpoint,
        }
    }

    #[must_use]
    pub const fn eui64(&self) -> u64 {
        self.eui64
    }

    #[must_use]
    pub const fn typ(&self) -> Type {
        self.typ
    }

    #[must_use]
    pub const fn id(&self) -> u16 {
        self.id
    }

    #[must_use]
    pub const fn phy(&self) -> u8 {
        self.phy
    }

    #[must_use]
    pub const fn power(&self) -> u8 {
        self.power
    }

    #[must_use]
    pub const fn timeout(&self) -> u8 {
        self.timeout
    }

    #[must_use]
    pub const fn gpd_ieee_address(&self) -> u64 {
        self.gpd_ieee_address
    }

    #[must_use]
    pub const fn source_id(&self) -> u32 {
        self.source_id
    }

    #[must_use]
    pub const fn application_id(&self) -> u8 {
        self.application_id
    }

    #[must_use]
    pub const fn endpoint(&self) -> u8 {
        self.endpoint
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
                                    Chain<IntoIter<Self::Item, 8>, Once<Self::Item>>,
                                    IntoIter<Self::Item, 2>,
                                >,
                                IntoIter<Self::Item, 1>,
                            >,
                            IntoIter<Self::Item, 1>,
                        >,
                        IntoIter<Self::Item, 1>,
                    >,
                    IntoIter<Self::Item, 8>,
                >,
                IntoIter<Self::Item, 4>,
            >,
            IntoIter<Self::Item, 1>,
        >,
        IntoIter<Self::Item, 1>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.eui64
            .to_le_bytes()
            .into_iter()
            .chain(once(self.typ.into()))
            .chain(self.id.to_le_bytes())
            .chain(self.phy.to_le_bytes())
            .chain(self.power.to_le_bytes())
            .chain(self.timeout.to_le_bytes())
            .chain(self.gpd_ieee_address.to_le_bytes())
            .chain(self.source_id.to_le_bytes())
            .chain(self.application_id.to_le_bytes())
            .chain(self.endpoint.to_le_bytes())
    }
}

impl Readable for Data {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let eui64 = src.read_num_le()?;
        let typ: u8 = src.read_num_le()?;
        let id = src.read_num_le()?;
        let phy = src.read_num_le()?;
        let power = src.read_num_le()?;
        let timeout = src.read_num_le()?;
        let gpd_ieee_address = src.read_num_le()?;
        let source_id = src.read_num_le()?;
        let application_id = src.read_num_le()?;
        let endpoint = src.read_num_le()?;
        Ok(Self {
            eui64,
            typ: typ.try_into()?,
            id,
            phy,
            power,
            timeout,
            gpd_ieee_address,
            source_id,
            application_id,
            endpoint,
        })
    }
}
