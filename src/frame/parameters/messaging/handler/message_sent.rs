use std::fmt::Display;

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::aps::Frame;
use crate::ember::message::Outgoing;
use crate::types::ByteSizedVec;

crate::frame::parameters::handler!(
    0x003F,
    {
        typ: u8,
        index_or_destination: u16,
        aps_frame: Frame,
        message_tag: u8,
        status: u8,
        message: ByteSizedVec<u8>,
    },
    impl {
        impl Handler {
            /// The type of message sent.
            ///
            /// # Errors
            ///
            /// Returns an error if the value is not a valid [`Outgoing`] variant.
            pub fn typ(&self) -> Result<Outgoing, u8> {
                Outgoing::try_from(self.typ)
            }

            /// The destination to which the message was sent, for direct unicasts,
            /// or the address table or binding index for other unicasts.
            ///
            /// The value is unspecified for multicasts and broadcasts.
            #[must_use]
            pub const fn index_or_destination(&self) -> u16 {
                self.index_or_destination
            }

            /// The APS frame for the message.
            #[must_use]
            pub const fn aps_frame(&self) -> &Frame {
                &self.aps_frame
            }

            /// The value supplied by the Host in the
            /// [`Messaging::send_unicast`](crate::Messaging::send_unicast),
            /// [`Messaging::send_broadcast`](crate::Messaging::send_broadcast) or
            /// [`Messaging::send_multicast`](crate::Messaging::send_multicast) command.
            #[must_use]
            pub const fn message_tag(&self) -> u8 {
                self.message_tag
            }

            /// Return the status.
            ///
            /// # Errors
            ///
            /// Returns the raw status value if it is not a valid status.
            pub fn status(&self) -> Result<Status, u8> {
                Status::from_u8(self.status).ok_or(self.status)
            }

            /// Returns `true` if an ACK was received from the destination or `false` if no ACK was received.
            ///
            /// # Errors
            ///
            /// Returns an [`Error`] if the status is not [`Status::Success`] or [`Status::DeliveryFailed`]
            /// or if the value is not a valid status.
            pub fn ack_received(&self) -> Result<bool, Error> {
                match self.status() {
                    Ok(Status::Success) => Ok(true),
                    Ok(Status::DeliveryFailed) => Ok(false),
                    other => Err(other.into()),
                }
            }

            /// The unicast message supplied by the Host.
            ///
            /// The message contents are only included here if the decision for the
            /// `messageContentsInCallback` policy is `messageTagAndContentsInCallback`.
            #[must_use]
            pub fn message(&self) -> &[u8] {
                self.message.as_ref()
            }
        }

        impl Display for Handler {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let message_length = self.message.len();
                write!(
                    f,
                    "Handler {{ typ: {:#04X}, index_or_destination: {:#06X}, aps_frame: {}, message_tag: {:#04X}, status: {:#04X}, message_length: {message_length:#04X}, message: [",
                    self.typ, self.index_or_destination, self.aps_frame, self.message_tag, self.status,
                )?;

                let mut message_bytes = self.message.iter();

                if let Some(byte) = message_bytes.next() {
                    write!(f, "{byte:#04X}")?;

                    for byte in message_bytes {
                        write!(f, ", {byte:#04X}")?;
                    }
                }

                write!(f, "] }}")
            }
        }
    }
);
