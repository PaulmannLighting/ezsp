use crate::read_write::Writable;
use std::io::Write;

pub mod callback;
pub mod counter_rollover_handler;
pub mod custom_frame;
pub mod custom_frame_handler;
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
pub mod invalid_command;
pub mod no_callbacks;
pub mod nop;
pub mod read_and_clear_counters;
pub mod read_counters;
pub mod set_mfg_token;
pub mod set_timer;
pub mod set_token;
pub mod stack_token_changed_handler;
pub mod timer_handler;

#[derive(Debug, Eq, PartialEq)]
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

impl Command {
    #[must_use]
    pub const fn id(&self) -> u16 {
        match self {
            Self::Callback(_) => callback::ID,
            Self::CustomFrame(_) => custom_frame::ID,
            Self::DebugWrite(_) => debug_write::ID,
            Self::DelayTest(_) => delay_test::ID,
            Self::Echo(_) => echo::ID,
            Self::GetEui64(_) => get_eui64::ID,
            Self::GetLibraryStatus(_) => get_library_status::ID,
            Self::GetMfgToken(_) => get_mfg_token::ID,
            Self::GetNodeId(_) => get_node_id::ID,
            Self::GetPhyInterfaceCount(_) => get_phy_interface_count::ID,
            Self::GetRandomNumber(_) => get_random_number::ID,
            Self::GetTimer(_) => get_timer::ID,
            Self::GetToken(_) => get_token::ID,
            Self::GetTrueRandomEntropySource(_) => get_true_random_entropy_source::ID,
            Self::GetXncpInfo(_) => get_xncp_info::ID,
            Self::Nop(_) => nop::ID,
            Self::ReadAndClearCounters(_) => read_and_clear_counters::ID,
            Self::ReadCounters(_) => read_counters::ID,
            Self::SetMfgToken(_) => set_mfg_token::ID,
            Self::SetTimer(_) => set_timer::ID,
            Self::SetToken(_) => set_token::ID,
        }
    }
}

