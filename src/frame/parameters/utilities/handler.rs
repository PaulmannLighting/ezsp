pub mod counter_rollover;
pub mod custom_frame;
pub mod stack_token_changed;
pub mod timer_handler;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    CounterRollover(counter_rollover::Handler),
    CustomFrame(custom_frame::Handler),
    StackTokenChanged(stack_token_changed::Handler),
    TimerHandler(timer_handler::Handler),
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

impl From<timer_handler::Handler> for Handler {
    fn from(handler: timer_handler::Handler) -> Self {
        Self::TimerHandler(handler)
    }
}
