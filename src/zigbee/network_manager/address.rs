/// Zigbee network address representation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Address {
    /// 16-bit network address
    Short(u16),
    /// 64-bit IEEE address
    Extended(u64),
}
