use crate::frame::enums::Parameters;
use crate::frame::parameters::binding::handler::Handler as Binding;
use crate::frame::parameters::bootloader::handler::Handler as Bootloader;
use crate::frame::parameters::cbke::handler::Handler as Cbke;
use crate::frame::parameters::green_power::handler::Handler as GreenPower;
use crate::frame::parameters::messaging::handler::Handler as Messaging;
use crate::frame::parameters::mfglib::handler::Handler as MfgLib;
use crate::frame::parameters::networking::handler::Handler as Networking;
use crate::frame::parameters::security::handler::Handler as Security;
use crate::frame::parameters::trust_center::handler::Handler as TrustCenter;
use crate::frame::parameters::utilities::handler::Handler as Utilities;
use crate::frame::parameters::zll::handler::Handler as Zll;

crate::frame::parameters::parameter_group_enum!(
    Callback,
    Binding,
    Bootloader,
    Cbke,
    GreenPower,
    Messaging,
    MfgLib,
    Networking,
    Security,
    TrustCenter,
    Utilities,
    Zll,
    impl {
        impl TryFrom<Parameters> for Callback {
            type Error = Parameters;

            fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
                match parameters {
                    Parameters::Callback(callback) => Ok(callback),
                    Parameters::Response(_) => Err(parameters),
                }
            }
        }
    },
);
