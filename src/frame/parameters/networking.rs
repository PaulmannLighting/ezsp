use crate::read_write::Writable;
use std::io::Write;

pub mod energy_scan_result_handler;
pub mod find_unused_pan_id;
pub mod form_network;
pub mod join_network;
pub mod network_found_handler;
pub mod network_init;
pub mod network_state;
pub mod scan_complete_handler;
pub mod set_manufacturer_code;
pub mod set_power_descriptor;
pub mod stack_status_handler;
pub mod start_scan;
pub mod stop_scan;
pub mod unused_pan_id_found_handler;

#[derive(Debug, Eq, PartialEq)]
pub enum Command {}

impl Command {
    #[must_use]
    pub const fn id(&self) -> u16 {
        todo!()
    }
}

impl Writable for Command {
    fn write_to<W>(self, dst: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        match self {}
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Response {}

impl Response {
    #[must_use]
    pub const fn id(&self) -> u16 {
        todo!()
    }
}

impl Writable for Response {
    fn write_to<W>(self, dst: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        match self {}
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Callback {}

impl Callback {
    #[must_use]
    pub const fn id(&self) -> u16 {
        todo!()
    }
}

impl Writable for Callback {
    fn write_to<W>(self, dst: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        match self {}
    }
}
