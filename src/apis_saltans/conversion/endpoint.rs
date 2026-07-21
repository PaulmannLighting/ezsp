//! Conversion between startup endpoints and ZDP simple descriptors.
//!
//! Converting a `SimpleDescriptor` to [`crate::Endpoint`] copies its endpoint,
//! profile, device, application flags, and cluster lists. Conversion back is
//! fallible because the raw EZSP endpoint representation can contain a profile
//! unknown to `apis-saltans` or a reserved endpoint number.

use apis_saltans_hw::core::Profile;
use apis_saltans_hw::zdp::{AppFlags, SimpleDescriptor};

use crate::Endpoint;

impl From<SimpleDescriptor> for Endpoint {
    fn from(value: SimpleDescriptor) -> Self {
        Self {
            id: value.endpoint_id(),
            profile_id: value.profile_id(),
            device_id: value.device_id(),
            app_flags: value.app_flags(),
            input_clusters: value.input_clusters().iter().copied().collect(),
            output_clusters: value.output_clusters().iter().copied().collect(),
        }
    }
}

impl TryFrom<Endpoint> for SimpleDescriptor {
    type Error = Endpoint;

    fn try_from(value: Endpoint) -> Result<Self, Self::Error> {
        let Ok(profile) = Profile::try_from(value.profile_id) else {
            return Err(value);
        };

        let Ok(endpoint) = apis_saltans_hw::core::Endpoint::try_from(value.id) else {
            return Err(value);
        };

        Ok(Self::new(
            endpoint,
            profile,
            value.device_id,
            AppFlags::from_bits_retain(value.app_flags),
            value.input_clusters,
            value.output_clusters,
        ))
    }
}
