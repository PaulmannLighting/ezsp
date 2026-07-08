use std::pin::Pin;

type BoxedFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

pub struct Futures<T> {
    pub serial_worker: BoxedFuture<T>,
    pub ash_transmitter: BoxedFuture<()>,
    pub ash_receiver: BoxedFuture<()>,
    pub frame_splitter: BoxedFuture<()>,
}

impl<T> Futures<T> {
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
