//! Enumeration containing all possible `EZSP` response parameters.

use crate::frame::parameters::binding::Response as Binding;
use crate::frame::parameters::bootloader::Response as Bootloader;
use crate::frame::parameters::cbke::Response as Cbke;
use crate::frame::parameters::configuration::Response as Configuration;
use crate::frame::parameters::green_power::Response as GreenPower;
use crate::frame::parameters::messaging::Response as Messaging;
use crate::frame::parameters::mfglib::Response as MfgLib;
use crate::frame::parameters::networking::Response as Networking;
use crate::frame::parameters::security::Response as Security;
use crate::frame::parameters::token_interface::Response as TokenInterface;
use crate::frame::parameters::trust_center::Response as TrustCenter;
use crate::frame::parameters::utilities::Response as Utilities;
use crate::frame::parameters::wwah::Response as Wwah;
use crate::frame::parameters::zll::Response as Zll;

crate::frame::parameters::parameter_group_enum!(
    Response,
    Binding,
    Bootloader,
    Cbke,
    Configuration,
    GreenPower,
    Messaging,
    MfgLib,
    Networking,
    Security,
    TokenInterface,
    TrustCenter,
    Utilities,
    Wwah,
    Zll,
);
