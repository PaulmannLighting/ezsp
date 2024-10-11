use bitflags::bitflags;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HighByte(u8);

bitflags! {
    impl Command: u8 {
        const SECURITY_ENABLED = 0b1000_0000;
        const PADDING_ENABLED = 0b0100_0000;
        const FRAME_FORMAT_VERSION_1 = 0b0000_0010;
        const FRAME_FORMAT_VERSION_0 = 0b0000_0001;
    }
}
