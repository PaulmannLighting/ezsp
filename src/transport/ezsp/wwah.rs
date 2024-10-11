use crate::Transport;

/// The `Wwah` trait provides an interface for the Work With All Hubs (WWAH) protocol.
pub trait Wwah {}

impl<T> Wwah for T where T: Transport {}
