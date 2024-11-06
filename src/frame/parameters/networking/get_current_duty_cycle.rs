//! Parameters for the [`Networking::get_current_duty_cycle`](crate::Networking::get_current_duty_cycle) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use le_stream::FromLeStream;
use log::error;
use num_traits::FromPrimitive;

use crate::ember::{DeviceDutyCycles, Status};
use crate::ember::{PerDeviceDutyCycle, MAX_END_DEVICE_CHILDREN};
use crate::error::Error;
use crate::frame::Parameter;

const ID: u16 = 0x004C;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    max_devices: u8,
}

impl Command {
    #[must_use]
    pub const fn new(max_devices: u8) -> Self {
        Self { max_devices }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    // FIXME: The docs specify 33 * 4 + 1 bytes = 134 bytes, but that doesn't make sense.
    device_duty_cycles: DeviceDutyCycles,
}

impl Response {
    /// Returns the per-device duty cycles.
    #[must_use]
    pub fn device_duty_cycles(&self) -> heapless::Vec<PerDeviceDutyCycle, MAX_END_DEVICE_CHILDREN> {
        let [max_devices, per_device_duty_cycles @ ..] = self.device_duty_cycles;

        per_device_duty_cycles
            .chunks_exact(4)
            .take(max_devices as usize)
            .filter_map(|bytes| {
                PerDeviceDutyCycle::from_le_slice(bytes)
                    .inspect_err(|error| error!("Failed to parse per-device duty cycle: {error}"))
                    .ok()
            })
            .collect()
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Converts the response into a [`heapless::Vec`] of [`PerDeviceDutyCycle`]
/// or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for heapless::Vec<PerDeviceDutyCycle, MAX_END_DEVICE_CHILDREN> {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.device_duty_cycles()),
            other => Err(other.into()),
        }
    }
}
