use std::collections::BTreeMap;
use std::time::Duration;

use apis_saltans_hw::{BackupError, Error, RestoreFailureState, RestoreOptions};
use tokio::time::{sleep, timeout};

use super::capture::{active_key, trust_center};
use super::{
    EmberSecurityState, RecoveryBackup, RecoveryReport, read_link_keys, table_indices, unchanged,
};
use crate::ember::{self, join, key, network, node, security};
use crate::ezsp::config;
use crate::frame::parameters::security::add_or_update_key_table_entry;
use crate::{Communicate, Configuration, Networking, Security, SecurityCounters, Utilities};

const COORDINATOR_ID: u16 = 0;
const FORMATION_TIMEOUT: Duration = Duration::from_secs(10);
const STATE_POLL_INTERVAL: Duration = Duration::from_millis(50);
const READBACK_MISMATCH: &str = "EZSP v8 restore readback did not match";

pub(super) async fn restore<T>(
    target: &mut T,
    backup: &RecoveryBackup,
    options: RestoreOptions,
    radio_tx_power: i8,
) -> Result<RecoveryReport, Error>
where
    T: Communicate,
{
    let snapshot = backup
        .snapshot()
        .prepare_restore(options)
        .map_err(unchanged)?;
    let mut security = RecoveryBackup::security(&snapshot).map_err(unchanged)?;
    security
        .advance(options.link_counter_advance().get())
        .map_err(unchanged)?;
    let identity = snapshot.network();
    let coordinator = identity.coordinator_ieee().into();
    let keys: BTreeMap<ember::Eui64, &key::Data> = snapshot
        .devices()
        .iter()
        .filter_map(|device| {
            Some((
                device.ieee_address().into(),
                device.link_key()?.key().expose_secret(),
            ))
        })
        .collect();
    let nwk_floor = snapshot.network_key().tx_counter();
    let aps_floor = snapshot
        .devices()
        .iter()
        .filter_map(|device| device.link_key())
        .map(apis_saltans_hw::LinkKeyState::tx_counter)
        .fold(security.aps_tx_counter(), u32::max);

    validate_target(target, coordinator, identity.security_level(), keys.len())
        .await
        .map_err(unchanged)?;

    let parameters = network::Parameters::new(
        identity.extended_pan_id().into(),
        identity.pan_id(),
        radio_tx_power,
        identity.channel().as_u8(),
        join::Method::MacAssociation,
        COORDINATOR_ID,
        identity.nwk_update_id(),
        identity.channel_mask().bits(),
    );
    let active = snapshot.network_key();
    let initial_security = security::initial::State::new(
        security.flags() | security::initial::Bitmask::NO_FRAME_COUNTER_RESET,
        *security.trust_center_key().expose_secret(),
        *active.key().expose_secret(),
        active.sequence_number(),
        coordinator,
    );

    apply(
        target,
        initial_security,
        &keys,
        parameters.clone(),
        nwk_floor,
        aps_floor,
    )
    .await
    .map_err(possibly_modified)?;
    verify(
        target,
        coordinator,
        &security,
        &keys,
        &parameters,
        active,
        aps_floor,
    )
    .await
    .map_err(possibly_modified)
}

async fn validate_target<T>(
    target: &mut T,
    coordinator: ember::Eui64,
    level: u8,
    key_count: usize,
) -> Result<(), Error>
where
    T: Communicate,
{
    if target.network_state().await? != network::Status::NoNetwork {
        return Err(BackupError::InvalidState.into());
    }
    if target.get_eui64().await? != coordinator {
        return Err(BackupError::Incompatible("target coordinator IEEE address differs").into());
    }
    if target
        .get_configuration_value(config::Id::SecurityLevel)
        .await?
        != u16::from(level)
    {
        return Err(BackupError::Incompatible("target security level differs").into());
    }
    if table_indices(target, config::Id::KeyTableSize)
        .await?
        .count()
        < key_count
    {
        return Err(BackupError::Incompatible("target link-key table is too small").into());
    }
    Ok(())
}

