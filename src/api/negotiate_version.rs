use std::num::NonZero;

use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

use crate::Error;
use crate::api::message::Message;

pub trait NegotiateVersion {
    fn negotiate_version(
        &self,
        desired_version: NonZero<u8>,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl NegotiateVersion for Sender<Message> {
    async fn negotiate_version(&self, desired_version: NonZero<u8>) -> Result<(), Error> {
        let (response, rx) = oneshot::channel();

        self.send(Message::Connect {
            desired_version,
            response,
        })
        .await?;

        rx.await?
    }
}
