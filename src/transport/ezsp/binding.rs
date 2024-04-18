use crate::ember::binding::TableEntry;
use crate::ember::NodeId;
use crate::{Error, Transport};
use std::future::Future;

pub trait Binding: Transport {
    /// Deletes all binding table entries.
    fn clear_binding_table(&self) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sets an entry in the binding table.
    fn set_binding(
        &self,
        index: u8,
        value: TableEntry,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Gets an entry from the binding table.
    fn get_binding(&self, index: u8) -> impl Future<Output = Result<TableEntry, Error>> + Send;

    /// Deletes a binding table entry.
    fn delete_binding(&self, index: u8) -> impl Future<Output = Result<(), Error>> + Send;

    /// Indicates whether any messages are currently being sent using this binding table entry.
    /// Note that this command does not indicate whether a binding is clear.
    /// To determine whether a binding is clear, check whether the type field of the
    /// [`TableEntry`] has the value [`Type::Unused`](crate::ember::binding::Type::Unused).
    fn binding_is_active(&self, index: u8) -> impl Future<Output = Result<bool, Error>> + Send;

    /// Returns the node ID for the binding's destination, if the ID is known.
    /// If a message is sent using the binding and the destination's ID is not known,
    /// the stack will discover the ID by broadcasting a ZDO address request.
    /// The application can avoid the need for this discovery by using [`set_binding_remote_node_id()`](Self::set_binding_remote_node_id)
    /// when it knows the correct ID via some other means.
    /// The destination's node ID is forgotten when the binding is changed,
    /// when the local node reboots or, much more rarely,
    /// when the destination node changes its ID in response to an ID conflict.
    fn get_binding_remote_node_id(
        &self,
        index: u8,
    ) -> impl Future<Output = Result<NodeId, Error>> + Send;

    /// Set the node ID for the binding's destination.
    /// See [`get_binding_remote_node_id()`](Self::get_binding_remote_node_id) for a description.
    fn set_binding_remote_node_id(
        &self,
        index: u8,
        node_id: NodeId,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}
