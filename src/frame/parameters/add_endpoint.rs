
pub const ID: u16 = 0x0002;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    endpoint: u8,
    profile_id: u16,
    device_id: u16,
    app_flags: u8,
    input_cluster_count: u8,
    output_cluster_count: u8,
    input_cluster_list: uint16_t[],
    output_cluster_list: uint16_t[],
}

impl Command {
    #[must_use]
    pub const fn new(endpoint: u8, profile_id: u16, device_id: u16, app_flags: u8, input_cluster_count: u8, output_cluster_count: u8, input_cluster_list: uint16_t[], output_cluster_list: uint16_t[]) -> Self {
        Self { endpoint, profile_id, device_id, app_flags, input_cluster_count, output_cluster_count, input_cluster_list, output_cluster_list }
    }

    #[must_use]
    pub const fn endpoint(&self) -> u8 {
        self.endpoint
    }


    #[must_use]
    pub const fn profile_id(&self) -> u16 {
        self.profile_id
    }


    #[must_use]
    pub const fn device_id(&self) -> u16 {
        self.device_id
    }


    #[must_use]
    pub const fn app_flags(&self) -> u8 {
        self.app_flags
    }


    #[must_use]
    pub const fn input_cluster_count(&self) -> u8 {
        self.input_cluster_count
    }


    #[must_use]
    pub const fn output_cluster_count(&self) -> u8 {
        self.output_cluster_count
    }


    #[must_use]
    pub const fn input_cluster_list(&self) -> uint16_t[] {
        self.input_cluster_list
    }


    #[must_use]
    pub const fn output_cluster_list(&self) -> uint16_t[] {
        self.output_cluster_list
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    status: EzspStatus,
}

impl Response {
    #[must_use]
    pub const fn new(status: EzspStatus) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> EzspStatus {
        self.status
    }
}
