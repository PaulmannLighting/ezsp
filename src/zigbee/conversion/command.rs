use zigbee::Profile;
use zigbee_nwk::Command;

use super::parse_command_error::ParseCommandError;
use crate::defragmentation::DefragmentedMessage;

impl TryFrom<DefragmentedMessage> for Command {
    type Error = ParseCommandError;

    fn try_from(frame: DefragmentedMessage) -> Result<Self, Self::Error> {
        let aps_frame = frame.aps_frame();
        let profile =
            Profile::try_from(aps_frame.profile_id()).map_err(ParseCommandError::InvalidProfile)?;

        match profile {
            Profile::Network => zdp::Frame::<zdp::Command>::try_from(frame)
                .map(Command::Zdp)
                .map_err(ParseCommandError::ParseZdpFrameError),
            Profile::ZigbeeHomeAutomation
            | Profile::SmartEnergy
            | Profile::TouchLink
            | Profile::BuildingAutomation
            | Profile::HealthCare
            | Profile::RemoteControl => zcl::Frame::<zcl::Cluster>::try_from(frame)
                .map(Command::Zcl)
                .map_err(ParseCommandError::ParseZclFrameError),
        }
    }
}
