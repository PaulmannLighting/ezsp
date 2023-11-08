use crate::ember::node::Type;
use crate::ember::types::{Eui64, NodeId};
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Data {
    eui64: Eui64,
    typ: u8,
    id: NodeId,
    phy: u8,
    power: u8,
    timeout: u8,
    gpd_ieee_address: Eui64,
    source_id: u32,
    application_id: u8,
    endpoint: u8,
}

impl Data {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        eui64: Eui64,
        typ: Type,
        id: NodeId,
        phy: u8,
        power: u8,
        timeout: u8,
        gpd_ieee_address: Eui64,
        source_id: u32,
        application_id: u8,
        endpoint: u8,
    ) -> Self {
        Self {
            eui64,
            typ: typ.into(),
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
    pub const fn eui64(&self) -> Eui64 {
        self.eui64
    }

    #[must_use]
    pub fn typ(&self) -> Option<Type> {
        Type::from_u8(self.typ)
    }

    #[must_use]
    pub const fn id(&self) -> NodeId {
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
    pub const fn gpd_ieee_address(&self) -> Eui64 {
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
