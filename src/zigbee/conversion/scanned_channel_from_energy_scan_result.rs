use zigbee_nwk::ScannedChannel;

use crate::parameters::networking::handler::EnergyScanResult;

impl From<EnergyScanResult> for ScannedChannel {
    fn from(energy_scan_result: EnergyScanResult) -> Self {
        Self::new(
            energy_scan_result.channel(),
            energy_scan_result.max_rssi_value(),
        )
    }
}
