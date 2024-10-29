use crate::frame::enums::{Parameters, Response};
use crate::parameters::green_power;

impl TryFrom<Parameters> for green_power::sink_table::clear_all::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::GreenPower(green_power::Response::SinkTable(
                green_power::sink_table::Response::ClearAll(response),
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for green_power::sink_table::find_or_allocate_entry::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::GreenPower(green_power::Response::SinkTable(
                green_power::sink_table::Response::FindOrAllocateEntry(response),
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for green_power::sink_table::get_entry::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::GreenPower(green_power::Response::SinkTable(
                green_power::sink_table::Response::GetEntry(response),
            ))) => Ok(*response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for green_power::sink_table::init::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::GreenPower(green_power::Response::SinkTable(
                green_power::sink_table::Response::Init(response),
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for green_power::sink_table::lookup::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::GreenPower(green_power::Response::SinkTable(
                green_power::sink_table::Response::Lookup(response),
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for green_power::sink_table::number_of_active_entries::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::GreenPower(green_power::Response::SinkTable(
                green_power::sink_table::Response::NumberOfActiveEntries(response),
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for green_power::sink_table::remove_entry::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::GreenPower(green_power::Response::SinkTable(
                green_power::sink_table::Response::RemoveEntry(response),
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for green_power::sink_table::set_entry::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::GreenPower(green_power::Response::SinkTable(
                green_power::sink_table::Response::SetEntry(response),
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for green_power::sink_table::set_security_frame_counter::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::GreenPower(green_power::Response::SinkTable(
                green_power::sink_table::Response::SetSecurityFrameCounter(response),
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}
