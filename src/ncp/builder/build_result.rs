use crate::Ncp;

/// Initialized NCP and the futures that drive application event delivery.
///
/// Spawn [`bridge`](Self::bridge) before [`event_handler`](Self::event_handler)
/// so callbacks are forwarded through the bounded message channel in the same
/// dependency order in which they are consumed. Both futures must remain
/// running while the [`Ncp`] is in use.
pub struct BuildResult<Bridge, EventHandler> {
    /// Initialized high-level NCP handle.
    pub ncp: Ncp,
    /// Future that forwards EZSP callbacks to the event handler.
    pub bridge: Bridge,
    /// Future that translates callbacks and correlates high-level responses.
    pub event_handler: EventHandler,
}
