use std::num::NonZero;

use tokio::sync::oneshot::Sender;

use crate::frame::Commands;
use crate::{Error, Frame, Parameters};

#[derive(Debug)]
pub enum Message {
    Connect {
        desired_version: NonZero<u8>,
        response: Sender<Result<(), Error>>,
    },
    Command {
        command: Commands,
        response: Sender<Result<Parameters, Error>>,
    },
    Response(Frame<Parameters>),
}
