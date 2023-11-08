
pub const ID: u16 = 0x0110;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    eui: EmberEUI64,
}

impl Command {
    #[must_use]
    pub const fn new(eui: EmberEUI64) -> Self {
        Self { eui }
    }

    #[must_use]
    pub const fn eui(&self) -> EmberEUI64 {
        self.eui
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    plaintext_key: sl_zb_sec_man_key_t,
    index: u8,
    key_data: sl_zb_sec_man_aps_key_metadata_t,
    status: sl_status_t,
}

impl Response {
    #[must_use]
    pub const fn new(plaintext_key: sl_zb_sec_man_key_t, index: u8, key_data: sl_zb_sec_man_aps_key_metadata_t, status: sl_status_t) -> Self {
        Self { plaintext_key, index, key_data, status }
    }

    #[must_use]
    pub const fn plaintext_key(&self) -> sl_zb_sec_man_key_t {
        self.plaintext_key
    }


    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }


    #[must_use]
    pub const fn key_data(&self) -> sl_zb_sec_man_aps_key_metadata_t {
        self.key_data
    }


    #[must_use]
    pub const fn status(&self) -> sl_status_t {
        self.status
    }
}
