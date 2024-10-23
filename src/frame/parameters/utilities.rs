//! Utilities Frames

pub(crate) mod callback;
pub(crate) mod custom_frame;
pub(crate) mod debug_write;
pub(crate) mod delay_test;
pub(crate) mod echo;
pub(crate) mod get_eui64;
pub(crate) mod get_library_status;
pub(crate) mod get_mfg_token;
pub(crate) mod get_node_id;
pub(crate) mod get_phy_interface_count;
pub(crate) mod get_random_number;
pub mod get_timer;
pub(crate) mod get_token;
pub(crate) mod get_true_random_entropy_source;
pub(crate) mod get_xncp_info;
pub mod handler;
pub mod invalid_command;
pub(crate) mod no_callbacks;
pub(crate) mod nop;
pub(crate) mod read_and_clear_counters;
pub(crate) mod read_counters;
pub(crate) mod set_mfg_token;
pub(crate) mod set_timer;
pub(crate) mod set_token;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    CustomFrame(custom_frame::Response),
    DebugWrite(debug_write::Response),
    DelayTest(delay_test::Response),
    Echo(echo::Response),
    GetEui64(get_eui64::Response),
    GetLibraryStatus(get_library_status::Response),
    GetMfgToken(get_mfg_token::Response),
    GetNodeId(get_node_id::Response),
    GetPhyInterfaceCount(get_phy_interface_count::Response),
    GetRandomNumber(get_random_number::Response),
    GetTimer(get_timer::Response),
    GetToken(get_token::Response),
    GetTrueRandomEntropySource(get_true_random_entropy_source::Response),
    GetXncpInfo(get_xncp_info::Response),
    InvalidCommand(invalid_command::Response),
    NoCallbacks(no_callbacks::Response),
    Nop(nop::Response),
    ReadAndClearCounters(read_and_clear_counters::Response),
    ReadCounters(read_counters::Response),
    SetMfgToken(set_mfg_token::Response),
    SetTimer(set_timer::Response),
    SetToken(set_token::Response),
    Handler(handler::Handler),
}
