use std::future::poll_fn;
use std::num::{NonZeroU32, NonZeroUsize};
use std::pin::pin;
use std::task::Poll;

use apis_saltans_hw::core::IeeeAddress;
use apis_saltans_hw::{
    BackupError, BackupExtension, BackupSource, Channel, ChannelMask, DeviceBackup, Driver, Error,
    LinkKeyState, NetworkBackup, NetworkDump, NetworkKey, NetworkKeyState, NetworkParameters,
    NetworkRestore, RestoreFailureState, RestoreOptions,
};
use tokio::sync::mpsc;

use super::{
    EMBER_BACKUP_NAMESPACE, EMBER_BACKUP_STACK, EMBER_SECURITY_EXTENSION, EMBER_SECURITY_VERSION,
    EmberSecurityState, RESTORE_BLOCKER, RestorePlan, SNAPSHOT_BLOCKER,
};
use crate::Ncp;
use crate::ember::aps::Options;
use crate::ember::security::initial::Bitmask;
use crate::test_support::{ConnectionScript, Script, Transaction, ready};

const IEEE: IeeeAddress = IeeeAddress::new(1, 2, 3, 4, 5, 6, 7, 8);
const REMOTE: IeeeAddress = IeeeAddress::new(8, 7, 6, 5, 4, 3, 2, 1);
const SECRET: [u8; 16] = [0xAB; 16];
const FIRMWARE: &str = "7.4.2-4660 (0 / 0)";
const NWK_COUNTER: u32 = 100;
const APS_COUNTER: u32 = 200;
const LINK_COUNTER: u32 = 300;
const RX_COUNTER: u32 = 400;
const NETWORK_STATE: u16 = 0x0018;
const GET_VALUE: u16 = 0x00AA;
const GET_EUI64: u16 = 0x0026;
const GET_CONFIGURATION: u16 = 0x0052;
const TOKEN_COUNT: u16 = 0x0100;
const VERSION: &[u8] = &[0, 7, 0x34, 0x12, 7, 4, 2, 0, 0];

pub(super) fn options() -> RestoreOptions {
    RestoreOptions::new(NonZeroU32::new(10).unwrap(), NonZeroU32::new(20).unwrap())
}

fn security(counter: u32) -> EmberSecurityState {
    EmberSecurityState::new(
        Bitmask::HAVE_NETWORK_KEY
            | Bitmask::HAVE_PRECONFIGURED_KEY
            | Bitmask::TRUST_CENTER_USES_HASHED_LINK_KEY,
        NetworkKey::new(SECRET),
        counter,
    )
    .unwrap()
}

fn snapshot(counter: u32, extensions: Vec<BackupExtension>) -> NetworkBackup {
    NetworkBackup::new(
        BackupSource::new(EMBER_BACKUP_STACK.into(), FIRMWARE.into()).unwrap(),
        NetworkParameters::new(
            IEEE,
            0x1234,
            [8, 7, 6, 5, 4, 3, 2, 1],
            Channel::MIN,
            ChannelMask::ALL,
            7,
            5,
        )
        .unwrap(),
        NetworkKeyState::new(NetworkKey::new(SECRET), 9, counter),
        vec![],
        vec![DeviceBackup::new(
            REMOTE,
            None,
            true,
            Some(LinkKeyState::new(
                NetworkKey::new(SECRET),
                LINK_COUNTER,
                RX_COUNTER,
            )),
            Some(RX_COUNTER),
        )],
        extensions,
    )
    .unwrap()
}

pub(super) fn backup() -> NetworkBackup {
    snapshot(
        NWK_COUNTER,
        vec![security(APS_COUNTER).to_extension().unwrap()],
    )
}

fn ncp(protocol: u8, transactions: Vec<Transaction>) -> (Ncp, ConnectionScript) {
    let (connection, script) = ConnectionScript::new(protocol, Script::new(transactions));
    let (events, _receiver) = mpsc::channel(1);
    let ncp = ready(Ncp::new(connection, Box::new([]), events, Options::NONE)).unwrap();
    (ncp, script)
}

