//! Conversion implementations between EZSP callback data and `apis-saltans` types.
//!
//! These conversions are used by the feature-gated NCP driver adapter and event
//! handler. They translate incoming APS frames, endpoint descriptors, scan
//! results, join/leave callbacks, and stack status callbacks into the types
//! expected by `apis_saltans_hw` and the EZSP endpoint registration command.

use apis_saltans_hw::Event;

pub use self::error::ParseApsFrameError;
use crate::Callback;
use crate::frame::parameters::networking::handler::Handler as Networking;
use crate::parameters::trust_center::handler::Handler as TrustCenter;

mod address;
mod defragmented_message;
mod envelope;
mod error;
mod event;
mod found_network;
mod metadata;
mod scanned_channel;

impl TryFrom<Callback> for Event {
    type Error = ();

    fn try_from(callback: Callback) -> Result<Self, Self::Error> {
        match callback {
            Callback::Networking(Networking::ChildJoin(child_join)) => {
                return Self::try_from(*child_join).map_err(drop);
            }
            Callback::Networking(Networking::StackStatus(status)) => {
                if let Ok(status) = status.result() {
                    return Self::try_from(status).map_err(drop);
                }
            }
            Callback::TrustCenter(TrustCenter::TrustCenterJoin(trust_center_join)) => {
                return Self::try_from(*trust_center_join).map_err(drop);
            }
            _ => return Err(()),
        }

        Err(())
    }
}
