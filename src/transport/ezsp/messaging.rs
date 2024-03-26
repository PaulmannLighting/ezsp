use crate::{Error, Transport};
use std::future::Future;

pub trait Messaging: Transport {
    fn address_table_entry_is_active(
        &mut self,
        address_table_index: u8,
    ) -> impl Future<Output = Result<bool, Error>> + Send;
}
