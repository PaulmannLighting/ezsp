use std::num::NonZeroUsize;

use apis_saltans_hw::{
    BackupCoverage, BackupError, Driver, Error, NetworkBackup, RestoreFailureState,
    RestoreNetworkState, RestoreOptions,
};

use super::super::tests::{ncp, options, with_actor};
use crate::test_support::Transaction;

const EZSP_V8: u8 = 8;
const ADDRESS_EUI64: u16 = 0x005E;
const ADDRESS_NODE_ID: u16 = 0x005F;
const ADD_LINK_KEY: u16 = 0x0066;
const CLEAR_KEY_TABLE: u16 = 0x00B1;
const FORM_NETWORK: u16 = 0x001E;
const GET_CHILD_DATA: u16 = 0x004A;
const GET_CONFIGURATION: u16 = 0x0052;
const GET_EUI64: u16 = 0x0026;
const GET_KEY: u16 = 0x006A;
const GET_KEY_TABLE_ENTRY: u16 = 0x0071;
const GET_NETWORK_PARAMETERS: u16 = 0x0028;
const GET_SECURITY: u16 = 0x0069;
const GET_VALUE: u16 = 0x00AA;
const NETWORK_STATE: u16 = 0x0018;
const SET_SECURITY: u16 = 0x0068;
const SET_VALUE: u16 = 0x00AB;

const COORDINATOR: [u8; 8] = [8, 7, 6, 5, 4, 3, 2, 1];
const CHILD: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
const LINK_ONLY: [u8; 8] = [2, 4, 6, 8, 1, 3, 5, 7];
const ADDRESS_ONLY: [u8; 8] = [3, 6, 7, 8, 1, 2, 4, 5];
const EXT_PAN: [u8; 8] = [1, 1, 2, 2, 3, 3, 4, 4];
const PAN_ID: [u8; 2] = [0x34, 0x12];
const CHANNEL_MASK: [u8; 4] = [0x00, 0xF8, 0xFF, 0x07];
const NWK_KEY: [u8; 16] = [0x11; 16];
const TC_KEY: [u8; 16] = [0x22; 16];
const PARTNER_KEY: [u8; 16] = [0x33; 16];
const NWK_SEQUENCE: u8 = 5;
const NWK_COUNTER: u32 = 100;
const APS_COUNTER: u32 = 300;
const PARTNER_COUNTER: u32 = 400;
const NWK_FLOOR: u32 = NWK_COUNTER + 10;
const APS_FLOOR: u32 = PARTNER_COUNTER + 20;
const CHILD_SHORT: u16 = 0x1122;
const ADDRESS_SHORT: u16 = 0x2233;
const UPDATE_ID: u8 = 7;
const TX_POWER: i8 = 8;

fn network_parameters(update_id: u8) -> Vec<u8> {
    let mut payload = EXT_PAN.to_vec();
    payload.extend(PAN_ID);
    payload.extend([TX_POWER.to_le_bytes()[0], 15, 0, 0, 0, update_id]);
    payload.extend(CHANNEL_MASK);
    payload
}

fn observed_parameters(update_id: u8) -> Transaction {
    let mut payload = vec![0, 1];
    payload.extend(network_parameters(update_id));
    Transaction::reply(GET_NETWORK_PARAMETERS, &[], &payload)
}

fn key_response(typ: u8, flags: u16, key: [u8; 16], tx: u32, partner: [u8; 8]) -> Vec<u8> {
    let mut payload = vec![0];
    payload.extend(flags.to_le_bytes());
    payload.push(typ);
    payload.extend(key);
    payload.extend(tx.to_le_bytes());
    payload.extend(19u32.to_le_bytes());
    payload.push(NWK_SEQUENCE);
    payload.extend(partner);
    payload
}

fn active_key(counter: u32) -> Transaction {
    Transaction::reply(
        GET_KEY,
        &[3],
        &key_response(3, 0x0003, NWK_KEY, counter, [0; 8]),
    )
}

fn trust_center(counter: u32) -> [Transaction; 2] {
    let mut status = vec![0, 0x94, 0];
    status.extend(COORDINATOR);
    [
        Transaction::reply(GET_SECURITY, &[], &status),
        Transaction::reply(
            GET_KEY,
            &[1],
            &key_response(1, 0x0002, TC_KEY, counter, [0xFF; 8]),
        ),
    ]
}

