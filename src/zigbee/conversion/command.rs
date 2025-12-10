use zigbee::Profile;
use zigbee_nwk::Command;

use super::parse_command_error::ParseCommandError;
use crate::parameters::messaging::handler::IncomingMessage;

impl TryFrom<IncomingMessage> for Command {
    type Error = ParseCommandError;

    fn try_from(frame: IncomingMessage) -> Result<Self, Self::Error> {
        let aps_frame = frame.aps_frame();
        let profile =
            Profile::try_from(aps_frame.profile_id()).map_err(ParseCommandError::InvalidProfile)?;

        match profile {
            Profile::Unspecified => zdp::Frame::<zdp::Command>::try_from(frame)
                .map(Command::Zdp)
                .map_err(ParseCommandError::ParseZdpFrameError),
            Profile::ZigbeeHomeAutomation | Profile::SmartEnergy | Profile::Touchlink => {
                zcl::Frame::<zcl::Cluster>::try_from(frame)
                    .map(Command::Zcl)
                    .map_err(ParseCommandError::ParseZclFrameError)
            }
        }
    }
}
