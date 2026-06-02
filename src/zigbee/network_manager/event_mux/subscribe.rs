use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::{Receiver, Sender};
use zigbee_hw::Event;

use super::Message;

/// Allow subscription to the event handler.
pub trait Subscribe {
    /// Subscribe to the event multiplexer.
    ///
    /// # Errors
    ///
    /// Returns a [`SendError<Message>`] if sending to the multiplexer fails.
    fn subscribe(
        &mut self,
        buffer: usize,
    ) -> impl Future<Output = Result<Receiver<Event>, SendError<Message>>> + Send;
}

impl Subscribe for Sender<Message> {
    async fn subscribe(&mut self, buffer: usize) -> Result<Receiver<Event>, SendError<Message>> {
        let (tx, rx) = tokio::sync::mpsc::channel(buffer);
        self.send(Message::Subscribe(tx)).await?;
        Ok(rx)
    }
}
