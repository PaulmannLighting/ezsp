
pub const ID: u16 = 0x00D1;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    routing_shortcut_thresh: u8,
}

impl Response {
    #[must_use]
    pub const fn new(routing_shortcut_thresh: u8) -> Self {
        Self { routing_shortcut_thresh }
    }

    #[must_use]
    pub const fn routing_shortcut_thresh(&self) -> u8 {
        self.routing_shortcut_thresh
    }
}
