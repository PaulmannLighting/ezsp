use crate::frame::parameters::add_endpoint::{Command, Response};

#[derive(Debug)]
pub struct Transaction {
    command: Command,
    response: Option<Response>,
}

impl Transaction {
    #[must_use]
    pub const fn new(command: Command) -> Self {
        Self {
            command,
            response: None,
        }
    }

    pub const fn response(&self) -> Option<&Response> {
        self.response.as_ref()
    }
}
