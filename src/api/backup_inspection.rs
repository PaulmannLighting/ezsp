//! Read-only evidence collection for future complete-backup firmware profiles.

use std::io::{self, ErrorKind};

use crate::ember::network::Status;
use crate::ember::token::Info;
use crate::ezsp::value::{EmberVersion, Id};
use crate::{Configuration, Connection, Error, GetValueExt, Networking, TokenInterface};

/// Protocol whose legacy token response layouts are used by backup inspection.
///
/// This identifies a wire-layout prerequisite, not a certified backup profile.
pub const BACKUP_INSPECTION_PROTOCOL_VERSION: u8 = 13;

/// Read-only firmware observations, without token contents or secret keys.
///
/// Successful inspection does **not** certify complete dump or restore support.
/// It cannot establish snapshot consistency, replay-state persistence, restored
/// identity behavior, or transport reconnection after reset. No firmware profile
/// has been certified for the complete `apis-saltans` backup contract yet.
#[derive(Debug)]
pub struct BackupInspection {
    /// Negotiated EZSP protocol version.
    pub protocol_version: u8,

    /// Reported `EmberZNet` firmware version.
    pub firmware: EmberVersion,

    /// Observed network state; this is not an exclusive maintenance lock.
    pub network_state: Status,

    /// Raw key storage version: classic storage is zero and PSA storage is one.
    /// Unknown values are retained for firmware qualification, not accepted as compatible.
    pub key_storage_version: u8,

    /// Token table descriptions, without reading their secret-bearing payloads.
    pub tokens: Vec<Info>,
}

impl Connection {
    /// Collect read-only evidence for firmware backup qualification.
    ///
    /// Queries version information, network state, key storage, token count,
    /// and token descriptions. Does not export keys, read token values, stop
    /// the network, write storage, or enable backup capabilities. Other protocol
    /// versions are rejected before issuing commands because their response
    /// layouts have not been qualified here.
    ///
    /// # Errors
    /// Returns unsupported for unqualified protocol layouts, forwards command
    /// failures (including an absent token-interface component), and rejects
    /// malformed firmware or key-storage responses. Never returns a partial report.
    pub async fn inspect_backup_firmware(&mut self) -> Result<BackupInspection, Error> {
        inspect(self, self.desired_version.get()).await
    }
}

pub async fn inspect<T>(target: &mut T, protocol_version: u8) -> Result<BackupInspection, Error>
where
    T: Configuration + Networking + TokenInterface + Send,
{
    if protocol_version != BACKUP_INSPECTION_PROTOCOL_VERSION {
        return Err(io::Error::new(
            ErrorKind::Unsupported,
            "backup inspection requires the EZSP v13 token response layout",
        )
        .into());
    }
    let firmware = target
        .get_ember_version()
        .await?
        .map_err(|error| io::Error::new(ErrorKind::InvalidData, error))?;
    let network_state = target.network_state().await?;
    let storage = target.get_value(Id::KeyStorageVersion).await?;
    let [key_storage_version] = storage.as_slice() else {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            "key storage version must contain exactly one byte",
        )
        .into());
    };
    let count = target.get_token_count().await?;
    let mut tokens = Vec::with_capacity(count.into());
    for index in 0..count {
        tokens.push(target.get_token_info(index).await?);
    }
    Ok(BackupInspection {
        protocol_version,
        firmware,
        network_state,
        key_storage_version: *key_storage_version,
        tokens,
    })
}

#[cfg(test)]
mod tests {
    use std::io::ErrorKind;

    use super::{BACKUP_INSPECTION_PROTOCOL_VERSION, inspect};
    use crate::Error;
    use crate::ember::network::Status;
    use crate::test_support::{Script, Transaction, ready};

    const GET_VALUE: u16 = 0x00AA;
    const NETWORK_STATE: u16 = 0x0018;
    const TOKEN_COUNT: u16 = 0x0100;
    const TOKEN_INFO: u16 = 0x0101;
    const VERSION: &[u8] = &[0, 7, 0x34, 0x12, 7, 4, 2, 0, 0];

