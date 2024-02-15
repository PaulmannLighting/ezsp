mod counters;
mod error;
mod transaction;

use error::Error;
use le_stream::{FromLeBytes, ToLeBytes};
use transaction::Transaction;

pub trait Protocol {
    const NAME: &'static str;

    async fn write<T>(&mut self, packet: T) -> Result<(), Error>
    where
        T: ToLeBytes;

    async fn read<T>(&mut self) -> Result<T, Error>
    where
        T: FromLeBytes;

    async fn communicate<T>(&mut self, transaction: T) -> Result<T::Response, Error>
    where
        T: Transaction,
    {
        self.write(transaction.command()).await?;
        self.read().await
    }
}
