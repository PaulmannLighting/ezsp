//! Handlers for the utility commands.

mod counter_rollover;
mod custom_frame;
mod stack_token_changed;
mod timer;

pub use counter_rollover::Handler as CounterRollover;
pub use custom_frame::Handler as CustomFrame;
pub use stack_token_changed::Handler as StackTokenChanged;
pub use timer::Handler as Timer;

/// Callbacks for the utility parameters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// The counter has rolled over.
    CounterRollover(CounterRollover),
    /// A custom frame has been received.
    CustomFrame(CustomFrame),
    /// The stack token has changed.
    StackTokenChanged(StackTokenChanged),
    /// A timer event has occurred.
    Timer(Timer),
}

impl From<CounterRollover> for Handler {
    fn from(handler: CounterRollover) -> Self {
        Self::CounterRollover(handler)
    }
}

impl From<CustomFrame> for Handler {
    fn from(handler: CustomFrame) -> Self {
        Self::CustomFrame(handler)
    }
}

impl From<StackTokenChanged> for Handler {
    fn from(handler: StackTokenChanged) -> Self {
        Self::StackTokenChanged(handler)
    }
}

impl From<Timer> for Handler {
    fn from(handler: Timer) -> Self {
        Self::Timer(handler)
    }
}
