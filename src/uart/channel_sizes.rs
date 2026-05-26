/// Represent channel sizes for the UART.
#[derive(Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq, Hash)]
pub struct ChannelSizes {
    pub(crate) payload: usize,
    pub(crate) message_queue: usize,
    pub(crate) callbacks: usize,
    pub(crate) responses: usize,
}

impl ChannelSizes {
    /// Create a new channel sizes struct.
    #[must_use]
    pub const fn new(
        payload: usize,
        message_queue: usize,
        callbacks: usize,
        responses: usize,
    ) -> Self {
        Self {
            payload,
            message_queue,
            callbacks,
            responses,
        }
    }

    /// Set the payload.
    #[must_use]
    pub const fn with_payload(mut self, payload: usize) -> Self {
        self.payload = payload;
        self
    }

    /// Set the message queue length.
    #[must_use]
    pub const fn with_message_queue(mut self, message_queue: usize) -> Self {
        self.message_queue = message_queue;
        self
    }

    /// Set the callbacks channel size.
    #[must_use]
    pub const fn with_callbacks(mut self, callbacks: usize) -> Self {
        self.callbacks = callbacks;
        self
    }

    /// Set the response channel size.
    #[must_use]
    pub const fn with_responses(mut self, responses: usize) -> Self {
        self.responses = responses;
        self
    }
}

impl From<usize> for ChannelSizes {
    fn from(value: usize) -> Self {
        Self::new(value, value, value, value)
    }
}
