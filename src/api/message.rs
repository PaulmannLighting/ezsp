use std::num::NonZero;
use std::sync::Arc;

use tokio::sync::oneshot::Sender;
use tokio::sync::{OwnedRwLockReadGuard, OwnedRwLockWriteGuard};

use crate::frame::Commands;
use crate::{Error, Frame, Parameters};

/// Keeps a command's maintenance permit alive until its response arrives,
/// even if the original caller is canceled.
#[derive(Clone, Debug)]
pub enum CommandGuard {
    Read {
        _guard: Arc<OwnedRwLockReadGuard<()>>,
    },

    Write {
        _guard: Arc<OwnedRwLockWriteGuard<()>>,
    },
}

#[derive(Debug)]
pub enum Message {
    Connect {
        desired_version: NonZero<u8>,
        response: Sender<Result<(), Error>>,
    },
    Command {
        command: Commands,
        response: Sender<Result<Parameters, Error>>,
        guard: CommandGuard,
    },
    Response(Frame<Parameters>),
}