fn key_table() -> [Transaction; 3] {
    let mut erased = [0; 37];
    erased[0] = 0xB6;
    [
        Transaction::reply(GET_CONFIGURATION, &[0x1E], &[0, 2, 0]),
        Transaction::reply(GET_KEY_TABLE_ENTRY, &[0], &erased),
        Transaction::reply(
            GET_KEY_TABLE_ENTRY,
            &[1],
            &key_response(5, 0x001E, PARTNER_KEY, PARTNER_COUNTER, LINK_ONLY),
        ),
    ]
}

fn source() -> Vec<Transaction> {
    let mut child = vec![0];
    child.extend(CHILD);
    child.push(4);
    child.extend(CHILD_SHORT.to_le_bytes());
    child.extend([0; 3]);
    let mut gap = [0; 15];
    gap[0] = 0x93;
    let mut end = [0; 15];
    end[0] = 0xB1;
    let mut transactions = vec![
        Transaction::reply(NETWORK_STATE, &[], &[2]),
        Transaction::reply(GET_EUI64, &[], &COORDINATOR),
        observed_parameters(UPDATE_ID),
        Transaction::reply(GET_VALUE, &[0x11], &[0, 7, 0x34, 0x12, 6, 7, 10, 0, 0]),
        Transaction::reply(GET_CONFIGURATION, &[0x0D], &[0, 5, 0]),
        active_key(NWK_COUNTER - 1),
        Transaction::reply(GET_CHILD_DATA, &[0], &gap),
        Transaction::reply(GET_CHILD_DATA, &[1], &child),
        Transaction::reply(GET_CHILD_DATA, &[2], &end),
        Transaction::reply(GET_CONFIGURATION, &[0x05], &[0, 3, 0]),
        Transaction::reply(ADDRESS_NODE_ID, &[0], &[0xFF, 0xFF]),
        Transaction::reply(ADDRESS_NODE_ID, &[1], &[0x99, 0x88]),
        Transaction::reply(ADDRESS_EUI64, &[1], &CHILD),
        Transaction::reply(ADDRESS_NODE_ID, &[2], &ADDRESS_SHORT.to_le_bytes()),
        Transaction::reply(ADDRESS_EUI64, &[2], &ADDRESS_ONLY),
    ];
    transactions.extend(key_table());
    transactions.extend(trust_center(APS_COUNTER));
    transactions.extend([
        active_key(NWK_COUNTER),
        observed_parameters(UPDATE_ID),
        Transaction::reply(NETWORK_STATE, &[], &[2]),
    ]);
    transactions
}

fn preflight(capacity: u8) -> [Transaction; 4] {
    [
        Transaction::reply(NETWORK_STATE, &[], &[0]),
        Transaction::reply(GET_EUI64, &[], &COORDINATOR),
        Transaction::reply(GET_CONFIGURATION, &[0x0D], &[0, 5, 0]),
        Transaction::reply(GET_CONFIGURATION, &[0x1E], &[0, capacity, 0]),
    ]
}

fn restore_writes(import_status: u8) -> Vec<Transaction> {
    let mut security = vec![0xC4, 0x1B];
    security.extend(TC_KEY);
    security.extend(NWK_KEY);
    security.push(NWK_SEQUENCE);
    security.extend(COORDINATOR);
    let mut add_link_key = LINK_ONLY.to_vec();
    add_link_key.push(1);
    add_link_key.extend(PARTNER_KEY);
    let mut nwk_counter = vec![0x23, 4];
    nwk_counter.extend(NWK_FLOOR.to_le_bytes());
    let mut aps_counter = vec![0x24, 4];
    aps_counter.extend(APS_FLOOR.to_le_bytes());
    vec![
        Transaction::reply(NETWORK_STATE, &[], &[0]),
        Transaction::reply(SET_VALUE, &nwk_counter, &[0]),
        Transaction::reply(NETWORK_STATE, &[], &[0]),
        Transaction::reply(SET_VALUE, &aps_counter, &[0]),
        Transaction::reply(SET_SECURITY, &security, &[0]),
        Transaction::reply(CLEAR_KEY_TABLE, &[], &[0]),
        Transaction::reply(ADD_LINK_KEY, &add_link_key, &[import_status]),
    ]
}

fn capture() -> NetworkBackup {
    let (driver, mut script) = ncp(EZSP_V8, source());
    let (handle, actor) = driver
        .actor_builder(NonZeroUsize::MIN)
        .with_network_dump()
        .build();
    script
        .run(with_actor(actor, handle.dump_network()))
        .unwrap()
}