pub(super) async fn with_actor<A, R>(actor: A, request: R) -> R::Output
where
    A: Future,
    R: Future,
{
    let mut actor = pin!(actor);
    let mut request = pin!(request);
    poll_fn(|context| {
        if let Poll::Ready(output) = request.as_mut().poll(context) {
            return Poll::Ready(output);
        }
        assert!(
            actor.as_mut().poll(context).is_pending(),
            "actor stopped unexpectedly"
        );
        request.as_mut().poll(context)
    })
    .await
}

fn target_prefix() -> Vec<Transaction> {
    vec![
        Transaction::reply(NETWORK_STATE, &[], &[0]),
        Transaction::reply(GET_VALUE, &[0x44], &[0, 1, 0]),
        Transaction::reply(GET_VALUE, &[0x11], VERSION),
        Transaction::reply(GET_EUI64, &[], &[8, 7, 6, 5, 4, 3, 2, 1]),
    ]
}

fn target() -> Vec<Transaction> {
    let mut steps = target_prefix();
    steps.extend([
        Transaction::reply(GET_CONFIGURATION, &[0x1E], &[0, 8, 0]),
        Transaction::reply(GET_CONFIGURATION, &[0x11], &[0, 8, 0]),
    ]);
    steps
}

fn dump_steps() -> Vec<Transaction> {
    vec![
        Transaction::reply(GET_VALUE, &[0x11], VERSION),
        Transaction::reply(NETWORK_STATE, &[], &[2]),
        Transaction::reply(GET_VALUE, &[0x44], &[0, 1, 0]),
        Transaction::reply(TOKEN_COUNT, &[], &[0]),
    ]
}

#[test]
fn extension_round_trip_has_exact_versioned_layout_and_redacted_keys() {
    let security = security(APS_COUNTER);
    let extension = security.to_extension().unwrap();
    let mut expected = security.flags().bits().to_le_bytes().to_vec();
    expected.extend(SECRET);
    expected.extend(APS_COUNTER.to_le_bytes());
    assert_eq!(extension.payload(), expected);
    assert_eq!(extension.payload().len(), 22);
    assert_eq!(
        EmberSecurityState::from_extension(&extension).unwrap(),
        security
    );
    assert_eq!(security.trust_center_key().expose_secret(), &SECRET);
    assert!(format!("{security:?}").contains("[REDACTED]"));
    assert!(!format!("{security:?}").contains("[171, 171"));
}

#[test]
fn extension_rejects_unknown_identity_version_length_and_flags() {
    let valid = security(APS_COUNTER).to_extension().unwrap();
    for (namespace, name, version, payload) in [
        (
            "foreign",
            EMBER_SECURITY_EXTENSION,
            EMBER_SECURITY_VERSION,
            valid.payload().to_vec(),
        ),
        (
            EMBER_BACKUP_NAMESPACE,
            "unknown",
            EMBER_SECURITY_VERSION,
            valid.payload().to_vec(),
        ),
        (
            EMBER_BACKUP_NAMESPACE,
            EMBER_SECURITY_EXTENSION,
            2,
            valid.payload().to_vec(),
        ),
        (
            EMBER_BACKUP_NAMESPACE,
            EMBER_SECURITY_EXTENSION,
            EMBER_SECURITY_VERSION,
            vec![0; 21],
        ),
        (
            EMBER_BACKUP_NAMESPACE,
            EMBER_SECURITY_EXTENSION,
            EMBER_SECURITY_VERSION,
            vec![0; 23],
        ),
        (
            EMBER_BACKUP_NAMESPACE,
            EMBER_SECURITY_EXTENSION,
            EMBER_SECURITY_VERSION,
            vec![0xFF; 22],
        ),
    ] {
        let extension =
            BackupExtension::new(namespace.into(), name.into(), version, payload).unwrap();
        assert!(EmberSecurityState::from_extension(&extension).is_err());
    }
    for flags in [Bitmask::HAVE_NETWORK_KEY, Bitmask::all()] {
        assert!(EmberSecurityState::new(flags, NetworkKey::new(SECRET), APS_COUNTER).is_err());
    }
}

