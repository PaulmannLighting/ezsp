use le_stream::{FromLeBytes, ToLeBytes};

pub trait Transaction {
    type Command: ToLeBytes;
    type Response: FromLeBytes;

    fn command(&self) -> Self::Command;
}
