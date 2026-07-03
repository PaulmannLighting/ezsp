//! Definitions of parameter structures used in the `Ember ZNet Serial Protocol` (`EZSP`).

macro_rules! parameter {
    ($name:ident, $id:expr, {}) => {
        #[doc = concat!(stringify!($name), " parameters.")]
        #[derive(
            Clone,
            Copy,
            Debug,
            Eq,
            PartialEq,
            le_stream::FromLeStream,
            le_stream::ToLeStream
        )]
        pub struct $name;

        impl crate::frame::Parameter for $name {
            const ID: u16 = $id;
        }
    };
    ($name:ident, $id:expr, { $($field:ident: $ty:ty),+ $(,)? }) => {
        #[doc = concat!(stringify!($name), " parameters.")]
        #[derive(
            Clone,
            Debug,
            Eq,
            PartialEq,
            le_stream::FromLeStream,
            le_stream::ToLeStream
        )]
        pub struct $name {
            $($field: $ty),+
        }

        impl crate::frame::Parameter for $name {
            const ID: u16 = $id;
        }
    };
}
pub(crate) use parameter;

macro_rules! command {
    ($id:expr, { $($field:ident: $ty:ty),* $(,)? }, $response:ty $(,)?) => {
        crate::frame::parameters::parameter!(Command, $id, { $($field: $ty),* });

        impl crate::frame::RespondsWith for Command {
            type Response = $response;
        }
    };
}
pub(crate) use command;

macro_rules! response {
    ($id:expr, { $($field:ident: $ty:ty),* $(,)? }) => {
        crate::frame::parameters::parameter!(Response, $id, { $($field: $ty),* });
    };
}
pub(crate) use response;

macro_rules! handler {
    ($id:expr, {}) => {
        /// Handler parameters.
        #[derive(Clone, Copy, Debug, Eq, PartialEq, le_stream::FromLeStream)]
        pub struct Handler;

        impl crate::frame::Parameter for Handler {
            const ID: u16 = $id;
        }
    };
    ($id:expr, { $($field:ident: $ty:ty),+ $(,)? }) => {
        /// Handler parameters.
        #[derive(Clone, Debug, Eq, PartialEq, le_stream::FromLeStream)]
        pub struct Handler {
            $($field: $ty),+
        }

        impl crate::frame::Parameter for Handler {
            const ID: u16 = $id;
        }
    };
}
pub(crate) use handler;

macro_rules! frame {
    (
        $id:expr,
        { $($command_field:ident: $command_ty:ty),* $(,)? },
        { $($response_field:ident: $response_ty:ty),* $(,)? }
    ) => {
        crate::frame::parameters::command!($id, { $($command_field: $command_ty),* }, Response);

        crate::frame::parameters::response!($id, { $($response_field: $response_ty),* });
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