fn restore_script(steps: impl IntoIterator<Item = Transaction>) -> Vec<Transaction> {
    let mut transactions = preflight(2).into_iter().collect::<Vec<_>>();
    // The trait checks again under its exclusive write session.
    transactions.extend(preflight(2));
    transactions.extend(steps);
    transactions
}

fn restore(
    backup: NetworkBackup,
    options: RestoreOptions,
    transactions: Vec<Transaction>,
) -> Result<apis_saltans_hw::RestoreReport, Error> {
    let (mut driver, mut script) = ncp(EZSP_V8, transactions);
    driver.restore_tx_power = TX_POWER;
    let (handle, actor) = driver
        .actor_builder(NonZeroUsize::MIN)
        .with_network_restore()
        .build();
    script.run(with_actor(actor, handle.restore_network(backup, options)))
}

#[test]
fn captures_union_and_restores_v8_through_the_generic_actor() {
    let backup = capture();
    assert_eq!(backup.coverage(), BackupCoverage::Rejoin);
    assert_eq!(backup.network().extended_pan_id(), [4, 4, 3, 3, 2, 2, 1, 1]);
    assert_eq!(backup.network_key().tx_counter(), NWK_COUNTER);
    assert_eq!(backup.devices().len(), 3);
    let child = backup
        .devices()
        .iter()
        .find(|device| device.is_child())
        .unwrap();
    assert_eq!(child.nwk_address().unwrap().as_u16(), CHILD_SHORT);
    let linked = backup
        .devices()
        .iter()
        .find(|device| device.link_key().is_some())
        .unwrap();
    assert_eq!(linked.nwk_address(), None);
    assert_eq!(linked.link_key().unwrap().rx_counter(), 19);
    assert!(backup.devices().iter().any(|device| {
        device
            .nwk_address()
            .is_some_and(|short| short.as_u16() == ADDRESS_SHORT)
    }));
    let mut transactions = restore_script(restore_writes(0));
    transactions.extend([
        Transaction::reply(FORM_NETWORK, &network_parameters(UPDATE_ID), &[0]),
        Transaction::reply(NETWORK_STATE, &[], &[2]),
        observed_parameters(0),
        Transaction::reply(GET_EUI64, &[], &COORDINATOR),
        active_key(NWK_FLOOR + 1),
    ]);
    transactions.extend(trust_center(APS_FLOOR + 1));
    transactions.extend(key_table());
    let report = restore(backup, options().allow_rejoin(), transactions).unwrap();
    assert_eq!(report.coverage(), BackupCoverage::Rejoin);
    assert_eq!(report.network_state(), RestoreNetworkState::Running);
    assert_eq!(report.network_counters(), &[(NWK_SEQUENCE, NWK_FLOOR + 1)]);
    assert_eq!(report.shared_aps_counter(), Some(APS_FLOOR + 1));
    assert_eq!(report.restored_link_keys(), 1);
    assert_eq!(report.nwk_update_id(), 0);
}

#[test]
fn rejects_missing_opt_in_and_insufficient_capacity_before_writing() {
    let backup = capture();
    assert!(matches!(
        restore(backup.clone(), options(), vec![]),
        Err(Error::Backup(BackupError::RejoinRequiresOptIn))
    ));
    assert!(matches!(
        restore(backup, options().allow_rejoin(), preflight(0).into()),
        Err(Error::Backup(BackupError::Incompatible(_)))
    ));
}

#[test]
fn failed_link_import_never_forms_the_network() {
    let backup = capture();
    let transactions = restore_script(restore_writes(0xB4));
    assert!(matches!(
        restore(backup, options().allow_rejoin(), transactions),
        Err(Error::Backup(BackupError::RestoreFailed {
            state: RestoreFailureState::PossiblyModified,
            ..
        }))
    ));
}

#[test]
fn rejects_a_network_counter_rollback_during_live_readback() {
    let backup = capture();
    let mut transactions = restore_script(restore_writes(0));
    transactions.extend([
        Transaction::reply(FORM_NETWORK, &network_parameters(UPDATE_ID), &[0]),
        Transaction::reply(NETWORK_STATE, &[], &[2]),
        observed_parameters(UPDATE_ID),
        Transaction::reply(GET_EUI64, &[], &COORDINATOR),
        active_key(NWK_FLOOR - 1),
    ]);
    transactions.extend(trust_center(APS_FLOOR));
    assert!(matches!(
        restore(backup, options().allow_rejoin(), transactions),
        Err(Error::Backup(BackupError::RestoreFailed {
            state: RestoreFailureState::PossiblyModified,
            ..
        }))
    ));
}
