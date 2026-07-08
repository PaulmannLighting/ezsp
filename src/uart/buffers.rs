/// A more or less sensible default channel size.
const DEFAULT_BUF_SIZE: usize = 128;

/// Buffer sizes to set up the `ASHv2` communication channels.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Buffers {
    /// Buffer size for the `ASHv2` receiver channel.
    pub ash_receiver: usize,

    /// Buffer size for the EZSP callback channel.
    pub ezsp_callbacks: usize,

    /// Buffer size for the EZSP messaging channel.
    pub ezsp_messages: usize,
}

impl Default for Buffers {
    fn default() -> Self {
        Self {
            ash_receiver: DEFAULT_BUF_SIZE,
            ezsp_callbacks: DEFAULT_BUF_SIZE,
            ezsp_messages: DEFAULT_BUF_SIZE,
        }
    }
}