#[test]
fn plan_advances_every_outgoing_counter_once_and_preserves_incoming_state() {
    let original = backup();
    let plan = RestorePlan::prepare(&original, options()).unwrap();
    let prepared = plan.backup();
    assert_eq!(original.network_key().tx_counter(), NWK_COUNTER);
    assert_eq!(prepared.network_key().tx_counter(), 110);
    assert_eq!(prepared.network_key().sequence_number(), 9);
    assert_eq!(prepared.network_key().key(), original.network_key().key());
    assert_eq!(prepared.network(), original.network());
    assert_eq!(prepared.devices()[0].link_key().unwrap().tx_counter(), 320);
    assert_eq!(
        prepared.devices()[0].link_key().unwrap().rx_counter(),
        RX_COUNTER
    );
    assert_eq!(prepared.devices()[0].nwk_rx_counter(), Some(RX_COUNTER));
    assert_eq!(plan.security().aps_tx_counter(), 220);
    assert_eq!(
        EmberSecurityState::from_extension(&prepared.extensions()[0])
            .unwrap()
            .aps_tx_counter(),
        220
    );
    let actor_prepared = original.prepare_restore(options()).unwrap();
    let actor_plan = RestorePlan::from_prepared(&actor_prepared, options()).unwrap();
    assert_eq!(actor_plan.backup(), prepared);
}

#[test]
fn plan_rejects_all_counter_overflows_and_unknown_required_extensions() {
    for counter in [u32::MAX - 1, u32::MAX] {
        assert!(matches!(
            RestorePlan::prepare(
                &snapshot(counter, vec![security(APS_COUNTER).to_extension().unwrap()]),
                options()
            ),
            Err(BackupError::CounterExhausted)
        ));
        assert!(matches!(
            RestorePlan::prepare(
                &snapshot(NWK_COUNTER, vec![security(counter).to_extension().unwrap()]),
                options()
            ),
            Err(BackupError::CounterExhausted)
        ));
    }
    for extensions in [
        vec![],
        vec![BackupExtension::new("foreign".into(), "state".into(), 1, vec![]).unwrap()],
        vec![
            security(APS_COUNTER).to_extension().unwrap(),
            BackupExtension::new("foreign".into(), "state".into(), 1, vec![]).unwrap(),
        ],
    ] {
        assert!(RestorePlan::prepare(&snapshot(NWK_COUNTER, extensions), options()).is_err());
    }
}

#[test]
fn disabled_actor_keeps_both_operations_unsupported() {
    let (ncp, mut script) = ncp(13, vec![]);
    let (handle, actor) = ncp.into_actor(NonZeroUsize::MIN);
    let result = script.run(with_actor(actor, async {
        let capabilities = handle.backup_capabilities().await.unwrap();
        assert!(!capabilities.can_dump());
        assert!(!capabilities.can_restore());
        handle.dump_network().await
    }));
    assert!(matches!(result, Err(Error::Unsupported(_))));
}

#[test]
fn registered_dump_checks_metadata_and_refuses_partial_export_without_writes() {
    let (ncp, mut script) = ncp(13, dump_steps());
    let (handle, actor) = ncp
        .actor_builder(NonZeroUsize::MIN)
        .with_network_dump()
        .build();
    let result = script.run(with_actor(actor, async {
        let capabilities = handle.backup_capabilities().await.unwrap();
        assert!(capabilities.can_dump());
        assert!(!capabilities.can_restore());
        handle.dump_network().await
    }));
    assert!(matches!(
        result,
        Err(Error::Backup(BackupError::Incompatible(SNAPSHOT_BLOCKER)))
    ));
}

