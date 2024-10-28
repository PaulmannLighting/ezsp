use std::future::Future;

use crate::frame::Handler as Callback;

/// A callback handler.
pub trait Handler {
    /// Handle a callback.
    fn handle(&mut self, callback: Callback) -> impl Future<Output = ()> + Send;
}
