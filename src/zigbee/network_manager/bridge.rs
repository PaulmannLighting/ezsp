use std::time::Duration;

use log::{error, trace};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::sleep;

/// Bridge two channels.
///
/// This function will read all messages from `input`, convert them to the message type of `output`
/// and send it to the output channel.
pub async fn bridge<T, U>(mut input: Receiver<T>, output: Sender<U>, burst: Option<Duration>)
where
    T: Into<U>,
{
    while let Some(msg_in) = input.recv().await {
        let mut msg_out = msg_in.into();

        while let Err(error) = output.send(msg_out).await {
            error!("Target channel is congested: {error}");
            msg_out = error.0;

            if let Some(duration) = burst {
                trace!("Retrying in {duration:?}");
                sleep(duration).await;
            }
        }
    }
}
