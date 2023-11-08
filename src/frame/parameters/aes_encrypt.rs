
pub const ID: u16 = 0x0094;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    plaintext: uint8_t[16],
    key: uint8_t[16],
}

impl Command {
    #[must_use]
    pub const fn new(plaintext: uint8_t[16], key: uint8_t[16]) -> Self {
        Self { plaintext, key }
    }

    #[must_use]
    pub const fn plaintext(&self) -> uint8_t[16] {
        self.plaintext
    }


    #[must_use]
    pub const fn key(&self) -> uint8_t[16] {
        self.key
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    ciphertext: uint8_t[16],
}

impl Response {
    #[must_use]
    pub const fn new(ciphertext: uint8_t[16]) -> Self {
        Self { ciphertext }
    }

    #[must_use]
    pub const fn ciphertext(&self) -> uint8_t[16] {
        self.ciphertext
    }
}
