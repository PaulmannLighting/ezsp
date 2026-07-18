//! Conversion from `apis-saltans` endpoint descriptors to EZSP cluster metadata.

use apis_saltans_hw::core::Cluster;
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

impl From<&Clusters> for apis_saltans_hw::Clusters {
    fn from(clusters: &Clusters) -> Self {
        Self::new(
            clusters
                .input_clusters()
                .iter()
                .copied()
                .filter_map(|cluster| Cluster::try_from(cluster).ok())
                .collect(),
            clusters
                .output_clusters()
                .iter()
                .copied()
                .filter_map(|cluster| Cluster::try_from(cluster).ok())
                .collect(),
        )
    }
}
