
pub const ID: u16 = 0x006F;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    context: EmberAesMmoHashContext,
    finalize: bool,
    length: u8,
    data: uint8_t[],
}

impl Command {
    #[must_use]
    pub const fn new(context: EmberAesMmoHashContext, finalize: bool, length: u8, data: uint8_t[]) -> Self {
        Self { context, finalize, length, data }
    }

    #[must_use]
    pub const fn context(&self) -> EmberAesMmoHashContext {
        self.context
    }


    #[must_use]
    pub const fn finalize(&self) -> bool {
        self.finalize
    }


    #[must_use]
    pub const fn length(&self) -> u8 {
        self.length
    }


    #[must_use]
    pub const fn data(&self) -> uint8_t[] {
        self.data
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    status: EmberStatus,
    return_context: EmberAesMmoHashContext,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, return_context: EmberAesMmoHashContext) -> Self {
        Self { status, return_context }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }


    #[must_use]
    pub const fn return_context(&self) -> EmberAesMmoHashContext {
        self.return_context
    }
}
