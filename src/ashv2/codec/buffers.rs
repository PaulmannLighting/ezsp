use crate::{MAX_HEADER_SIZE, MAX_PARAMETER_SIZE};

#[derive(Debug, Default)]
pub struct Buffers {
    pub header: heapless::Vec<u8, MAX_HEADER_SIZE>,
    pub parameters: heapless::Vec<u8, MAX_PARAMETER_SIZE>,
}

impl Buffers {
    /// Clears the buffers.
    pub fn clear(&mut self) {
        self.header.clear();
        self.parameters.clear();
    }
}
