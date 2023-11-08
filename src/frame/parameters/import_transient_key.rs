
pub const ID: u16 = 0x0111;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    context: sl_zb_sec_man_context_t,
    eui64: EmberEUI64,
    plaintext_key: sl_zb_sec_man_key_t,
    flags: sl_zigbee_sec_man_flags_t,
}

impl Command {
    #[must_use]
    pub const fn new(context: sl_zb_sec_man_context_t, eui64: EmberEUI64, plaintext_key: sl_zb_sec_man_key_t, flags: sl_zigbee_sec_man_flags_t) -> Self {
        Self { context, eui64, plaintext_key, flags }
    }

    #[must_use]
    pub const fn context(&self) -> sl_zb_sec_man_context_t {
        self.context
    }


    #[must_use]
    pub const fn eui64(&self) -> EmberEUI64 {
        self.eui64
    }


    #[must_use]
    pub const fn plaintext_key(&self) -> sl_zb_sec_man_key_t {
        self.plaintext_key
    }


    #[must_use]
    pub const fn flags(&self) -> sl_zigbee_sec_man_flags_t {
        self.flags
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    status: sl_status_t,
    status: sl_status_t,
}

impl Response {
    #[must_use]
    pub const fn new(status: sl_status_t, status: sl_status_t) -> Self {
        Self { status, status }
    }

    #[must_use]
    pub const fn status(&self) -> sl_status_t {
        self.status
    }


    #[must_use]
    pub const fn status(&self) -> sl_status_t {
        self.status
    }
}