    fn prefix(storage: &[u8]) -> Vec<Transaction> {
        vec![
            Transaction::reply(GET_VALUE, &[0x11], VERSION),
            Transaction::reply(NETWORK_STATE, &[], &[2]),
            Transaction::reply(GET_VALUE, &[0x44], storage),
        ]
    }

    #[test]
    fn collects_only_read_only_metadata_and_preserves_unknown_storage_versions() {
        let mut steps = prefix(&[0, 1, 0xFF]);
        steps.extend([
            Transaction::reply(TOKEN_COUNT, &[], &[2]),
            Transaction::reply(TOKEN_INFO, &[0], &[0, 0x78, 0x56, 0x34, 0x12, 1, 0, 4, 1]),
            Transaction::reply(TOKEN_INFO, &[1], &[0, 0x79, 0x56, 0x34, 0x12, 0, 1, 16, 8]),
        ]);
        let mut target = Script::new(steps);
        let report = ready(inspect(&mut target, BACKUP_INSPECTION_PROTOCOL_VERSION)).unwrap();
        assert_eq!(report.protocol_version, 13);
        assert_eq!(report.firmware.build(), 0x1234);
        assert_eq!(report.firmware.major(), 7);
        assert_eq!(report.firmware.minor(), 4);
        assert_eq!(report.firmware.patch(), 2);
        assert_eq!(report.network_state, Status::JoinedNetwork);
        assert_eq!(report.key_storage_version, 0xFF);
        assert_eq!(report.tokens.len(), 2);
        assert_eq!(report.tokens[0].nvm3_key(), 0x1234_5678);
        assert!(report.tokens[0].is_cnt());
        assert!(report.tokens[1].is_idx());
        assert_eq!(report.tokens[1].array_size(), 8);
        target.assert_finished();
    }

    #[test]
    fn unqualified_protocol_versions_issue_no_commands() {
        for version in [0, 8, 12, 14, 18, u8::MAX] {
            let mut target = Script::new([]);
            assert!(matches!(ready(inspect(&mut target, version)),
                Err(Error::Io(error)) if error.kind() == ErrorKind::Unsupported));
            target.assert_finished();
        }
    }

    #[test]
    fn rejects_malformed_version_before_further_probes() {
        let mut target = Script::new([Transaction::reply(GET_VALUE, &[0x11], &[0, 1, 7])]);
        assert!(
            matches!(ready(inspect(&mut target, BACKUP_INSPECTION_PROTOCOL_VERSION)),
            Err(Error::Io(error)) if error.kind() == ErrorKind::InvalidData)
        );
        target.assert_finished();
    }

    #[test]
    fn malformed_storage_version_prevents_token_queries() {
        for storage in [&[0, 0][..], &[0, 2, 0, 1][..]] {
            let mut target = Script::new(prefix(storage));
            assert!(
                matches!(ready(inspect(&mut target, BACKUP_INSPECTION_PROTOCOL_VERSION)),
                Err(Error::Io(error)) if error.kind() == ErrorKind::InvalidData)
            );
            target.assert_finished();
        }
    }

    #[test]
    fn absent_token_component_or_partial_inventory_returns_an_error() {
        for failed_info in [false, true] {
            let mut steps = prefix(&[0, 1, 0]);
            if failed_info {
                steps.push(Transaction::reply(TOKEN_COUNT, &[], &[1]));
                steps.push(Transaction::fail(TOKEN_INFO, &[0]));
            } else {
                steps.push(Transaction::fail(TOKEN_COUNT, &[]));
            }
            let mut target = Script::new(steps);
            assert!(matches!(
                ready(inspect(&mut target, BACKUP_INSPECTION_PROTOCOL_VERSION)),
                Err(Error::ChannelClosed)
            ));
            target.assert_finished();
        }
    }
}
