use crate::Transport;

pub trait Zll {}

impl<T> Zll for T where T: Transport {}
