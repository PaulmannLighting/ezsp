
pub const ID: u16 = 0x00DD;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    sink_index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(sink_index: u8) -> Self {
        Self { sink_index }
    }

    #[must_use]
    pub const fn sink_index(&self) -> u8 {
        self.sink_index
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    status: EmberStatus,
    entry: EmberGpSinkTableEntry,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, entry: EmberGpSinkTableEntry) -> Self {
        Self { status, entry }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }


    #[must_use]
    pub const fn entry(&self) -> EmberGpSinkTableEntry {
        self.entry
    }
}
