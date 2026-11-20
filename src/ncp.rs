//! High-level EZSP Network Co-Processor helper.
//!
//! [`Ncp`] wraps a connected EZSP communicator and adds the state needed by
//! host-side Zigbee workflows: endpoint cluster metadata, APS message tags,
//! baseline and per-message APS options, scan aggregation, message-sent
//! correlation, and callback dispatch through a background event handler.
//! Public send methods accept an application APS sequence and carry it in
//! EZSP's message-tag field; the NCP assigns the APS sequence stored in the
//! outgoing EZSP APS frame itself.
//!
//! [`Builder`] negotiates the protocol version through caller-spawned transport
//! actors, configures the stack, registers endpoints, and returns an [`Ncp`]
//! together with callback-processing futures for the caller to spawn.
//! [`Startup`] records whether the builder should restore the NCP's persisted
//! network or explicitly form a new network. With the
//! `apis-saltans` feature, `Ncp` also implements
//! `apis_saltans_hw::Driver` and gains conversions
//! between EZSP and `apis-saltans` endpoint, scan, APS, and event types.

use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::error::SendError;

pub use self::builder::{BuildResult, Builder};
pub use self::endpoint::Endpoint;
pub use self::event_handler::EventHandler;
pub use self::initialization_parameters::InitializationParameters;
pub use self::message::Message;
pub use self::multicast_options::MulticastOptions;
pub use self::network_credentials::NetworkCredentials;
pub use self::scans::Scans;
pub use self::startup::Startup;
use crate::ember::aps::Options;
use crate::{Connection, Error};

mod await_event;
pub mod builder;
mod endpoint;
mod event_handler;
mod initialization_parameters;
mod message;
mod messaging;
mod multicast_options;
mod network_credentials;
mod scanning;
mod scans;
mod startup;

// The ZDP profile ID.
const ZDP: u16 = 0x0000;

/// Host-side helper for an EZSP Network Co-Processor.
///
/// `Ncp` owns a cloneable [`Connection`] actor handle. Its methods provide
/// higher-level operations
/// that need callback correlation or local host state, such as scans, outgoing
/// APS message confirmation, and source endpoint lookup from the configured
/// endpoint cluster lists. Outgoing frames combine the baseline APS options
/// stored by [`Builder`] with options supplied to each send method. The builder
/// gives another clone of the connected handle to the background [`EventHandler`].
#[derive(Debug)]
pub struct Ncp {
    pub(crate) connection: Connection,
    pub(crate) endpoints: Box<[Endpoint]>,
    event_handler_handle: Sender<Message>,
    options: Options,
    #[cfg(feature = "apis-saltans")]
    pub(crate) backup_profile: Option<crate::apis_saltans::backup::OfflineTokenProfile>,
}

impl Ncp {
    /// Borrows the negotiated EZSP connection for direct command access.
    ///
    /// Use the reference for read and write commands or clone it to retain an
    /// owned [`Connection`] after transferring ownership of the `Ncp`.
    #[must_use]
    pub const fn connection(&self) -> &Connection {
        &self.connection
    }

    /// Returns the first configured local endpoint that advertises an output cluster.
    ///
    /// ZDP messages always use endpoint zero. For other profiles, the endpoint
    /// registry is searched in supplied order and the first
    /// endpoint containing `cluster_id` in its output-cluster set is returned.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NoMatchingSourceEndpoint`] when no configured local
    /// endpoint advertises `cluster_id` as an output cluster.
    pub fn source_endpoint(&self, profile_id: u16, cluster_id: u16) -> Result<u8, Error> {
        if profile_id == ZDP {
            return Ok(0);
        }

        self.endpoints
            .iter()
            .find_map(|endpoint| {
                if endpoint.output_clusters.contains(&cluster_id) {
                    Some(endpoint.id)
                } else {
                    None
                }
            })
            .ok_or(Error::NoMatchingSourceEndpoint(cluster_id))
    }

    /// Sends a termination request to the background event handler.
    ///
    /// # Errors
    ///
    /// Returns [`SendError`] if the termination
    /// request cannot be sent to the message handler.
    pub async fn terminate(self) -> Result<(), SendError<Message>> {
        self.event_handler_handle.send(Message::Terminate).await
    }

    /// Registers endpoints and constructs a high-level NCP helper.
    ///
    /// Each endpoint is registered on the NCP before the value is returned.
    /// The supplied event-handler sender must feed the same callback handler
    /// that receives callbacks for `connection`, because scans and APS send
    /// confirmations are correlated through that channel. `options` provides
    /// the baseline APS flags that are combined with each send's options.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any endpoint registration command fails.
    pub async fn new(
        mut connection: Connection,
        endpoints: Box<[Endpoint]>,
        event_handler_handle: Sender<Message>,
        options: Options,
    ) -> Result<Self, Error> {
        for endpoint in endpoints.iter().cloned() {
            endpoint.add_to(&mut connection).await?;
        }

        Ok(Self {
            connection,
            endpoints,
            event_handler_handle,
            options,
            #[cfg(feature = "apis-saltans")]
            backup_profile: None,
        })
    }
}
