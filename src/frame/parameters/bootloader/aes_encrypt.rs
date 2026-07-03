//! Parameters for the [`Bootloader::aes_encrypt()`](crate::Bootloader::aes_encrypt) command.

crate::frame::parameters::frame!(0x0094, { plaintext: [u8; 16], key: [u8; 16] }, { ciphertext: [u8; 16] });

impl Command {
    #[must_use]
    pub const fn new(plaintext: [u8; 16], key: [u8; 16]) -> Self {
        Self { plaintext, key }
    }
}

impl Response {
    /// Returns the ciphertext.
    #[must_use]
    pub const fn ciphertext(&self) -> [u8; 16] {
        self.ciphertext
    }
}
