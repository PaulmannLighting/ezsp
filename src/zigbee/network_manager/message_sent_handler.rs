use crate::parameters::messaging::handler::MessageSent;
use crate::zigbee::NetworkManager;
use crate::{Error, Transport};

/// Trait for handling sent messages.
pub trait MessageSentHandler {
    fn handle_message_sent(
        &mut self,
        message_sent: MessageSent,
    ) -> impl Future<Output = Result<(), Error>>;
}

impl<T> MessageSentHandler for NetworkManager<T>
where
    T: Transport,
{
    async fn handle_message_sent(&mut self, message_sent: MessageSent) -> Result<(), Error> {
        todo!()
    }
}
