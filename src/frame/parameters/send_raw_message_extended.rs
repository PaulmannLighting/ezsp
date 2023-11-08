pub const ID: u16 = 0x0051;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    message_length: u8,
    message_contents: ByteSizedVec<u8>,
    priority: u8,
    use_cca: bool,
}

impl Command {
    #[must_use]
    pub const fn new(
        message_length: u8,
        message_contents: ByteSizedVec<u8>,
        priority: u8,
        use_cca: bool,
    ) -> Self {
        Self {
            message_length,
            message_contents,
            priority,
            use_cca,
        }
    }

    #[must_use]
    pub const fn message_length(&self) -> u8 {
        self.message_length
    }

    #[must_use]
    pub const fn message_contents(&self) -> ByteSizedVec<u8> {
        self.message_contents
    }

    #[must_use]
    pub const fn priority(&self) -> u8 {
        self.priority
    }

    #[must_use]
    pub const fn use_cca(&self) -> bool {
        self.use_cca
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    status: EmberStatus,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }
}
