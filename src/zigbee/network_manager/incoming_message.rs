use log::{debug, warn};
use zigbee_nwk::{Error, Proxy};

use crate::ember::aps::Options;
use crate::parameters::messaging::handler::IncomingMessage;

/// Trait for handling incoming messages.
pub trait HandlerIncomingMessage {
    fn handle_incoming_message(
        &mut self,
        message: IncomingMessage,
    ) -> impl Future<Output = Result<(), Error>>;
}

impl<T> HandlerIncomingMessage for T
where
    T: Proxy,
{
    async fn handle_incoming_message(&mut self, message: IncomingMessage) -> Result<(), Error> {
        debug!("Handling incoming message: {message:?}");
        let ember_aps_frame = message.aps_frame();
        let pan_id = self.get_pan_id().await?;

        if ember_aps_frame.options().contains(Options::FRAGMENT) {
            warn!(
                "Received fragmented message, but fragmentation handling is not yet properly implemented."
            );
            // TODO: Implement proper fragmentation handling.
            /*
            self.proxy
                .send_reply(
                    pan_id,
                    aps::Frame::new(
                        ember_aps_frame.profile_id(),
                        ember_aps_frame.cluster_id(),
                        0x01, // TODO: Get endpoint from device.
                        ember_aps_frame.source_endpoint(),
                        Options::FRAGMENT,
                        ember_aps_frame.group_id(),
                        ember_aps_frame.sequence(),
                    ),
                    ByteSizedVec::default(),
                )
                .await?;
                */
        }

        // TODO: Implement acknowledgment sending for Proxy..
        /*
        let seq = self.next_aps_seq();
        let tag = self.next_message_tag();
        let ack = Acknowledgment::new(
            ember_aps_frame.sequence(),
            Some(AckFmt::new(
                ember_aps_frame.source_endpoint(),
                ember_aps_frame.cluster_id(),
                ember_aps_frame.profile_id(),
                0x01, // TODO: Get endpoint from device.
            )),
            None,
        );

        self.proxy
            .send_unicast(
                Destination::Direct(message.sender()),
                aps::Frame::new(
                    ember_aps_frame.profile_id(),
                    ember_aps_frame.cluster_id(),
                    0x01, // TODO: Get endpoint from device.
                    ember_aps_frame.source_endpoint(),
                    if ember_aps_frame.options().contains(Options::ENCRYPTION) {
                        Options::ENCRYPTION
                    } else {
                        Options::NONE
                    },
                    ember_aps_frame.group_id(),
                    seq,
                ),
                tag,
                ack.to_le_stream().collect(),
            )
            .await?;


         */
        Ok(())
    }
}
