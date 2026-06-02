use log::error;
use tokio::sync::mpsc::{Receiver, Sender};

/// Bridge two channels.
///
/// This function will read all messages from `input`, convert them to the message type of `output`
/// and send it to the output channel.
pub async fn bridge<T, U>(mut input: Receiver<T>, output: Sender<U>)
where
    T: Into<U>,
{
    while let Some(msg) = input.recv().await {
        if let Err(error) = output.send(msg.into()).await {
            error!("Callback to message translation channel is congested: {error}");
        }
    }
}
