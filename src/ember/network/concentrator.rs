use le_stream::derive::ToLeBytes;

#[derive(Clone, Debug, Default, Eq, PartialEq, ToLeBytes)]
pub struct Parameters {
    concentrator_type: u16,
    min_time: u16,
    max_time: u16,
    route_error_threshold: u8,
    delivery_failure_threshold: u8,
    max_hops: u8,
}

impl Parameters {
    #[must_use]
    pub const fn new(
        concentrator_type: u16,
        min_time: u16,
        max_time: u16,
        route_error_threshold: u8,
        delivery_failure_threshold: u8,
        max_hops: u8,
    ) -> Self {
        Self {
            concentrator_type,
            min_time,
            max_time,
            route_error_threshold,
            delivery_failure_threshold,
            max_hops,
        }
    }
}
