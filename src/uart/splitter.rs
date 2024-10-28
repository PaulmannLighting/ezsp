use ashv2::Payload;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::{Handler, Response};

async fn splitter(
    mut incoming: Receiver<Payload>,
    responses: Sender<Response>,
    callbacks: Sender<Handler>,
) {
    loop {
        match incoming.recv().await {
            Some(payload) => {
                let handler = Handler::new(payload, responses.clone());
                callbacks.send(handler).await.unwrap();
            }
            None => break,
        }
    }
}
