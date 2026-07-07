//! Parameters for the [`Networking::get_current_duty_cycle`](crate::Networking::get_current_duty_cycle) command.

use le_stream::FromLeStream;
use log::error;
use num_traits::FromPrimitive;

use crate::ember::{DeviceDutyCycles, MAX_END_DEVICE_CHILDREN, PerDeviceDutyCycle, Status};
use crate::error::Error;
crate::frame::parameters::frame!(
    0x004C,
    { max_devices: u8 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(max_devices: u8) -> Self {
                Self { max_devices }
            }
        }
    },
    {
        status: u8,
        device_duty_cycles: DeviceDutyCycles,
    } => Networking(networking)::GetCurrentDutyCycle,
    impl {
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
    }
);
