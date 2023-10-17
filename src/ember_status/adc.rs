use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Adc {
    ConversionDone = 0x80,
    ConversionBusy = 0x81,
    ConversionDeferred = 0x82,
    NoConversionPending = 0x84,
}
