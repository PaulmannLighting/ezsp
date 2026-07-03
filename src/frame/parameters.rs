//! Definitions of parameter structures used in the `Ember ZNet Serial Protocol` (`EZSP`).

macro_rules! parameter {
    ($name:ident, $id:expr, { $($field:ident: $ty:ty),* $(,)? } $(, impl { $($impls:item)* })?) => {
        crate::frame::parameters::parameter!(@struct $name, { $($field: $ty),* });

        impl crate::frame::Parameter for $name {
            const ID: u16 = $id;
        }

        $($($impls)*)?
    };
    (@struct $name:ident, {}) => {
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
    };
    (@struct $name:ident, { $($field:ident: $ty:ty),+ }) => {
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
    };
}
pub(crate) use parameter;

macro_rules! command {
    ($id:expr, { $($field:ident: $ty:ty),* $(,)? }, $response:ty $(, impl { $($impls:item)* })? $(,)?) => {
        crate::frame::parameters::parameter!(Command, $id, { $($field: $ty),* } $(, impl { $($impls)* })?);

        impl crate::frame::RespondsWith for Command {
            type Response = $response;
        }
    };
}
pub(crate) use command;

macro_rules! response {
    ($id:expr, { $($field:ident: $ty:ty),* $(,)? } $(, impl { $($impls:item)* })? $(,)?) => {
        crate::frame::parameters::parameter!(Response, $id, { $($field: $ty),* } $(, impl { $($impls)* })?);
    };
}
pub(crate) use response;

macro_rules! handler {
    ($id:expr, { $($field:ident: $ty:ty),* $(,)? } $(, impl { $($impls:item)* })? $(,)?) => {
        crate::frame::parameters::parameter!(Handler, $id, { $($field: $ty),* } $(, impl { $($impls)* })?);
    };
}
pub(crate) use handler;

macro_rules! frame {
    (
        $id:expr,
        { $($command_field:ident: $command_ty:ty),* $(,)? }
        $(, impl { $($command_impls:item)* })?,
        { $($response_field:ident: $response_ty:ty),* $(,)? }
        $(, impl { $($response_impls:item)* })?
        $(,)?
    ) => {
        crate::frame::parameters::command!($id, { $($command_field: $command_ty),* }, Response $(, impl { $($command_impls)* })?);

        crate::frame::parameters::response!($id, { $($response_field: $response_ty),* } $(, impl { $($response_impls)* })?);
    };
}
pub(crate) use frame;

#[allow(unused_macros)]
macro_rules! ids {
    ($name:ident, $($ty:ty),+ $(,)?) => {
        macro_rules! $name {
            () => {
                $(<$ty as crate::frame::Parameter>::ID)|+
            };
        }
    };
}
#[allow(unused_imports)]
pub(crate) use ids;

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
