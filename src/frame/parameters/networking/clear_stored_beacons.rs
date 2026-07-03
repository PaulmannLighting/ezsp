//! Parameters for the [`Networking::clear_stored_beacons`](crate::Networking::clear_stored_beacons) command.

crate::frame::parameters::frame!(
    0x003C,
    {},
    {} => Networking(networking)::ClearStoredBeacons,
);
