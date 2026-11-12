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

// Payload fields returned as raw byte arrays/vectors can contain keys or tokens.
// Redact at the parameter boundary so nested Debug output and error chains are safe.
macro_rules! debug_field {
    ($builder:ident, $value:ident, key) => {
        $builder.field("key", &"[REDACTED]");
    };
    ($builder:ident, $value:ident, plaintext_key) => {
        $builder.field("plaintext_key", &"[REDACTED]");
    };
    ($builder:ident, $value:ident, token_data) => {
        $builder.field("token_data", &"[REDACTED]");
    };
    ($builder:ident, $value:ident, value) => {
        $builder.field("value", &"[REDACTED]");
    };
    ($builder:ident, $value:ident, $field:ident) => {
        $builder.field(stringify!($field), &$value.$field);
    };
}
pub(crate) use debug_field;

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
            Eq,
            PartialEq,
            le_stream::FromLeStream,
            le_stream::ToLeStream
        )]
        pub struct $name {
            $($field: $ty),+
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let mut builder = formatter.debug_struct(stringify!($name));
                $(crate::frame::parameters::debug_field!(builder, self, $field);)+
                builder.finish()
            }
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

        impl $name {
            /// Returns the frame parameter ID for this command.
            #[must_use]
            pub const fn id(&self) -> u16 {
                match self {
                    $(
                        Self::$variant(command) => command.id()
                    ),+
                }
            }
        }

        impl le_stream::ToLeStream for $name {
            type Iter = Box<dyn Iterator<Item = u8>>;

            fn to_le_stream(self) -> Self::Iter {
                match self {
                    $(
                        Self::$variant(command) => Box::new(
                            le_stream::ToLeStream::to_le_stream(*command)
                        )
                    ),+
                }
            }
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

        impl Command {
            /// Returns the frame parameter ID for this command.
            #[must_use]
            pub const fn id(&self) -> u16 {
                <Self as crate::frame::Parameter>::ID
            }
        }

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

#[cfg(test)]
mod secret_debug_tests {
    use std::iter::once;

    use le_stream::FromLeStream;
    use silizium::zigbee::security::man::{Context, DerivedKeyType, Flags, KeyType};

    use crate::ember::Eui64;
    use crate::ember::security::initial::{Bitmask, State};
    use crate::parameters::{configuration, security, utilities};
    use crate::{Error, Parameters, Parsable};

    const SECRET: [u8; 16] = [0xAB; 16];
    const REVEALED_PREFIX: &str = "[171, 171";

    #[test]
    fn nested_export_responses_and_errors_redact_keys() {
        let response = SECRET.into_iter().chain([0, 0, 0, 0]);
        let parameters = Parameters::parse_from_le_stream(0x0114, response).unwrap();
        let error = Error::UnexpectedResponse(Box::new(parameters));
        for output in [format!("{error:?}"), format!("{error}")] {
            assert!(output.contains("[REDACTED]"));
            assert!(!output.contains(REVEALED_PREFIX));
        }
    }

    #[test]
    fn directly_exposed_link_key_payloads_redact_keys() {
        let payload = [0; 8].into_iter().chain(SECRET).chain([0; 12]);
        let payload =
            security::export_link_key_by_index::Payload::from_le_stream_exact(payload).unwrap();
        assert_eq!(payload.plaintext_key(), &SECRET);
        let output = format!("{payload:?}");
        assert!(output.contains("[REDACTED]"));
        assert!(!output.contains(REVEALED_PREFIX));
    }

    #[test]
    fn raw_values_and_manufacturing_tokens_are_redacted() {
        let response = [0, 16].into_iter().chain(SECRET);
        let response = configuration::get_value::Response::from_le_stream_exact(response).unwrap();
        assert_eq!(
            format!("{response:?}"),
            "Response { status: 0, value: \"[REDACTED]\" }"
        );
        let response = once(16).chain(SECRET);
        let response = utilities::get_mfg_token::Response::from_le_stream_exact(response).unwrap();
        assert_eq!(
            format!("{response:?}"),
            "Response { token_data: \"[REDACTED]\" }"
        );
    }

    #[test]
    fn imported_keys_and_initial_security_state_are_redacted() {
        let context = Context::new(
            KeyType::Network,
            0,
            DerivedKeyType::None,
            Eui64::default(),
            0,
            Flags::NONE,
            0,
        );
        let command = security::import_key::Command::new(context, SECRET);
        let state = State::new(
            Bitmask::HAVE_NETWORK_KEY,
            SECRET,
            SECRET,
            0,
            Eui64::default(),
        );
        for output in [
            format!("{command:?}"),
            format!("{state:?}"),
            format!(
                "{:?}",
                security::set_initial_security_state::Command::new(state)
            ),
        ] {
            assert!(output.contains("[REDACTED]"));
            assert!(!output.contains(REVEALED_PREFIX));
        }
    }

    #[test]
    fn ordinary_frame_metadata_remains_visible() {
        let command = configuration::version::Command::new(13);
        assert_eq!(
            format!("{command:?}"),
            "Command { desired_protocol_version: 13 }"
        );
    }
}
