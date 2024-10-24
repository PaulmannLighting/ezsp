use std::future::Future;

use crate::ember::binding::TableEntry;
use crate::ember::NodeId;
use crate::frame::parameters::binding::{
    clear_table, delete, get, get_remote_node_id, is_active, set, set_remote_node_id,
};
use crate::{Error, Transport};

/// The `Binding` trait provides an interface for the binding table.
pub trait Binding {
    /// Indicates whether any messages are currently being sent using this binding table entry.
    /// Note that this command does not indicate whether a binding is clear.
    /// To determine whether a binding is clear, check whether the type field of the
    /// [`TableEntry`] has the value [`Type::Unused`](crate::ember::binding::Type::Unused).
    #[allow(clippy::wrong_self_convention)]
    fn is_active(&mut self, index: u8) -> impl Future<Output = Result<bool, Error>> + Send;

    /// Deletes all binding table entries.
    fn clear_table(&mut self) -> impl Future<Output = Result<(), Error>> + Send;

    /// Deletes a binding table entry.
    fn delete(&mut self, index: u8) -> impl Future<Output = Result<(), Error>> + Send;

    /// Gets an entry from the binding table.
    fn get(&mut self, index: u8) -> impl Future<Output = Result<TableEntry, Error>> + Send;

    /// Returns the node ID for the binding's destination, if the ID is known.
    ///
    /// If a message is sent using the binding and the destination's ID is not known,
    /// the stack will discover the ID by broadcasting a ZDO address request.
    /// The application can avoid the need for this discovery by using
    /// [`set_binding_remote_node_id()`](Self::set_remote_node_id)
    /// when it knows the correct ID via some other means.
    ///
    /// The destination's node ID is forgotten when the binding is changed,
    /// when the local node reboots or, much more rarely,
    /// when the destination node changes its ID in response to an ID conflict.
    fn get_remote_node_id(
        &mut self,
        index: u8,
    ) -> impl Future<Output = Result<Option<NodeId>, Error>> + Send;

    /// Sets an entry in the binding table.
    fn set(
        &mut self,
        index: u8,
        value: TableEntry,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Set the node ID for the binding's destination.
    /// See [`get_binding_remote_node_id()`](Self::get_remote_node_id) for a description.
    fn set_remote_node_id(
        &mut self,
        index: u8,
        node_id: NodeId,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl<T> Binding for T
where
    T: Transport,
{
    async fn is_active(&mut self, index: u8) -> Result<bool, Error> {
        self.communicate::<_, is_active::Response>(is_active::Command::new(index))
            .await
            .map(|response| response.active())
    }

    async fn clear_table(&mut self) -> Result<(), Error> {
        self.communicate::<_, clear_table::Response>(clear_table::Command)
            .await?
            .try_into()
    }

    async fn delete(&mut self, index: u8) -> Result<(), Error> {
        self.communicate::<_, delete::Response>(delete::Command::new(index))
            .await?
            .try_into()
    }

    async fn get(&mut self, index: u8) -> Result<TableEntry, Error> {
        self.communicate::<_, get::Response>(get::Command::new(index))
            .await?
            .try_into()
    }

    async fn get_remote_node_id(&mut self, index: u8) -> Result<Option<NodeId>, Error> {
        self.communicate::<_, get_remote_node_id::Response>(get_remote_node_id::Command::new(index))
            .await
            .map(|response| response.node_id())
    }

    async fn set(&mut self, index: u8, value: TableEntry) -> Result<(), Error> {
        self.communicate::<_, set::Response>(set::Command::new(index, value))
            .await?
            .try_into()
    }

    async fn set_remote_node_id(&mut self, index: u8, node_id: NodeId) -> Result<(), Error> {
        self.communicate::<_, set_remote_node_id::Response>(set_remote_node_id::Command::new(
            index, node_id,
        ))
        .await
        .map(drop)
    }
}
