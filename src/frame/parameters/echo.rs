pub const ID: u16 = 0x0081;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    data_length: u8,
    data: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(data_length: u8, data: ByteSizedVec<u8>) -> Self {
        Self { data_length, data }
    }

    #[must_use]
    pub const fn data_length(&self) -> u8 {
        self.data_length
    }

    #[must_use]
    pub const fn data(&self) -> ByteSizedVec<u8> {
        self.data
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    echo_length: u8,
    echo: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub const fn new(echo_length: u8, echo: ByteSizedVec<u8>) -> Self {
        Self { echo_length, echo }
    }

    #[must_use]
    pub const fn echo_length(&self) -> u8 {
        self.echo_length
    }

    #[must_use]
    pub const fn echo(&self) -> ByteSizedVec<u8> {
        self.echo
    }
}
