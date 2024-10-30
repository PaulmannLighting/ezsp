//! Utilities Frames

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

/// Response types of the utilities commands.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    /// Response to the `custom_frame` command.
    CustomFrame(custom_frame::Response),
    /// Response to the `debug_write` command.
    DebugWrite(debug_write::Response),
    /// Response to the `delay_test` command.
    DelayTest(delay_test::Response),
    /// Response to the `echo` command.
    Echo(echo::Response),
    /// Response to the `get_eui64` command.
    GetEui64(get_eui64::Response),
    /// Response to the `get_library_status` command.
    GetLibraryStatus(get_library_status::Response),
    /// Response to the `get_mfg_token` command.
    GetMfgToken(get_mfg_token::Response),
    /// Response to the `get_node_id` command.
    GetNodeId(get_node_id::Response),
    /// Response to the `get_phy_interface_count` command.
    GetPhyInterfaceCount(get_phy_interface_count::Response),
    /// Response to the `get_random_number` command.
    GetRandomNumber(get_random_number::Response),
    /// Response to the `get_timer` command.
    GetTimer(get_timer::Response),
    /// Response to the `get_token` command.
    GetToken(get_token::Response),
    /// Response to the `get_true_random_entropy_source` command.
    GetTrueRandomEntropySource(get_true_random_entropy_source::Response),
    /// Response to the `get_xncp_info` command.
    GetXncpInfo(get_xncp_info::Response),
    /// Response to commands if the command was not recognized.
    InvalidCommand(invalid_command::Response),
    /// Response to the `no_callbacks` command.
    NoCallbacks(no_callbacks::Response),
    /// Response to the `nop` command.
    Nop(nop::Response),
    /// Response to the `read_and_clear_counters` command.
    ReadAndClearCounters(read_and_clear_counters::Response),
    /// Response to the `read_counters` command.
    ReadCounters(read_counters::Response),
    /// Response to the `set_mfg_token` command.
    SetMfgToken(set_mfg_token::Response),
    /// Response to the `set_timer` command.
    SetTimer(set_timer::Response),
    /// Response to the `set_token` command.
    SetToken(set_token::Response),
}
