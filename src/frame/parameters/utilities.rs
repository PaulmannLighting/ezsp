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
