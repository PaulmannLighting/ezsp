use apis_saltans_aps::Data;
use apis_saltans_nwk::{Envelope, Metadata, Source};
use bytes::Bytes;

use crate::parameters::messaging::handler::IncomingMessage;

impl TryFrom<IncomingMessage> for Envelope<Data<Bytes>> {
    type Error = <Data<Bytes> as TryFrom<IncomingMessage>>::Error;

    fn try_from(incoming_message: IncomingMessage) -> Result<Self, Self::Error> {
        let src_address = incoming_message.sender();
        let metadata = Metadata::from(&incoming_message);
        incoming_message
            .try_into()
            .map(|frame| Self::new(Source::new(src_address, None), metadata, frame))
    }
}
