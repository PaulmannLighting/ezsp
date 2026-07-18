use apis_saltans_hw::aps::Data;
use apis_saltans_hw::nwk::{Envelope, Metadata, Source};
use bytes::Bytes;

use crate::DefragmentedMessage;

impl TryFrom<DefragmentedMessage> for Envelope<Data<Bytes>> {
    type Error = <Data<Bytes> as TryFrom<DefragmentedMessage>>::Error;

    fn try_from(defragmented_message: DefragmentedMessage) -> Result<Self, Self::Error> {
        let src_address = defragmented_message.sender();
        let metadata = Metadata::from(&defragmented_message);
        defragmented_message
            .try_into()
            .map(|frame| Self::new(Source::new(src_address, None), metadata, frame))
    }
}
