use std::future::Future;

use crate::ember::binding::TableEntry;
use crate::ember::NodeId;
use crate::frame::parameters::binding::{
    binding_is_active, clear_binding_table, delete_binding, get_binding,
    get_binding_remote_node_id, set_binding, set_binding_remote_node_id,
};
use crate::Resolve;
use crate::{Error, Transport};

/// The `Binding` trait provides an interface for the binding table.
pub trait Binding {
    /// Indicates whether any messages are currently being sent using this binding table entry.
    /// Note that this command does not indicate whether a binding is clear.
    /// To determine whether a binding is clear, check whether the type field of the
    /// [`TableEntry`] has the value [`Type::Unused`](crate::ember::binding::Type::Unused).
    fn binding_is_active(&mut self, index: u8) -> impl Future<Output = Result<bool, Error>> + Send;

    /// Deletes all binding table entries.
    fn clear_binding_table(&mut self) -> impl Future<Output = Result<(), Error>> + Send;

    /// Deletes a binding table entry.
    fn delete_binding(&mut self, index: u8) -> impl Future<Output = Result<(), Error>> + Send;

    /// Gets an entry from the binding table.
    fn get_binding(&mut self, index: u8) -> impl Future<Output = Result<TableEntry, Error>> + Send;

    /// Returns the node ID for the binding's destination, if the ID is known.
    ///
    /// If a message is sent using the binding and the destination's ID is not known,
    /// the stack will discover the ID by broadcasting a ZDO address request.
    /// The application can avoid the need for this discovery by using
    /// [`set_binding_remote_node_id()`](Self::set_binding_remote_node_id)
    /// when it knows the correct ID via some other means.
    ///
    /// The destination's node ID is forgotten when the binding is changed,
    /// when the local node reboots or, much more rarely,
    /// when the destination node changes its ID in response to an ID conflict.
    fn get_binding_remote_node_id(
        &mut self,
        index: u8,
    ) -> impl Future<Output = Result<NodeId, Error>> + Send;

    /// Sets an entry in the binding table.
    fn set_binding(
        &mut self,
        index: u8,
        value: TableEntry,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Set the node ID for the binding's destination.
    /// See [`get_binding_remote_node_id()`](Self::get_binding_remote_node_id) for a description.
    fn set_binding_remote_node_id(
        &mut self,
        index: u8,
        node_id: NodeId,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl<T> Binding for T
where
    T: Transport,
{
    async fn binding_is_active(&mut self, index: u8) -> Result<bool, Error> {
        self.communicate::<_, binding_is_active::Response>(binding_is_active::Command::new(index))
            .await
            .map(|response| response.active())
    }

    async fn clear_binding_table(&mut self) -> Result<(), Error> {
        self.communicate::<_, clear_binding_table::Response>(clear_binding_table::Command)
            .await?
            .resolve()
    }

    async fn delete_binding(&mut self, index: u8) -> Result<(), Error> {
        self.communicate::<_, delete_binding::Response>(delete_binding::Command::new(index))
            .await?
            .resolve()
    }

    async fn get_binding(&mut self, index: u8) -> Result<TableEntry, Error> {
        self.communicate::<_, get_binding::Response>(get_binding::Command::new(index))
            .await?
            .resolve()
    }

    async fn get_binding_remote_node_id(&mut self, index: u8) -> Result<NodeId, Error> {
        self.communicate::<_, get_binding_remote_node_id::Response>(
            get_binding_remote_node_id::Command::new(index),
        )
        .await
        .map(|response| response.node_id())
    }

    async fn set_binding(&mut self, index: u8, value: TableEntry) -> Result<(), Error> {
        self.communicate::<_, set_binding::Response>(set_binding::Command::new(index, value))
            .await?
            .resolve()
    }

    async fn set_binding_remote_node_id(
        &mut self,
        index: u8,
        node_id: NodeId,
    ) -> Result<(), Error> {
        self.communicate::<_, set_binding_remote_node_id::Response>(
            set_binding_remote_node_id::Command::new(index, node_id),
        )
        .await
        .map(drop)
    }
}
