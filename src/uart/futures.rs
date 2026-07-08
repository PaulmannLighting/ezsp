use std::pin::Pin;

type BoxedFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

/// Futures that drive an EZSP-UART transport.
///
/// The UART transport no longer spawns background tasks internally. Callers
/// must spawn or otherwise poll all futures in this struct for the serial link,
/// `ASHv2` actor, and EZSP frame splitter to make progress.
pub struct Futures<T> {
    /// Drives the blocking serial-port worker and resolves with the original
    /// serial port when the async reader and writer halves are dropped.
    pub serial_worker: BoxedFuture<T>,

    /// Drives outbound `ASHv2` frame transmission.
    pub ash_transmitter: BoxedFuture<()>,

    /// Drives inbound `ASHv2` frame reception.
    pub ash_receiver: BoxedFuture<()>,

    /// Routes decoded EZSP frames into response and callback channels.
    pub frame_splitter: BoxedFuture<()>,
}

impl<T> Futures<T> {
    /// Create an EZSP-UART future set from the EZSP splitter future and `ASHv2`
    /// actor futures.
    pub fn new(
        frame_splitter: BoxedFuture<()>,
        ash_futures: ashv2::Futures<
            impl Future<Output = T> + Send + 'static,
            impl Future<Output = ()> + Send + 'static,
            impl Future<Output = ()> + Send + 'static,
        >,
    ) -> Self {
        Self {
            ash_transmitter: Box::pin(ash_futures.transmitter),
            ash_receiver: Box::pin(ash_futures.receiver),
            serial_worker: Box::pin(ash_futures.serial_worker),
            frame_splitter: Box::pin(frame_splitter),
        }
    }
}
