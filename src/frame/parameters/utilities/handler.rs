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
#[allow(variant_size_differences)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// The counter has rolled over.
    CounterRollover(CounterRollover),
    /// A custom frame has been received.
    CustomFrame(Box<CustomFrame>),
    /// The stack token has changed.
    StackTokenChanged(StackTokenChanged),
    /// A timer event has occurred.
    Timer(Timer),
}
