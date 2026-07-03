//! Parameters for the [`Wwah::is_uptime_long`](crate::Wwah::is_uptime_long) command.

crate::frame::parameters::frame!(0x00E5, {}, { has_long_up_time: bool });

/// Convert the response into a boolean value of whether the uptime is long.
impl From<Response> for bool {
    fn from(response: Response) -> Self {
        response.has_long_up_time
    }
}
