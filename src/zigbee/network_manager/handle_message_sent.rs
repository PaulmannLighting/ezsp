use zigbee_nwk::Proxy;

use crate::Error;
use crate::parameters::messaging::handler::MessageSent;

/// Trait for handling sent messages.
pub trait HandleMessageSent {
    fn handle_message_sent(
        &self,
        message_sent: MessageSent,
    ) -> impl Future<Output = Result<(), Error>>;
}

impl<T> HandleMessageSent for T
where
    T: Proxy,
{
    async fn handle_message_sent(&self, message_sent: MessageSent) -> Result<(), Error> {
        todo!()
    }
}
