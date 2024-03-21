mod response_handler;

use crate::ezsp::Status;
use crate::frame::header::Control;
use crate::frame::parameters::add_endpoint;
use crate::frame::Header;
use crate::transport::ashv2::response_handler::ResponseHandler;
use crate::transport::{Error, Transport};
use crate::types::ByteSizedVec;
use ashv2::Host;
use le_stream::ToLeBytes;
use serialport::SerialPort;

/// Mockup for ASHv2 command implementation.
#[derive(Debug)]
pub struct Ashv2<S>
where
    S: SerialPort + 'static,
{
    host: Host<S>,
    sequence: u8,
    control: Control,
}

impl<S> Ashv2<S>
where
    S: SerialPort,
{
    #[must_use]
    pub const fn new(host: Host<S>, control: Control) -> Self {
        Self {
            host,
            sequence: 0,
            control,
        }
    }

    fn next_command<T>(&mut self, frame_id: u16, parameters: T) -> Vec<u8>
    where
        T: ToLeBytes,
    {
        let mut command = Vec::new();
        command.extend(Header::new(self.sequence, self.control, frame_id).to_le_bytes());
        self.sequence = self.sequence.checked_add(1).unwrap_or(0);
        command.extend_from_slice(&frame_id.to_le_bytes());
        command.extend(parameters.to_le_bytes());
        command
    }
}

impl<S> Transport for Ashv2<S>
where
    S: SerialPort,
{
    async fn add_endpoint(
        &mut self,
        endpoint: u8,
        profile_id: u16,
        device_id: u16,
        app_flags: u8,
        input_clusters: ByteSizedVec<u16>,
        output_clusters: ByteSizedVec<u16>,
    ) -> Result<Status, Error> {
        let command = self.next_command(
            add_endpoint::ID,
            add_endpoint::Command::new(
                endpoint,
                profile_id,
                device_id,
                app_flags,
                input_clusters,
                output_clusters,
            ),
        );
        self.host
            .communicate::<ResponseHandler<add_endpoint::Response>>(command.as_slice())
            .await
            .and_then(|response| response.status().map_err(Error::InvalidEzspStatus))
    }
}
