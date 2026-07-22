/// Transport futures returned by [`Builder::ashv2`](crate::Builder::ashv2).
///
/// The caller must spawn these futures as tasks in dependency order:
///
/// 1. [`ash_futures.serial_worker`](ashv2::Futures::serial_worker);
/// 2. [`ash_futures.transmitter`](ashv2::Futures::transmitter);
/// 3. [`ash_futures.receiver`](ashv2::Futures::receiver);
/// 4. [`ezsp_tx`](Self::ezsp_tx); and
/// 5. [`ezsp_rx`](Self::ezsp_rx).
///
/// Spawn all five before awaiting [`Builder::start`](crate::Builder::start).
/// Starting a higher layer before the lower layers that service it can leave
/// bounded-channel operations waiting on futures that are not being polled and
/// deadlock initialization.
pub struct Futures<SerialWorker, AshTx, AshRx, EzspTx, EzspRx> {
    /// Futures that drive the `ASHv2` serial worker, transmitter, and receiver.
    pub ash_futures: ashv2::Futures<SerialWorker, AshTx, AshRx>,
    /// Future that drives the EZSP transmitter actor.
    pub ezsp_tx: EzspTx,
    /// Future that drives the EZSP receiver actor.
    pub ezsp_rx: EzspRx,
}
