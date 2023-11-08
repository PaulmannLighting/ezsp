pub const ID: u16 = 0x010C;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    context_in: sl_zb_sec_man_context_t,
}

impl Command {
    #[must_use]
    pub const fn new(context_in: sl_zb_sec_man_context_t) -> Self {
        Self { context_in }
    }

    #[must_use]
    pub const fn context_in(&self) -> sl_zb_sec_man_context_t {
        self.context_in
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    eui: EmberEUI64,
    key_data: sl_zb_sec_man_aps_key_metadata_t,
    status: sl_status_t,
}

impl Response {
    #[must_use]
    pub const fn new(
        eui: EmberEUI64,
        key_data: sl_zb_sec_man_aps_key_metadata_t,
        status: sl_status_t,
    ) -> Self {
        Self {
            eui,
            key_data,
            status,
        }
    }

    #[must_use]
    pub const fn eui(&self) -> EmberEUI64 {
        self.eui
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
