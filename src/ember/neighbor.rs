use crate::ember::types::Eui64;
use le_stream::derive::{FromLeBytes, ToLeBytes};

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
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
