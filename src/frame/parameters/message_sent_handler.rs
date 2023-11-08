
pub const ID: u16 = 0x003F;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    type: EmberOutgoingMessageType,
    index_or_destination: u16,
    aps_frame: EmberApsFrame,
    message_tag: u8,
    status: EmberStatus,
    message_length: u8,
    message_contents: uint8_t[],
}

impl Response {
    #[must_use]
    pub const fn new(type: EmberOutgoingMessageType, index_or_destination: u16, aps_frame: EmberApsFrame, message_tag: u8, status: EmberStatus, message_length: u8, message_contents: uint8_t[]) -> Self {
        Self { type, index_or_destination, aps_frame, message_tag, status, message_length, message_contents }
    }

    #[must_use]
    pub const fn type(&self) -> EmberOutgoingMessageType {
        self.type
    }


    #[must_use]
    pub const fn index_or_destination(&self) -> u16 {
        self.index_or_destination
    }


    #[must_use]
    pub const fn aps_frame(&self) -> EmberApsFrame {
        self.aps_frame
    }


    #[must_use]
    pub const fn message_tag(&self) -> u8 {
        self.message_tag
    }


    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
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
