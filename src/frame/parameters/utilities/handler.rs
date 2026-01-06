//! Handlers for the utility commands.

pub use self::counter_rollover::Handler as CounterRollover;
pub use self::custom_frame::Handler as CustomFrame;
pub use self::stack_token_changed::Handler as StackTokenChanged;
pub use self::timer::Handler as Timer;

mod counter_rollover;
mod custom_frame;
mod stack_token_changed;
mod timer;

/// Callback handlers for utility commands.
#[cfg_attr(target_pointer_width = "64", expect(variant_size_differences))]
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
