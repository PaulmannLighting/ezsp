use crate::ember::binding::TableEntry;
use crate::ember::NodeId;
use crate::{Error, Transport};
use std::future::Future;

pub trait Binding: Transport {
    fn clear_binding_table(&self) -> impl Future<Output = Result<(), Error>> + Send;

    fn set_binding(
        &self,
        index: u8,
        value: TableEntry,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn get_binding(&self, index: u8) -> impl Future<Output = Result<TableEntry, Error>> + Send;

    fn delete_binding(&self, index: u8) -> impl Future<Output = Result<(), Error>> + Send;

    fn binding_is_active(&self, index: u8) -> impl Future<Output = Result<bool, Error>> + Send;

    fn get_binding_remote_node_id(
        &self,
        index: u8,
    ) -> impl Future<Output = Result<NodeId, Error>> + Send;

    fn set_binding_remote_node_id(
        &self,
        index: u8,
        node_id: NodeId,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}
