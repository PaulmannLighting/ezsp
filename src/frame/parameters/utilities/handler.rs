pub mod counter_rollover;
pub mod custom_frame;
pub mod no_callbacks;
pub mod stack_token_changed;
pub mod timer;

/// Callbacks for the utility parameters.
#[allow(variant_size_differences, clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// The counter has rolled over.
    CounterRollover(counter_rollover::Handler),
    /// A custom frame has been received.
    CustomFrame(custom_frame::Handler),
    /// No callbacks are available.
    NoCallbacks(no_callbacks::Response),
    /// The stack token has changed.
    StackTokenChanged(stack_token_changed::Handler),
    /// A timer event has occurred.
    Timer(timer::Handler),
}

impl From<counter_rollover::Handler> for Handler {
    fn from(handler: counter_rollover::Handler) -> Self {
        Self::CounterRollover(handler)
    }
}

impl From<custom_frame::Handler> for Handler {
    fn from(handler: custom_frame::Handler) -> Self {
        Self::CustomFrame(handler)
    }
}

impl From<no_callbacks::Response> for Handler {
    fn from(response: no_callbacks::Response) -> Self {
        Self::NoCallbacks(response)
    }
}

impl From<stack_token_changed::Handler> for Handler {
    fn from(handler: stack_token_changed::Handler) -> Self {
        Self::StackTokenChanged(handler)
    }
}

impl From<timer::Handler> for Handler {
    fn from(handler: timer::Handler) -> Self {
        Self::Timer(handler)
    }
}