impl Writable for Command {
    fn write_to<W>(self, dst: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        match self {
            Self::Callback(callback) => callback.write_to(dst),
            Self::CustomFrame(custom_frame) => custom_frame.write_to(dst),
            Self::DebugWrite(debug_write) => debug_write.write_to(dst),
            Self::DelayTest(delay_test) => delay_test.write_to(dst),
            Self::Echo(echo) => echo.write_to(dst),
            Self::GetEui64(get_eui64) => get_eui64.write_to(dst),
            Self::GetLibraryStatus(get_library_status) => get_library_status.write_to(dst),
            Self::GetMfgToken(get_mfg_token) => get_mfg_token.write_to(dst),
            Self::GetNodeId(get_node_id) => get_node_id.write_to(dst),
            Self::GetPhyInterfaceCount(get_phy_interface_count) => {
                get_phy_interface_count.write_to(dst)
            }
            Self::GetRandomNumber(get_random_number) => get_random_number.write_to(dst),
            Self::GetTimer(get_timer) => get_timer.write_to(dst),
            Self::GetToken(get_token) => get_token.write_to(dst),
            Self::GetTrueRandomEntropySource(get_true_random_entropy_source) => {
                get_true_random_entropy_source.write_to(dst)
            }
            Self::GetXncpInfo(get_xncp_info) => get_xncp_info.write_to(dst),
            Self::Nop(nop) => nop.write_to(dst),
            Self::ReadAndClearCounters(read_and_clear_counters) => {
                read_and_clear_counters.write_to(dst)
            }
            Self::ReadCounters(read_counters) => read_counters.write_to(dst),
            Self::SetMfgToken(set_mfg_token) => set_mfg_token.write_to(dst),
            Self::SetTimer(set_timer) => set_timer.write_to(dst),
            Self::SetToken(set_token) => set_token.write_to(dst),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
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
    Nop(nop::Response),
    ReadAndClearCounters(read_and_clear_counters::Response),
    ReadCounters(read_counters::Response),
    SetMfgToken(set_mfg_token::Response),
    SetTimer(set_timer::Response),
    SetToken(set_token::Response),
}

impl Response {
    #[must_use]
    pub const fn id(&self) -> u16 {
        match self {
            Self::CustomFrame(_) => custom_frame::ID,
            Self::DebugWrite(_) => debug_write::ID,
            Self::DelayTest(_) => delay_test::ID,
            Self::Echo(_) => echo::ID,
            Self::GetEui64(_) => get_eui64::ID,
            Self::GetLibraryStatus(_) => get_library_status::ID,
            Self::GetMfgToken(_) => get_mfg_token::ID,
            Self::GetNodeId(_) => get_node_id::ID,
            Self::GetPhyInterfaceCount(_) => get_phy_interface_count::ID,
            Self::GetRandomNumber(_) => get_random_number::ID,
            Self::GetTimer(_) => get_timer::ID,
            Self::GetToken(_) => get_token::ID,
            Self::GetTrueRandomEntropySource(_) => get_true_random_entropy_source::ID,
            Self::GetXncpInfo(_) => get_xncp_info::ID,
            Self::InvalidCommand(_) => invalid_command::ID,
            Self::Nop(_) => nop::ID,
            Self::ReadAndClearCounters(_) => read_and_clear_counters::ID,
            Self::ReadCounters(_) => read_counters::ID,
            Self::SetMfgToken(_) => set_mfg_token::ID,
            Self::SetTimer(_) => set_timer::ID,
            Self::SetToken(_) => set_token::ID,
        }
    }
}

impl Writable for Response {
    fn write_to<W>(self, dst: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        match self {
            Self::CustomFrame(custom_frame) => custom_frame.write_to(dst),
            Self::DebugWrite(debug_write) => debug_write.write_to(dst),
            Self::DelayTest(delay_test) => delay_test.write_to(dst),
            Self::Echo(echo) => echo.write_to(dst),
            Self::GetEui64(get_eui64) => get_eui64.write_to(dst),
            Self::GetLibraryStatus(get_library_status) => get_library_status.write_to(dst),
            Self::GetMfgToken(get_mfg_token) => get_mfg_token.write_to(dst),
            Self::GetNodeId(get_node_id) => get_node_id.write_to(dst),
            Self::GetPhyInterfaceCount(get_phy_interface_count) => {
                get_phy_interface_count.write_to(dst)
            }
            Self::GetRandomNumber(get_random_number) => get_random_number.write_to(dst),
            Self::GetTimer(get_timer) => get_timer.write_to(dst),
            Self::GetToken(get_token) => get_token.write_to(dst),
            Self::GetTrueRandomEntropySource(get_true_random_entropy_source) => {
                get_true_random_entropy_source.write_to(dst)
            }
            Self::GetXncpInfo(get_xncp_info) => get_xncp_info.write_to(dst),
            Self::InvalidCommand(invalid_command) => invalid_command.write_to(dst),
            Self::Nop(nop) => nop.write_to(dst),
            Self::ReadAndClearCounters(read_and_clear_counters) => {
                read_and_clear_counters.write_to(dst)
            }
            Self::ReadCounters(read_counters) => read_counters.write_to(dst),
            Self::SetMfgToken(set_mfg_token) => set_mfg_token.write_to(dst),
            Self::SetTimer(set_timer) => set_timer.write_to(dst),
            Self::SetToken(set_token) => set_token.write_to(dst),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Callback {
    CounterRollover(counter_rollover_handler::Response),
    CustomFrame(custom_frame_handler::Response),
    NoCallbacks(no_callbacks::Response),
    StackTokenChanged(stack_token_changed_handler::Response),
    Timer(timer_handler::Response),
}

impl Callback {
    #[must_use]
    pub const fn id(&self) -> u16 {
        match self {
            Self::CounterRollover(_) => counter_rollover_handler::ID,
            Self::CustomFrame(_) => custom_frame_handler::ID,
            Self::NoCallbacks(_) => no_callbacks::ID,
            Self::StackTokenChanged(_) => stack_token_changed_handler::ID,
            Self::Timer(_) => timer_handler::ID,
        }
    }
}

impl Writable for Callback {
    fn write_to<W>(self, dst: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        match self {
            Self::CounterRollover(counter_rollover_handler) => {
                counter_rollover_handler.write_to(dst)
            }
            Self::CustomFrame(custom_frame_handler) => custom_frame_handler.write_to(dst),
            Self::NoCallbacks(no_callbacks) => no_callbacks.write_to(dst),
            Self::StackTokenChanged(stack_token_changed_handler) => {
                stack_token_changed_handler.write_to(dst)
            }
            Self::Timer(timer_handler) => timer_handler.write_to(dst),
        }
    }
}
