//! Utilities Frames

pub use self::custom_frame::Response as CustomFrame;
pub use self::debug_write::Response as DebugWrite;
pub use self::delay_test::Response as DelayTest;
pub use self::echo::Response as Echo;
pub use self::get_eui64::Response as GetEui64;
pub use self::get_library_status::Response as GetLibraryStatus;
pub use self::get_mfg_token::Response as GetMfgToken;
pub use self::get_node_id::Response as GetNodeId;
pub use self::get_phy_interface_count::Response as GetPhyInterfaceCount;
pub use self::get_random_number::Response as GetRandomNumber;
pub use self::get_timer::Response as GetTimer;
pub use self::get_token::Response as GetToken;
pub use self::get_true_random_entropy_source::Response as GetTrueRandomEntropySource;
pub use self::get_xncp_info::Response as GetXncpInfo;
pub use self::invalid_command::Response as InvalidCommand;
pub use self::no_callbacks::Response as NoCallbacks;
pub use self::nop::Response as Nop;
pub use self::read_and_clear_counters::Response as ReadAndClearCounters;
pub use self::read_counters::Response as ReadCounters;
pub use self::set_mfg_token::Response as SetMfgToken;
pub use self::set_timer::Response as SetTimer;
pub use self::set_token::Response as SetToken;

pub mod callback;
pub mod custom_frame;
pub mod debug_write;
pub mod delay_test;
pub mod echo;
pub mod get_eui64;
pub mod get_library_status;
pub mod get_mfg_token;
pub mod get_node_id;
pub mod get_phy_interface_count;
pub mod get_random_number;
pub mod get_timer;
pub mod get_token;
pub mod get_true_random_entropy_source;
pub mod get_xncp_info;
pub mod handler;
pub mod invalid_command;
pub mod no_callbacks;
pub mod nop;
pub mod read_and_clear_counters;
pub mod read_counters;
pub mod set_mfg_token;
pub mod set_timer;
pub mod set_token;

crate::frame::parameters::command_enum!(
    Command,
    Callback(callback::Command),
    CustomFrame(custom_frame::Command),
    DebugWrite(debug_write::Command),
    DelayTest(delay_test::Command),
    Echo(echo::Command),
    GetEui64(get_eui64::Command),
    GetLibraryStatus(get_library_status::Command),
    GetMfgToken(get_mfg_token::Command),
    GetNodeId(get_node_id::Command),
    GetPhyInterfaceCount(get_phy_interface_count::Command),
    GetRandomNumber(get_random_number::Command),
    GetTimer(get_timer::Command),
    GetToken(get_token::Command),
    GetTrueRandomEntropySource(get_true_random_entropy_source::Command),
    GetXncpInfo(get_xncp_info::Command),
    Nop(nop::Command),
    ReadAndClearCounters(read_and_clear_counters::Command),
    ReadCounters(read_counters::Command),
    SetMfgToken(set_mfg_token::Command),
    SetTimer(set_timer::Command),
    SetToken(set_token::Command),
);

crate::frame::parameters::parameter_enum!(
    Response,
    CustomFrame,
    DebugWrite,
    DelayTest,
    Echo,
    GetEui64,
    GetLibraryStatus,
    GetMfgToken,
    GetNodeId,
    GetPhyInterfaceCount,
    GetRandomNumber,
    GetTimer,
    GetToken,
    GetTrueRandomEntropySource,
    GetXncpInfo,
    InvalidCommand,
    NoCallbacks,
    Nop,
    ReadAndClearCounters,
    ReadCounters,
    SetMfgToken,
    SetTimer,
    SetToken
);
