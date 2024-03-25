use crate::ember::binding::TableEntry;
use crate::ember::NodeId;
use crate::{Error, Transport};
use std::future::Future;

pub trait Binding: Transport {
    fn clear_binding_table(&mut self) -> impl Future<Output = Result<(), Error>>;

    fn set_binding(
        &mut self,
        index: u8,
        value: TableEntry,
    ) -> impl Future<Output = Result<(), Error>>;

    fn get_binding(&mut self, index: u8) -> impl Future<Output = Result<TableEntry, Error>>;

    fn delete_binding(&mut self, index: u8) -> impl Future<Output = Result<(), Error>>;

    fn binding_is_active(&mut self, index: u8) -> impl Future<Output = Result<bool, Error>>;

    fn get_binding_remote_node_id(
        &mut self,
        index: u8,
    ) -> impl Future<Output = Result<NodeId, Error>>;

    fn set_binding_remote_node_id(
        &mut self,
        index: u8,
        node_id: NodeId,
    ) -> impl Future<Output = Result<(), Error>>;
}