async fn apply<T>(
    target: &mut T,
    security: security::initial::State,
    keys: &BTreeMap<ember::Eui64, &key::Data>,
    parameters: network::Parameters,
    nwk_floor: u32,
    aps_floor: u32,
) -> Result<(), Error>
where
    T: Communicate,
{
    target.set_nwk_frame_counter(nwk_floor).await?;
    target.set_aps_frame_counter(aps_floor).await?;
    target.set_initial_security_state(security).await?;
    target.clear_key_table().await?;
    for (&partner, &key) in keys {
        let response = target
            .communicate(add_or_update_key_table_entry::Command::link_key(
                partner, *key,
            ))
            .await?;
        <()>::try_from(response)?;
    }
    target.form_network(parameters).await?;
    if target.network_state().await? == network::Status::JoinedNetwork {
        return Ok(());
    }
    timeout(FORMATION_TIMEOUT, wait_for_network(target))
        .await
        .map_err(|_| BackupError::Incompatible("EZSP v8 network formation timed out"))?
}

async fn wait_for_network<T>(target: &mut T) -> Result<(), Error>
where
    T: Networking + Send,
{
    loop {
        match target.network_state().await? {
            network::Status::JoinedNetwork => return Ok(()),
            network::Status::NoNetwork | network::Status::JoiningNetwork => {
                sleep(STATE_POLL_INTERVAL).await;
            }
            _ => return Err(BackupError::InvalidState.into()),
        }
    }
}

async fn verify<T>(
    target: &mut T,
    coordinator: ember::Eui64,
    security: &EmberSecurityState,
    keys: &BTreeMap<ember::Eui64, &key::Data>,
    requested: &network::Parameters,
    network_key: &apis_saltans_hw::NetworkKeyState,
    aps_floor: u32,
) -> Result<RecoveryReport, Error>
where
    T: Communicate,
{
    let (role, observed) = target.get_network_parameters().await?;
    if role != node::Type::Coordinator
        || target.get_eui64().await? != coordinator
        || observed.pan_id() != requested.pan_id()
        || observed.extended_pan_id() != requested.extended_pan_id()
        || observed.radio_channel() != requested.radio_channel()
    {
        return Err(BackupError::Incompatible(READBACK_MISMATCH).into());
    }
    let active = active_key(target).await?;
    let (flags, tc_key) = trust_center(target).await?;
    let hashed = security::initial::Bitmask::TRUST_CENTER_USES_HASHED_LINK_KEY;
    if active.key() != network_key.key().expose_secret()
        || active.sequence_number() != network_key.sequence_number()
        || active.outgoing_frame_counter() < network_key.tx_counter()
        || active.outgoing_frame_counter() == u32::MAX
        || flags.contains(hashed) != security.flags().contains(hashed)
        || tc_key.key() != security.trust_center_key().expose_secret()
        || tc_key.outgoing_frame_counter() < aps_floor
        || tc_key.outgoing_frame_counter() == u32::MAX
    {
        return Err(BackupError::Incompatible(READBACK_MISMATCH).into());
    }
    let readback = read_link_keys(target).await?;
    let mut verified = BTreeMap::new();
    for entry in &readback {
        if entry.typ() != Ok(key::Type::ApplicationLinkKey)
            || !entry.has_flag(key::Bitmask::HasPartnerEui64)
            || verified
                .insert(entry.partner_eui64(), entry.key())
                .is_some()
        {
            return Err(BackupError::Incompatible(READBACK_MISMATCH).into());
        }
    }
    if &verified != keys {
        return Err(BackupError::Incompatible(READBACK_MISMATCH).into());
    }
    Ok(RecoveryReport {
        nwk_tx_counter: active.outgoing_frame_counter(),
        aps_tx_counter: tc_key.outgoing_frame_counter(),
        restored_link_keys: verified.len(),
        requested_nwk_update_id: requested.nwk_update_id(),
        observed_nwk_update_id: observed.nwk_update_id(),
    })
}

fn possibly_modified(error: Error) -> Error {
    BackupError::restore_failed(RestoreFailureState::PossiblyModified, error).into()
}
