//! Combine modern network-key export with its security metadata.

use std::fmt::{self, Debug, Formatter};

use silizium::zigbee::security::man::{
    Context, DerivedKeyType, Flags, Key, KeyType, NetworkKeyInfo,
};

use crate::ember::Eui64;
use crate::{Error, Security, ValueError};

const PRIMARY_NETWORK: u8 = 0;
const CURRENT_KEY: u8 = 0;
const DEFAULT_PSA_PERMISSION: u32 = 0;

/// Current primary-network key together with sequence and outgoing-counter metadata.
///
/// This is an observation, not a complete or atomic backup. Export is bracketed
/// by metadata reads to reject observable changes, but callers must still
/// synchronize with autonomous NCP activity. Debug output redacts the key;
/// this type does not zeroize memory.
#[derive(Clone, Eq, PartialEq)]
pub struct NetworkKeyExport {
    key: Key,
    info: NetworkKeyInfo,
}

impl NetworkKeyExport {
    /// Explicitly access the secret network key bytes.
    #[must_use]
    pub const fn expose_key(&self) -> &Key {
        &self.key
    }

    /// Read metadata, including the active sequence number, outgoing counter,
    /// and whether an alternate network key exists.
    #[must_use]
    pub const fn info(&self) -> &NetworkKeyInfo {
        &self.info
    }
}

impl Debug for NetworkKeyExport {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NetworkKeyExport")
            .field("key", &"[REDACTED]")
            .field("info", &self.info)
            .finish()
    }
}

/// Helpers for exporting security material without dropping its metadata.
pub trait SecurityExt {
    /// Export the current primary-network key and its metadata using Security Manager APIs.
    ///
    /// Requires firmware implementing `getNetworkKeyInfo` and `exportKey`.
    /// Does not export the alternate key or replace snapshot synchronization.
    ///
    /// # Errors
    /// Forwards command failures, rejects an unset active key, and rejects any
    /// observed metadata change while exporting. No partial value is returned.
    fn export_network_key_with_info(
        &mut self,
    ) -> impl Future<Output = Result<NetworkKeyExport, Error>> + Send;
}

impl<T> SecurityExt for T
where
    T: Security + Send,
{
    async fn export_network_key_with_info(&mut self) -> Result<NetworkKeyExport, Error> {
        let info = self.get_network_key_info().await?;
        if !info.network_key_set() {
            return Err(ValueError::MissingNetworkKey.into());
        }
        let context = Context::new(
            KeyType::Network,
            CURRENT_KEY,
            DerivedKeyType::None,
            Eui64::default(),
            PRIMARY_NETWORK,
            Flags::NONE,
            DEFAULT_PSA_PERMISSION,
        );
        let key = self.export_key(context).await?;
        if self.get_network_key_info().await? != info {
            return Err(ValueError::NetworkKeyChanged.into());
        }
        Ok(NetworkKeyExport { key, info })
    }
}

#[cfg(test)]
mod tests {
    use super::SecurityExt;
    use crate::test_support::{Script, Transaction, ready};
    use crate::{Error, ValueError};

    const NETWORK_KEY_INFO: u16 = 0x0116;
    const EXPORT_KEY: u16 = 0x0114;
    const INFO: &[u8] = &[0, 0, 0, 0, 1, 1, 7, 8, 0x78, 0x56, 0x34, 0x12];
    const CONTEXT: &[u8] = &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    const KEY: [u8; 16] = [0xAB; 16];

    fn exported_key() -> Vec<u8> {
        KEY.into_iter().chain([0, 0, 0, 0]).collect()
    }

    #[test]
    fn retains_modern_sequence_counter_and_alternate_key_metadata() {
        let mut target = Script::new([
            Transaction::reply(NETWORK_KEY_INFO, &[], INFO),
            Transaction::reply(EXPORT_KEY, CONTEXT, &exported_key()),
            Transaction::reply(NETWORK_KEY_INFO, &[], INFO),
        ]);
        let export = ready(target.export_network_key_with_info()).unwrap();
        assert_eq!(export.expose_key(), &KEY);
        assert_eq!(export.info().network_key_sequence_number(), 7);
        assert_eq!(export.info().network_key_frame_counter(), 0x1234_5678);
        assert!(export.info().alternate_network_key_set());
        assert_eq!(export.info().alt_network_key_sequence_number(), 8);
        assert!(format!("{export:?}").contains("[REDACTED]"));
        assert!(!format!("{export:?}").contains("[171, 171"));
        target.assert_finished();
    }

    #[test]
    fn rejects_missing_key_before_exporting() {
        let mut info = INFO.to_vec();
        info[4] = 0;
        let mut target = Script::new([Transaction::reply(NETWORK_KEY_INFO, &[], &info)]);
        assert!(matches!(
            ready(target.export_network_key_with_info()),
            Err(Error::ValueError(ValueError::MissingNetworkKey))
        ));
        target.assert_finished();
    }

    #[test]
    fn rejects_changed_sequence_counter_or_alternate_key_state() {
        for index in [5, 6, 7, 8] {
            let mut changed = INFO.to_vec();
            changed[index] ^= 1;
            let mut target = Script::new([
                Transaction::reply(NETWORK_KEY_INFO, &[], INFO),
                Transaction::reply(EXPORT_KEY, CONTEXT, &exported_key()),
                Transaction::reply(NETWORK_KEY_INFO, &[], &changed),
            ]);
            assert!(matches!(
                ready(target.export_network_key_with_info()),
                Err(Error::ValueError(ValueError::NetworkKeyChanged))
            ));
            target.assert_finished();
        }
    }

    #[test]
    fn key_export_failure_stops_before_the_second_metadata_read() {
        let mut target = Script::new([
            Transaction::reply(NETWORK_KEY_INFO, &[], INFO),
            Transaction::fail(EXPORT_KEY, CONTEXT),
        ]);
        assert!(matches!(
            ready(target.export_network_key_with_info()),
            Err(Error::ChannelClosed)
        ));
        target.assert_finished();
    }
}
