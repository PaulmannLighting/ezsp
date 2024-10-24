use le_stream::derive::FromLeStream;
use num_traits::FromPrimitive;

use crate::ember::duty_cycle::State;
use crate::frame::Identified;

const ID: u16 = 0x004D;

/// Callback fires when the duty cycle state has changed
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    channel_page: u8,
    channel: u8,
    state: u8,
    total_devices: u8,
    // TODO: EmberPerDeviceDutyCycle arrayOfDeviceDutyCycles - what's that?
    // <https://community.silabs.com/s/question/0D5Vm00000OKBVFKA5/ambiguities-in-ezsp-protocol-definition>
}

impl Handler {
    /// The channel page whose duty cycle state has changed.
    #[must_use]
    pub const fn channel_page(&self) -> u8 {
        self.channel_page
    }

    /// The channel number whose duty cycle state has changed.
    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }

    /// The current duty cycle state.
    ///
    /// # Errors
    ///
    /// Returns an error if the state is invalid.
    pub fn state(&self) -> Result<State, u8> {
        State::from_u8(self.state).ok_or(self.state)
    }

    /// The total number of connected end devices that are being monitored for duty cycle.
    #[must_use]
    pub const fn total_devices(&self) -> u8 {
        self.total_devices
    }

    /// Consumed duty cycles of end devices that are being monitored.
    ///
    /// The first entry always be the local stack's nodeId,
    /// and thus the total aggregate duty cycle for the device.
    #[must_use]
    pub fn array_of_device_duty_cycles(&self) -> &[u8] {
        todo!("https://community.silabs.com/s/question/0D5Vm00000OKBVFKA5/ambiguities-in-ezsp-protocol-definition")
    }
}

impl Identified for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
