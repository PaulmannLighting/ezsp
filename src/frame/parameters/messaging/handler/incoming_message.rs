use crate::ember::NodeId;
use crate::ember::aps::Frame;
use crate::ember::message::Incoming;
use crate::types::ByteSizedVec;

crate::frame::parameters::handler!(
    0x0045,
    {
        typ: u8,
        aps_frame: Frame,
        last_hop_lqi: u8,
        last_hop_rssi: i8,
        sender: NodeId,
        binding_index: u8,
        address_index: u8,
        message: ByteSizedVec<u8>,
        source_route_overhead: Option<u8>,
    },
    impl {
        impl Handler {
            /// The type of the incoming message.
            ///
            /// One of the following:
            ///
            /// - [`Incoming::Unicast`]
            /// - [`Incoming::UnicastReply`]
            /// - [`Incoming::Multicast`]
            /// - [`Incoming::MulticastLoopback`]
            /// - [`Incoming::Broadcast`]
            /// - [`Incoming::BroadcastLoopback`]
            ///
            /// # Errors
            ///
            /// Returns an error if the value is not a valid incoming message type.
            pub fn typ(&self) -> Result<Incoming, u8> {
                Incoming::try_from(self.typ)
            }

            /// Returns the raw incoming message type value.
            #[must_use]
            pub const fn typ_value(&self) -> u8 {
                self.typ
            }

            /// The APS frame from the incoming message.
            #[must_use]
            pub const fn aps_frame(&self) -> &Frame {
                &self.aps_frame
            }

            /// Removes APS fragmentation metadata after host-side reassembly.
            pub fn clear_fragmentation(&mut self) {
                self.aps_frame.clear_fragmentation();
            }

            /// The link quality from the node that last relayed the message.
            #[must_use]
            pub const fn last_hop_lqi(&self) -> u8 {
                self.last_hop_lqi
            }

            /// The energy level (in units of dBm) observed during the reception.
            #[must_use]
            pub const fn last_hop_rssi(&self) -> i8 {
                self.last_hop_rssi
            }

            /// The sender of the message.
            #[must_use]
            pub const fn sender(&self) -> NodeId {
                self.sender
            }

            /// The index of a binding that matches the message or 0xFF if there is no matching binding.
            #[must_use]
            pub const fn binding_index(&self) -> u8 {
                self.binding_index
            }

            /// The index of the entry in the address table that matches the sender
            /// of the message or 0xFF if there is no matching entry.
            #[must_use]
            pub const fn address_index(&self) -> u8 {
                self.address_index
            }

            /// The incoming message.
            #[must_use]
            pub fn message(&self) -> &[u8] {
                self.message.as_ref()
            }

            /// Returns the source route overhead, if any.
            #[must_use]
            pub const fn source_route_overhead(&self) -> Option<u8> {
                self.source_route_overhead
            }

            /// Consumes the handler and returns the incoming message.
            #[must_use]
            pub fn into_message(self) -> ByteSizedVec<u8> {
                self.message
            }
        }
    }
);
