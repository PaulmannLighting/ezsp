use crate::ember::types::Eui64;
use crate::read_write::Readable;
use crate::Error;
use rw_exact_ext::ReadExactExt;
use std::array::IntoIter;
use std::io::Read;
use std::iter::Chain;

#[derive(Debug, Eq, PartialEq)]
pub struct TableEntry {
    short_id: u16,
    average_lqi: u8,
    in_cost: u8,
    out_cost: u8,
    age: u8,
    long_id: Eui64,
}

impl TableEntry {
    #[must_use]
    pub const fn new(
        short_id: u16,
        average_lqi: u8,
        in_cost: u8,
        out_cost: u8,
        age: u8,
        long_id: Eui64,
    ) -> Self {
        Self {
            short_id,
            average_lqi,
            in_cost,
            out_cost,
            age,
            long_id,
        }
    }

    #[must_use]
    pub const fn short_id(&self) -> u16 {
        self.short_id
    }

    #[must_use]
    pub const fn average_lqi(&self) -> u8 {
        self.average_lqi
    }

    #[must_use]
    pub const fn in_cost(&self) -> u8 {
        self.in_cost
    }

    #[must_use]
    pub const fn out_cost(&self) -> u8 {
        self.out_cost
    }

    #[must_use]
    pub const fn age(&self) -> u8 {
        self.age
    }

    #[must_use]
    pub const fn long_id(&self) -> Eui64 {
        self.long_id
    }
}

impl IntoIterator for TableEntry {
    type Item = u8;
    type IntoIter = Chain<
        Chain<
            Chain<
                Chain<
                    Chain<IntoIter<Self::Item, 2>, IntoIter<Self::Item, 1>>,
                    IntoIter<Self::Item, 1>,
                >,
                IntoIter<Self::Item, 1>,
            >,
            IntoIter<Self::Item, 1>,
        >,
        IntoIter<Self::Item, 8>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.short_id
            .to_le_bytes()
            .into_iter()
            .chain(self.average_lqi.to_le_bytes())
            .chain(self.in_cost.to_le_bytes())
            .chain(self.out_cost.to_le_bytes())
            .chain(self.age.to_le_bytes())
            .chain(self.long_id.to_le_bytes())
    }
}

impl Readable for TableEntry {
    fn try_read<R>(src: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let short_id = src.read_num_le()?;
        let average_lqi = src.read_num_le()?;
        let in_cost = src.read_num_le()?;
        let out_cost = src.read_num_le()?;
        let age = src.read_num_le()?;
        let long_id = src.read_num_le()?;
        Ok(Self {
            short_id,
            average_lqi,
            in_cost,
            out_cost,
            age,
            long_id,
        })
    }
}
