//! Conversion from `apis-saltans` endpoint descriptors to EZSP cluster metadata.

use apis_saltans_hw::zdp::SimpleDescriptor;

use crate::ncp::builder::Endpoint;
use crate::parameters::configuration::add_endpoint::Clusters;

impl From<&SimpleDescriptor> for Clusters {
    fn from(descriptor: &SimpleDescriptor) -> Self {
        Self::new(
            descriptor.input_clusters().iter().copied().collect(),
            descriptor.output_clusters().iter().copied().collect(),
        )
    }
}

impl From<&SimpleDescriptor> for Endpoint {
    fn from(descriptor: &SimpleDescriptor) -> Self {
        Self::new(
            descriptor.profile_id(),
            descriptor.device_id(),
            descriptor.app_flags(),
            descriptor.into(),
        )
    }
}