#[test]
fn registered_restore_validates_the_target_without_mutation() {
    let (ncp, mut script) = ncp(13, target());
    let (handle, actor) = ncp
        .actor_builder(NonZeroUsize::MIN)
        .with_network_restore()
        .build();
    let result = script.run(with_actor(actor, async {
        let capabilities = handle.backup_capabilities().await.unwrap();
        assert!(!capabilities.can_dump());
        assert!(capabilities.can_restore());
        handle.restore_network(backup(), options()).await
    }));
    assert!(matches!(
        result,
        Err(Error::Backup(BackupError::Incompatible(RESTORE_BLOCKER)))
    ));
}

#[test]
fn direct_restore_revalidates_and_reports_unchanged_with_source() {
    let (mut ncp, mut script) = ncp(13, target());
    let prepared = backup().prepare_restore(options()).unwrap();
    let result = script.run(ncp.restore_network(prepared, options()));
    let Err(Error::Backup(BackupError::RestoreFailed { state, source })) = result else {
        panic!("expected an unchanged restore failure");
    };
    assert_eq!(state, RestoreFailureState::Unchanged);
    assert!(matches!(
        source.downcast_ref::<Error>(),
        Some(Error::Backup(BackupError::Incompatible(RESTORE_BLOCKER)))
    ));
}

#[test]
fn unqualified_protocol_and_bad_snapshots_issue_no_commands() {
    let (mut ncp, mut script) = ncp(14, vec![]);
    assert!(matches!(
        script.run(ncp.dump_network()),
        Err(Error::Backup(BackupError::Incompatible(_)))
    ));
    let invalid = snapshot(NWK_COUNTER, vec![]);
    assert!(
        script
            .run(ncp.validate_restore(&invalid, options()))
            .is_err()
    );
}

#[test]
fn actor_rejects_normalized_and_extension_overflow_before_target_io() {
    for original in [
        snapshot(
            u32::MAX,
            vec![security(APS_COUNTER).to_extension().unwrap()],
        ),
        snapshot(
            NWK_COUNTER,
            vec![security(u32::MAX).to_extension().unwrap()],
        ),
    ] {
        let (ncp, mut script) = ncp(13, vec![]);
        let (handle, actor) = ncp
            .actor_builder(NonZeroUsize::MIN)
            .with_network_restore()
            .build();
        assert!(matches!(
            script.run(with_actor(
                actor,
                handle.restore_network(original, options())
            )),
            Err(Error::Backup(BackupError::CounterExhausted))
        ));
    }
}

#[test]
fn restore_rejects_live_targets_and_preserves_transport_errors() {
    for response in [
        Transaction::reply(NETWORK_STATE, &[], &[2]),
        Transaction::fail(NETWORK_STATE, &[]),
    ] {
        let (mut ncp, mut script) = ncp(13, vec![response]);
        let result = script.run(ncp.validate_restore(&backup(), options()));
        assert!(matches!(
            result,
            Err(Error::Backup(BackupError::InvalidState) | Error::Backend(_))
        ));
    }
}

#[test]
fn insufficient_tables_are_rejected_before_writes() {
    let mut small = target_prefix();
    small.extend([
        Transaction::reply(GET_CONFIGURATION, &[0x1E], &[0, 0, 0]),
        Transaction::reply(GET_CONFIGURATION, &[0x11], &[0, 0, 0]),
    ]);
    let (mut ncp, mut script) = ncp(13, small);
    assert!(matches!(
        script.run(ncp.validate_restore(&backup(), options())),
        Err(Error::Backup(BackupError::Incompatible(
            "target key or child table is too small"
        )))
    ));
}

