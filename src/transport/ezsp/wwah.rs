use crate::Transport;

pub trait Wwah {}

impl<T> Wwah for T where T: Transport {}
