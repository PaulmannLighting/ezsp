mod counters;
mod error;

use error::Error;
use le_stream::{FromLeBytes, ToLeBytes};

// XXX: Mockup
pub trait Transaction {
    type Response: FromLeBytes;

    fn command(&self) -> impl ToLeBytes;
}

pub trait Protocol {
    const NAME: &'static str;

    async fn write<T>(&mut self, packet: T) -> Result<(), Error>
    where
        T: ToLeBytes;

    async fn read<T>(&mut self) -> Result<T, Error>
    where
        T: FromLeBytes;

    async fn send_transaction<T>(&mut self, transaction: T) -> Result<T::Response, Error>
    where
        T: Transaction,
    {
        self.write(transaction.command()).await?;
        self.read().await
    }
}