#[test]
fn rejects_foreign_sources_and_alternate_network_key_transitions_without_io() {
    let original = backup();
    for foreign in [true, false] {
        let source = BackupSource::new(
            if foreign {
                "foreign"
            } else {
                EMBER_BACKUP_STACK
            }
            .into(),
            FIRMWARE.into(),
        )
        .unwrap();
        let keys = if foreign {
            vec![]
        } else {
            vec![NetworkKeyState::new(
                NetworkKey::new(SECRET),
                10,
                NWK_COUNTER,
            )]
        };
        let unsupported = NetworkBackup::new(
            source,
            original.network().clone(),
            original.network_key().clone(),
            keys,
            original.devices().to_vec(),
            original.extensions().to_vec(),
        )
        .unwrap();
        let (mut ncp, mut script) = ncp(13, vec![]);
        assert!(matches!(
            script.run(ncp.validate_restore(&unsupported, options())),
            Err(Error::Backup(BackupError::Incompatible(_)))
        ));
    }
}

#[test]
fn rejects_psa_unknown_storage_and_mismatched_firmware_without_writes() {
    for storage in [&[0, 1, 1][..], &[0, 1, 0xFF][..], &[0, 0][..]] {
        let (mut ncp, mut script) = ncp(
            13,
            vec![
                Transaction::reply(NETWORK_STATE, &[], &[0]),
                Transaction::reply(GET_VALUE, &[0x44], storage),
            ],
        );
        assert!(matches!(
            script.run(ncp.validate_restore(&backup(), options())),
            Err(Error::Backup(BackupError::Incompatible(_)))
        ));
    }
    let (mut ncp, mut script) = ncp(
        13,
        vec![
            Transaction::reply(NETWORK_STATE, &[], &[0]),
            Transaction::reply(GET_VALUE, &[0x44], &[0, 1, 0]),
            Transaction::reply(GET_VALUE, &[0x11], &[0, 7, 0, 0, 7, 4, 2, 0, 0]),
        ],
    );
    assert!(matches!(
        script.run(ncp.validate_restore(&backup(), options())),
        Err(Error::Backup(BackupError::Incompatible(
            "source and target firmware differ"
        )))
    ));
}

#[test]
fn identity_writability_is_checked_without_reset_or_write() {
    let mut steps = target_prefix();
    steps.pop();
    steps.extend([
        Transaction::reply(GET_EUI64, &[], &[1; 8]),
        Transaction::reply(0x000B, &[0x0C], &[8, 1, 1, 1, 1, 1, 1, 1, 1]),
    ]);
    let (mut ncp, mut script) = ncp(13, steps);
    assert!(matches!(
        script.run(ncp.validate_restore(&backup(), options())),
        Err(Error::Backup(BackupError::Incompatible(
            "custom EUI64 token cannot be overwritten"
        )))
    ));
}

#[test]
fn combined_actor_keeps_ordinary_requests_and_survives_canceled_dump_receivers() {
    let mut steps = dump_steps();
    steps.push(Transaction::reply(
        GET_EUI64,
        &[],
        &[8, 7, 6, 5, 4, 3, 2, 1],
    ));
    let (ncp, mut script) = ncp(13, steps);
    let (handle, actor) = ncp
        .actor_builder(NonZeroUsize::MIN)
        .with_network_dump()
        .with_network_restore()
        .build();
    let request = async {
        // Enqueue a dump, then drop only its response future. The actor must
        // finish the accepted operation before answering the next request.
        {
            let mut canceled = pin!(handle.dump_network());
            poll_fn(|context| {
                assert!(canceled.as_mut().poll(context).is_pending());
                Poll::Ready(())
            })
            .await;
        }
        let capabilities = handle.backup_capabilities().await.unwrap();
        assert!(capabilities.can_dump() && capabilities.can_restore());
        handle.get_ieee_address().await.unwrap()
    };
    assert_eq!(script.run(with_actor(actor, request)), IEEE);
}
