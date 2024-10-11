use bitflags::bitflags;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Command(u8);

bitflags! {
    impl Command: u8 {
        const IS_RESPONSE = 0b1000_0000;
        const NETWORK_INDEX_1 = 0b0100_0000;
        const NETWORK_INDEX_0 = 0b0010_0000;
        const CALLBACK_TYPE_1 = 0b0001_0000;
        const CALLBACK_TYPE_0 = 0b0000_1000;
        const CALLBACK_PENDING = 0b0000_0100;
        const TRUNCATED = 0b0000_0010;
        const OVERFLOW = 0b0000_0001;
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Response(u8);

bitflags! {
    impl Response: u8 {
        const IS_RESPONSE = 0b1000_0000;
        const NETWORK_INDEX_1 = 0b0100_0000;
        const NETWORK_INDEX_0 = 0b0010_0000;
        const CALLBACK_TYPE_1 = 0b0001_0000;
        const CALLBACK_TYPE_0 = 0b0000_1000;
        const CALLBACK_PENDING = 0b0000_0100;
        const TRUNCATED = 0b0000_0010;
        const OVERFLOW = 0b0000_0001;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LowByte {
    Command(Command),
    Response(Response),
}

impl From<u8> for LowByte {
    fn from(byte: u8) -> Self {
        if byte & Response::IS_RESPONSE.bits() != 0 {
            LowByte::Response(Response::from_bits_retain(byte))
        } else {
            LowByte::Command(Command::from_bits_retain(byte))
        }
    }
}
