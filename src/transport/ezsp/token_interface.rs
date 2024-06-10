use crate::Transport;

pub trait TokenInterface {}

impl<T> TokenInterface for T where T: Transport {}
