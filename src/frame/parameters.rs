//! Definitions of parameter structures used in the `Ember ZNet Serial Protocol` (`EZSP`).

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

macro_rules! parameter {
    (
        $name:ident,
        $id:expr,
        { $($field:ident: $ty:ty),* $(,)? }
        $(, impl { $($impls:item)* })?
    ) => {
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

macro_rules! command_enum {
    ($name:ident, $($variant:ident($ty:path)),+ $(,)?) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        #[doc = concat!(stringify!($name), " parameters.")]
        pub enum $name {
            $(
                #[doc = concat!(stringify!($variant), " command parameters.")]
                $variant(Box<$ty>)
            ),+
        }
    };
}
pub(crate) use command_enum;

macro_rules! command {
    (
        $id:expr,
        { $($field:ident: $ty:ty),* $(,)? },
        $response:ty =>
            $group:ident($module:ident)::$nested:ident($nested_module:ident)::$variant:ident
        $(, impl { $($impls:item)* })?
        $(,)?) => {
        crate::frame::parameters::command!(
            @emit
            $id,
            { $($field: $ty),* },
            $response
            $(, impl { $($impls)* })?
        );

        impl From<Command> for crate::frame::enums::Command {
            fn from(command: Command) -> Self {
                Self::$group(Box::new(
                    crate::frame::parameters::$module::Command::$nested(Box::new(
                        crate::frame::parameters::$module::$nested_module::Command::$variant(
                            Box::new(command),
                        ),
                    )),
                ))
            }
        }
    };
    (
        $id:expr,
        { $($field:ident: $ty:ty),* $(,)? },
        $response:ty => $group:ident($module:ident)::$variant:ident
        $(, impl { $($impls:item)* })?
        $(,)?) => {
        crate::frame::parameters::command!(
            @emit
            $id,
            { $($field: $ty),* },
            $response
            $(, impl { $($impls)* })?
        );

        impl From<Command> for crate::frame::enums::Command {
            fn from(command: Command) -> Self {
                Self::$group(Box::new(
                    crate::frame::parameters::$module::Command::$variant(Box::new(command)),
                ))
            }
        }
    };
    (@emit
        $id:expr,
        { $($field:ident: $ty:ty),* $(,)? },
        $response:ty
        $(, impl { $($impls:item)* })?
        $(,)?) => {
        crate::frame::parameters::parameter!(
            Command,
            $id,
            { $($field: $ty),* }
            $(, impl { $($impls)* })?
        );

        impl crate::frame::RespondsWith for Command {
            type Response = $response;
        }
    };
}
pub(crate) use command;

macro_rules! response {
    (
        $id:expr,
        { $($field:ident: $ty:ty),* $(,)? } =>
            $group:ident($module:ident)::$nested:ident($nested_module:ident)::$variant:ident
        $(, impl { $($impls:item)* })?
        $(,)?
    ) => {
        crate::frame::parameters::parameter!(
            Response,
            $id,
            { $($field: $ty),* }
            $(, impl { $($impls)* })?
        );
        crate::frame::parameters::response!(
            @try_from $group $module $nested $nested_module $variant
        );
    };
    (
        $id:expr,
        { $($field:ident: $ty:ty),* $(,)? } =>
            $group:ident($module:ident)::$variant:ident
        $(, impl { $($impls:item)* })?
        $(,)?
    ) => {
        crate::frame::parameters::parameter!(
            Response,
            $id,
            { $($field: $ty),* }
            $(, impl { $($impls)* })?
        );
        crate::frame::parameters::response!(@try_from $group $module $variant);
    };
    (
        $id:expr,
        { $($field:ident: $ty:ty),* $(,)? }
        $(, impl { $($impls:item)* })?
        $(,)?
    ) => {
        crate::frame::parameters::parameter!(
            Response,
            $id,
            { $($field: $ty),* }
            $(, impl { $($impls)* })?
        );
    };
    (@try_from $group:ident $module:ident $variant:ident) => {
        impl TryFrom<crate::frame::enums::Parameters> for Response {
            type Error = crate::frame::enums::Parameters;

            fn try_from(parameters: crate::frame::enums::Parameters) -> Result<Self, Self::Error> {
                match parameters {
                    crate::frame::enums::Parameters::Response(
                        crate::frame::enums::Response::$group(
                            crate::frame::parameters::$module::Response::$variant(response),
                        ),
                    ) => Ok(*response),
                    _ => Err(parameters),
                }
            }
        }
    };
    (@try_from $group:ident $module:ident $nested:ident $nested_module:ident $variant:ident) => {
        impl TryFrom<crate::frame::enums::Parameters> for Response {
            type Error = crate::frame::enums::Parameters;

            fn try_from(parameters: crate::frame::enums::Parameters) -> Result<Self, Self::Error> {
                match parameters {
                    crate::frame::enums::Parameters::Response(
                        crate::frame::enums::Response::$group(
                            crate::frame::parameters::$module::Response::$nested(response),
                        ),
                    ) => match *response {
                        crate::frame::parameters::$module::$nested_module::Response::$variant(
                            response,
                        ) => Ok(*response),
                        response => Err(crate::frame::enums::Parameters::Response(
                            crate::frame::enums::Response::$group(
                                crate::frame::parameters::$module::Response::$nested(Box::new(
                                    response,
                                )),
                            ),
                        )),
                    },
                    _ => Err(parameters),
                }
            }
        }
    };
}
pub(crate) use response;

macro_rules! handler {
    (
        $id:expr,
        { $($field:ident: $ty:ty),* $(,)? }
        $(, impl { $($impls:item)* })?
        $(,)?
    ) => {
        crate::frame::parameters::parameter!(
            Handler,
            $id,
            { $($field: $ty),* }
            $(, impl { $($impls)* })?
        );
    };
}
pub(crate) use handler;

macro_rules! frame {
    (
        $id:expr,
        { $($command_field:ident: $command_ty:ty),* $(,)? }
        $(, impl { $($command_impls:item)* })?,
        { $($response_field:ident: $response_ty:ty),* $(,)? } => $group:ident($module:ident)::$nested:ident($nested_module:ident)::$variant:ident
        $(, impl { $($response_impls:item)* })?
        $(,)?
    ) => {
        crate::frame::parameters::command!(
            $id,
            { $($command_field: $command_ty),* },
            Response => $group($module)::$nested($nested_module)::$variant
            $(, impl { $($command_impls)* })?
        );

        crate::frame::parameters::response!(
            $id,
            { $($response_field: $response_ty),* } => $group($module)::$nested($nested_module)::$variant
            $(, impl { $($response_impls)* })?
        );
    };
    (
        $id:expr,
        { $($command_field:ident: $command_ty:ty),* $(,)? }
        $(, impl { $($command_impls:item)* })?,
        { $($response_field:ident: $response_ty:ty),* $(,)? } => $group:ident($module:ident)::$variant:ident
        $(, impl { $($response_impls:item)* })?
        $(,)?
    ) => {
        crate::frame::parameters::command!(
            $id,
            { $($command_field: $command_ty),* },
            Response => $group($module)::$variant
            $(, impl { $($command_impls)* })?
        );

        crate::frame::parameters::response!(
            $id,
            { $($response_field: $response_ty),* } => $group($module)::$variant
            $(, impl { $($response_impls)* })?
        );
    };
}
pub(crate) use frame;

#[allow(unused_macros)]
macro_rules! parameter_enum {
    ($name:ident, $($ty:ident),+ $(,)?) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        #[doc = concat!(stringify!($name), " parameters.")]
        pub enum $name {
            $(
                #[doc = concat!(stringify!($ty), " parameters.")]
                $ty(Box<$ty>)
            ),+
        }

        impl crate::frame::Parsable for $name {
            fn parse_from_le_stream<T>(id: u16, stream: T) -> Result<Self, crate::error::Decode>
            where
                T: Iterator<Item = u8>,
            {
                let bytes: Vec<u8> = stream.collect();

                $(
                    match <$ty as crate::frame::Parsable>::parse_from_le_stream(
                        id,
                        bytes.iter().copied(),
                    ) {
                        Ok(parameters) => return Ok(Self::$ty(Box::new(parameters))),
                        Err(
                            crate::error::Decode::FrameIdMismatch { .. }
                            | crate::error::Decode::InvalidFrameId(_),
                        ) => {}
                        Err(error) => return Err(error),
                    }
                )+

                Err(crate::error::Decode::InvalidFrameId(id))
            }
        }
    };
}
pub(crate) use parameter_enum;

