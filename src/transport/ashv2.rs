mod response_handler;

use crate::ezsp::Status;
use crate::frame::parameters::add_endpoint;
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
    frame_low: u8,
    frame_high: u8,
}

impl<S> Ashv2<S>
where
    S: SerialPort,
{
    #[must_use]
    pub const fn new(host: Host<S>) -> Self {
        Self {
            host,
            sequence: 0,
            frame_low: 0,
            frame_high: 0,
        }
    }

    fn next_command<T>(&mut self, frame_id: u16, parameters: T) -> Vec<u8>
    where
        T: ToLeBytes,
    {
        let mut command = vec![self.sequence, self.frame_low, self.frame_high];
        self.sequence += 1;
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
            .communicate::<ResponseHandler<add_endpoint::Response>, add_endpoint::Response, Error>(
                command.as_slice(),
            )
            .await
            .and_then(|response| response.status().map_err(Error::InvalidEzspStatus))
    }
}
