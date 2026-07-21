//! Enumeration containing all possible `EZSP` command parameters.

crate::frame::parameters::command_enum!(
    Command,
    Binding(crate::frame::parameters::binding::Command),
    Bootloader(crate::frame::parameters::bootloader::Command),
    Cbke(crate::frame::parameters::cbke::Command),
    Configuration(crate::frame::parameters::configuration::Command),
    GreenPower(crate::frame::parameters::green_power::Command),
    Messaging(crate::frame::parameters::messaging::Command),
    MfgLib(crate::frame::parameters::mfglib::Command),
    Networking(crate::frame::parameters::networking::Command),
    Security(crate::frame::parameters::security::Command),
    TokenInterface(crate::frame::parameters::token_interface::Command),
    TrustCenter(crate::frame::parameters::trust_center::Command),
    Utilities(crate::frame::parameters::utilities::Command),
    Wwah(crate::frame::parameters::wwah::Command),
    Zll(crate::frame::parameters::zll::Command),
);

#[cfg(test)]
mod tests {
    use le_stream::ToLeStream;

    use crate::frame::Parameter;
    use crate::frame::enums::Command;
    use crate::frame::parameters::{configuration, green_power, utilities};

    const DESIRED_PROTOCOL_VERSION: u8 = 13;
    const SINK_INDEX: u8 = 7;

    #[test]
    fn converts_concrete_commands() {
        assert!(matches!(
            Command::from(configuration::version::Command::new(
                DESIRED_PROTOCOL_VERSION
            )),
            Command::Configuration(command)
                if matches!(*command, configuration::Command::Version(_))
        ));
        assert!(matches!(
            Command::from(green_power::sink_table::clear_all::Command),
            Command::GreenPower(command)
                if matches!(
                    command.as_ref(),
                    green_power::Command::SinkTable(command)
                        if matches!(
                            command.as_ref(),
                            green_power::sink_table::Command::ClearAll(_)
                        )
                )
        ));
        assert!(matches!(
            Command::from(utilities::callback::Command),
            Command::Utilities(command)
                if matches!(*command, utilities::Command::Callback(_))
        ));
    }

    #[test]
    fn returns_concrete_command_ids() {
        let command = Command::from(configuration::version::Command::new(
            DESIRED_PROTOCOL_VERSION,
        ));
        assert_eq!(command.id(), configuration::version::Command::ID);

        let command = Command::from(green_power::sink_table::remove_entry::Command::new(
            SINK_INDEX,
        ));
        assert_eq!(
            command.id(),
            green_power::sink_table::remove_entry::Command::ID
        );

        let command = Command::from(utilities::callback::Command);
        assert_eq!(command.id(), utilities::callback::Command::ID);
    }

    #[test]
    fn serializes_concrete_commands() {
        let command = Command::from(configuration::version::Command::new(
            DESIRED_PROTOCOL_VERSION,
        ));
        assert_eq!(
            command.to_le_stream().collect::<Vec<_>>(),
            [DESIRED_PROTOCOL_VERSION]
        );

        let command = Command::from(green_power::sink_table::remove_entry::Command::new(
            SINK_INDEX,
        ));
        assert_eq!(command.to_le_stream().collect::<Vec<_>>(), [SINK_INDEX]);
    }
}
