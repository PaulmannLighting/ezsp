use std::num::NonZero;

use apis_saltans_aps::data::Header;
use apis_saltans_aps::{Destination, ExtendedControl};
use apis_saltans_core::Endpoint;

use crate::ember::aps::{Frame, Options};

const STACK_ASSIGNED_APS_SEQUENCE: u8 = 0;

impl From<Header> for Frame {
    fn from(header: Header) -> Self {
        let (group_id, destination_endpoint) = match header.destination() {
            Destination::Unicast(endpoint) | Destination::Broadcast(endpoint) => (None, endpoint),
            Destination::Group(group) => (Some(group), Endpoint::Broadcast.into()),
        };
        let source_endpoint = header.source_endpoint().map_or_else(Into::into, Into::into);

        let mut frame = Self::new(
            header.profile_id(),
            header.cluster_id(),
            source_endpoint,
            destination_endpoint,
            Options::ENCRYPTION | Options::RETRY | Options::ENABLE_ROUTE_DISCOVERY,
            group_id.unwrap_or_default(),
            STACK_ASSIGNED_APS_SEQUENCE,
        );

        if let Some(extended) = header.extended() {
            if extended.control().contains(ExtendedControl::FIRST_FRAGMENT)
                && let Some(blocks) = extended.block_number()
            {
                frame.set_first_fragment(blocks);
            } else if extended
                .control()
                .contains(ExtendedControl::FOLLOWUP_FRAGMENT)
                && let Some(index) = extended.block_number().and_then(NonZero::new)
            {
                frame.set_followup_fragment(index);
            }
        }

        frame
    }
}
