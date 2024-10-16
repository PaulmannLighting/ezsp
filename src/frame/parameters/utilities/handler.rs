pub mod counter_rollover;
pub mod custom_frame;
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
