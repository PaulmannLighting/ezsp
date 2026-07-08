/// Represent channel sizes for the UART.
#[derive(Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq, Hash)]
pub struct ChannelSizes {
    pub(crate) payload: usize,
    pub(crate) callbacks: usize,
    pub(crate) responses: usize,
}

impl ChannelSizes {
    /// Create a new channel sizes struct.
    #[must_use]
    pub const fn new(payload: usize, callbacks: usize, responses: usize) -> Self {
        Self {
            payload,
            callbacks,
            responses,
        }
    }

    /// Set the payload.
    #[must_use]
    pub const fn with_payload(&mut self, payload: usize) -> &mut Self {
        self.payload = payload;
        self
    }

    /// Set the callbacks channel size.
    #[must_use]
    pub const fn with_callbacks(&mut self, callbacks: usize) -> &mut Self {
        self.callbacks = callbacks;
        self
    }

    /// Set the response channel size.
    #[must_use]
    pub const fn with_responses(&mut self, responses: usize) -> &mut Self {
        self.responses = responses;
        self
    }
}

impl From<usize> for ChannelSizes {
    fn from(value: usize) -> Self {
        Self::new(value, value, value)
    }
}
