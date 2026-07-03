//! Definitions of parameter structures used in the `Ember ZNet Serial Protocol` (`EZSP`).

macro_rules! frame {
    (
        $id:expr,
        { $($command_field:ident: $command_ty:ty),* $(,)? },
        { $($response_field:ident: $response_ty:ty),* $(,)? }
    ) => {
        const ID: u16 = $id;

        frame! {
            @command { $($command_field: $command_ty),* }
        }

        impl crate::frame::Parameter for Command {
            const ID: u16 = ID;
        }

        impl crate::frame::RespondsWith for Command {
            type Response = Response;
        }

        frame! {
            @response { $($response_field: $response_ty),* }
        }

        impl crate::frame::Parameter for Response {
            const ID: u16 = ID;
        }
    };
    (@command {}) => {
        #[derive(Clone, Debug, Eq, PartialEq, le_stream::ToLeStream)]
        pub(crate) struct Command;
    };
    (@command { $($field:ident: $ty:ty),+ }) => {
        #[derive(Clone, Debug, Eq, PartialEq, le_stream::ToLeStream)]
        pub(crate) struct Command {
            $($field: $ty),+
        }
    };
    (@response {}) => {
        /// Response parameters.
        #[derive(Clone, Debug, Eq, PartialEq, le_stream::FromLeStream)]
        pub struct Response;
    };
    (@response { $($field:ident: $ty:ty),+ }) => {
        /// Response parameters.
        #[derive(Clone, Debug, Eq, PartialEq, le_stream::FromLeStream)]
        pub struct Response {
            $($field: $ty),+
        }
    };
}
pub(crate) use frame;

pub mod binding;
pub mod bootloader;
pub mod cbke;
pub mod configuration;
pub mod green_power;
pub mod messaging;
pub mod mfglib;
pub mod networking;
pub mod security;
pub mod token_interface;
pub mod trust_center;
pub mod utilities;
pub mod wwah;
pub mod zll;