macro_rules! parameter_group_enum {
    ($name:ident, $($tokens:tt)*) => {
        crate::frame::parameters::parameter_group_enum!(@collect [$name] [] $($tokens)*);
    };
    (@collect [$name:ident] [$($variant:ident),*] impl { $($impls:item)* } $(,)?) => {
        crate::frame::parameters::parameter_group_enum!(
            @emit [$name] [$($variant),*] [$($impls)*]
        );
    };
    (@collect [$name:ident] [$($variant:ident),*] $next:ident, $($rest:tt)+) => {
        crate::frame::parameters::parameter_group_enum!(
            @collect [$name] [$($variant,)* $next] $($rest)+
        );
    };
    (@collect [$name:ident] [$($variant:ident),*] $last:ident $(,)?) => {
        crate::frame::parameters::parameter_group_enum!(
            @emit [$name] [$($variant,)* $last] []
        );
    };
    (@emit [$name:ident] [$($variant:ident),+] [$($impls:item)*]) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        #[doc = concat!(stringify!($name), " parameters grouped by parameter namespace.")]
        pub enum $name {
            $(
                #[doc = concat!(stringify!($variant), " parameters.")]
                $variant($variant)
            ),+
        }

        impl crate::frame::Parsable for $name {
            fn parse_from_le_stream<T>(id: u16, stream: T) -> Result<Self, crate::error::Decode>
            where
                T: Iterator<Item = u8>,
            {
                let bytes: Vec<u8> = stream.collect();

                $(
                    match <$variant as crate::frame::Parsable>::parse_from_le_stream(
                        id,
                        bytes.iter().copied(),
                    ) {
                        Ok(parameters) => return Ok(Self::$variant(parameters)),
                        Err(
                            crate::error::Decode::FrameIdMismatch { .. }
                            | crate::error::Decode::InvalidFrameId(_),
                        ) => {}
                        Err(error) => return Err(error),
                    }
                )+

                Err(crate::error::Decode::InvalidFrameId(id))
            }
        }

        $($impls)*
    };
}
pub(crate) use parameter_group_enum;
