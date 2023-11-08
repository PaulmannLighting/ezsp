pub const ID: u16 = 0x0108;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    endpoint: u8,
    cluster: u16,
    attribute_id: u16,
    mask: u8,
    manufacturer_code: u16,
}

impl Command {
    #[must_use]
    pub const fn new(
        endpoint: u8,
        cluster: u16,
        attribute_id: u16,
        mask: u8,
        manufacturer_code: u16,
    ) -> Self {
        Self {
            endpoint,
            cluster,
            attribute_id,
            mask,
            manufacturer_code,
        }
    }

    #[must_use]
    pub const fn endpoint(&self) -> u8 {
        self.endpoint
    }

    #[must_use]
    pub const fn cluster(&self) -> u16 {
        self.cluster
    }

    #[must_use]
    pub const fn attribute_id(&self) -> u16 {
        self.attribute_id
    }

    #[must_use]
    pub const fn mask(&self) -> u8 {
        self.mask
    }

    #[must_use]
    pub const fn manufacturer_code(&self) -> u16 {
        self.manufacturer_code
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    status: EmberStatus,
    data_type: u8,
    read_length: u8,
    data_ptr: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub const fn new(
        status: EmberStatus,
        data_type: u8,
        read_length: u8,
        data_ptr: ByteSizedVec<u8>,
    ) -> Self {
        Self {
            status,
            data_type,
            read_length,
            data_ptr,
        }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }

    #[must_use]
    pub const fn data_type(&self) -> u8 {
        self.data_type
    }

    #[must_use]
    pub const fn read_length(&self) -> u8 {
        self.read_length
    }

    #[must_use]
    pub const fn data_ptr(&self) -> ByteSizedVec<u8> {
        self.data_ptr
    }
}
