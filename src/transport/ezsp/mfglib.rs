use crate::Transport;

pub trait Mfglib {}

impl<T> Mfglib for T where T: Transport {}
