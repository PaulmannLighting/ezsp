//! Conversion from `apis-saltans` endpoint descriptors to EZSP cluster metadata.

use apis_saltans_hw::zdp::SimpleDescriptor;

use crate::parameters::configuration::add_endpoint::Clusters;

impl From<&SimpleDescriptor> for Clusters {
    fn from(descriptor: &SimpleDescriptor) -> Self {
        Self::new(
            descriptor.input_clusters().iter().copied().collect(),
            descriptor.output_clusters().iter().copied().collect(),
        )
    }
}
