//! Energy-scan result conversion.
//!
//! The channel number and maximum observed RSSI are preserved. Results outside
//! the Zigbee page-zero channel range are rejected.

use apis_saltans_hw::{Channel, ScannedChannel};

use crate::parameters::networking::handler::EnergyScanResult;

impl TryFrom<EnergyScanResult> for ScannedChannel {
    type Error = u8;

    fn try_from(energy_scan_result: EnergyScanResult) -> Result<Self, Self::Error> {
        let channel = energy_scan_result.channel();

        Ok(Self::new(
            Channel::new(channel).ok_or(channel)?,
            energy_scan_result.max_rssi_value(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use apis_saltans_hw::{Channel, ScannedChannel};
    use le_stream::FromLeStream;

    use crate::parameters::networking::handler::EnergyScanResult;

    const VALID_CHANNEL: u8 = 11;
    const INVALID_CHANNEL: u8 = VALID_CHANNEL - 1;
    const MAX_RSSI_DBM: i8 = -42;

    fn energy_scan_result(channel: u8) -> EnergyScanResult {
        EnergyScanResult::from_le_stream([channel, MAX_RSSI_DBM.to_le_bytes()[0]].into_iter())
            .expect("energy scan result test callback is complete")
    }

    #[test]
    fn converts_valid_channel() {
        let scanned = ScannedChannel::try_from(energy_scan_result(VALID_CHANNEL))
            .expect("test channel is valid");

        assert_eq!(scanned.channel(), Channel::MIN);
        assert_eq!(scanned.max_rssi_dbm(), MAX_RSSI_DBM);
    }

    #[test]
    fn rejects_invalid_channel() {
        assert_eq!(
            ScannedChannel::try_from(energy_scan_result(INVALID_CHANNEL)),
            Err(INVALID_CHANNEL)
        );
    }
}
