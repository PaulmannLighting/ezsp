
pub const ID: u16 = 0x0037;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    source: EmberNodeId,
    destination: EmberNodeId,
    nwk_sequence: u8,
    aps_frame: EmberApsFrame,
    radius: u8,
    message_tag: u8,
    message_length: u8,
    message_contents: uint8_t[],
}

impl Command {
    #[must_use]
    pub const fn new(source: EmberNodeId, destination: EmberNodeId, nwk_sequence: u8, aps_frame: EmberApsFrame, radius: u8, message_tag: u8, message_length: u8, message_contents: uint8_t[]) -> Self {
        Self { source, destination, nwk_sequence, aps_frame, radius, message_tag, message_length, message_contents }
    }

    #[must_use]
    pub const fn source(&self) -> EmberNodeId {
        self.source
    }


    #[must_use]
    pub const fn destination(&self) -> EmberNodeId {
        self.destination
    }


    #[must_use]
    pub const fn nwk_sequence(&self) -> u8 {
        self.nwk_sequence
    }


    #[must_use]
    pub const fn aps_frame(&self) -> EmberApsFrame {
        self.aps_frame
    }


    #[must_use]
    pub const fn radius(&self) -> u8 {
        self.radius
    }


    #[must_use]
    pub const fn message_tag(&self) -> u8 {
        self.message_tag
    }


    #[must_use]
    pub const fn message_length(&self) -> u8 {
        self.message_length
    }


    #[must_use]
    pub const fn message_contents(&self) -> uint8_t[] {
        self.message_contents
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    status: EmberStatus,
    aps_sequence: u8,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, aps_sequence: u8) -> Self {
        Self { status, aps_sequence }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }


    #[must_use]
    pub const fn aps_sequence(&self) -> u8 {
        self.aps_sequence
    }
}
