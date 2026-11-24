use std::collections::{BTreeMap, BTreeSet};

use apis_saltans_hw::core::{IeeeAddress, short_id};
use apis_saltans_hw::{
    BackupError, BackupSource, Channel, ChannelMask, DeviceBackup, Error, LinkKeyState,
    NetworkBackup, NetworkKey, NetworkKeyState, NetworkParameters,
};

use super::{
    BASE_SECURITY_FLAGS, EmberSecurityState, INVALID_KEY, STACK, read_link_keys, table_indices,
    valid_ieee, validate_key, validate_snapshot,
};
use crate::ember::{self, key, network, node, security};
use crate::ezsp::config;
use crate::{Communicate, Configuration, GetValueExt, Messaging, Networking, Security, Utilities};

#[derive(Default)]
struct Device {
    short: Option<short_id::Device>,
    child: bool,
    key: Option<LinkKeyState>,
}

pub(in crate::apis_saltans::backup) async fn capture<T>(
    target: &mut T,
) -> Result<NetworkBackup, Error>
where
    T: Communicate,
{
    if target.network_state().await? != network::Status::JoinedNetwork {
        return Err(BackupError::InvalidState.into());
    }
    let ieee = target.get_eui64().await?;
    let (typ, parameters) = target.get_network_parameters().await?;
    if typ != node::Type::Coordinator {
        return Err(BackupError::InvalidState.into());
    }
    let firmware = target
        .get_ember_version()
        .await?
        .map_err(|_| BackupError::Incompatible("malformed Ember firmware version"))?;
    let level = target
        .get_configuration_value(config::Id::SecurityLevel)
        .await?;
    let settings = network_parameters(ieee, &parameters, level)?;
    let initial_key = active_key(target).await?;
    let mut devices = read_devices(target, ieee).await?;
    let keys = read_link_keys(target).await?;
    let (flags, tc_key) = trust_center(target).await?;
    let aps_counter = add_link_keys(&mut devices, keys, ieee, &tc_key)?;
    let final_key = active_key(target).await?;
    if initial_key.sequence_number() != final_key.sequence_number()
        || initial_key.key() != final_key.key()
        || initial_key.outgoing_frame_counter() > final_key.outgoing_frame_counter()
        || target.get_network_parameters().await? != (typ, parameters)
        || target.network_state().await? != network::Status::JoinedNetwork
    {
        return Err(BackupError::Incompatible("network changed during EZSP v8 capture").into());
    }
    let security = EmberSecurityState::new(flags, NetworkKey::new(*tc_key.key()), aps_counter)?;
    let devices = devices
        .into_iter()
        .map(|(ieee, device)| DeviceBackup::new(ieee, device.short, device.child, device.key, None))
        .collect();
    let snapshot = NetworkBackup::new(
        BackupSource::new(STACK.into(), firmware.to_string())?,
        settings,
        NetworkKeyState::new(
            NetworkKey::new(*final_key.key()),
            final_key.sequence_number(),
            final_key.outgoing_frame_counter(),
        ),
        Vec::new(),
        devices,
        vec![security.to_extension()?],
    )?
    .with_rejoin_coverage();
    validate_snapshot(&snapshot)?;
    Ok(snapshot)
}

fn network_parameters(
    ieee: ember::Eui64,
    parameters: &network::Parameters,
    level: u16,
) -> Result<NetworkParameters, BackupError> {
    let channel =
        Channel::new(parameters.radio_channel()).ok_or(BackupError::InvalidNetworkParameters)?;
    let mut channel_mask =
        ChannelMask::new(parameters.channels()).ok_or(BackupError::InvalidNetworkParameters)?;
    // The formation API may return an empty preferred mask for the coordinator.
    if channel_mask == ChannelMask::empty() {
        channel_mask = std::iter::once(channel).collect();
    }
    let level = u8::try_from(level).map_err(|_| BackupError::InvalidNetworkParameters)?;
    NetworkParameters::new(
        ieee.into(),
        parameters.pan_id(),
        parameters.extended_pan_id().into_array(),
        channel,
        channel_mask,
        parameters.nwk_update_id(),
        level,
    )
}

#[allow(deprecated, reason = "getKey is the EZSP v8 legacy security API")]
pub(super) async fn active_key<T>(target: &mut T) -> Result<key::Struct, Error>
where
    T: Security + Send,
{
    let key = target.get_key(key::Type::CurrentNetworkKey).await?;
    if key.typ() != Ok(key::Type::CurrentNetworkKey)
        || !key.has_flag(key::Bitmask::HasSequenceNumber)
        || !key.has_flag(key::Bitmask::HasOutgoingFrameCounter)
    {
        return Err(BackupError::Incompatible(INVALID_KEY).into());
    }
    validate_key(key.key())?;
    Ok(key)
}

#[allow(deprecated, reason = "getKey is the EZSP v8 legacy security API")]
pub(super) async fn trust_center<T>(
    target: &mut T,
) -> Result<(security::initial::Bitmask, key::Struct), Error>
where
    T: Security + Send,
{
    use security::{current, initial};

    let mode = target.get_current_security_state().await?.bitmask();
    if mode.contains(current::Bitmask::DISTRIBUTED_TRUST_CENTER_MODE)
        || !mode.contains(current::Bitmask::GLOBAL_LINK_KEY)
    {
        return Err(BackupError::Incompatible("unsupported EZSP v8 Trust Center mode").into());
    }
    let mut flags = BASE_SECURITY_FLAGS;
    if mode.contains(current::Bitmask::TRUST_CENTER_USES_HASHED_LINK_KEY) {
        flags.insert(initial::Bitmask::TRUST_CENTER_USES_HASHED_LINK_KEY);
    }
    let key = target.get_key(key::Type::TrustCenterLinkKey).await?;
    if key.typ() != Ok(key::Type::TrustCenterLinkKey)
        || !key.has_flag(key::Bitmask::HasOutgoingFrameCounter)
    {
        return Err(BackupError::Incompatible(INVALID_KEY).into());
    }
    validate_key(key.key())?;
    Ok((flags, key))
}

async fn read_devices<T>(
    target: &mut T,
    coordinator: ember::Eui64,
) -> Result<BTreeMap<IeeeAddress, Device>, Error>
where
    T: Communicate,
{
    let mut devices = BTreeMap::new();
    let mut child_shorts = BTreeSet::new();
    for index in u8::MIN..=u8::MAX {
        match target.get_child_data(index).await {
            Ok(child) => {
                let (Some(short), ieee) = (short_id::Device::new(child.id()), child.eui64()) else {
                    continue;
                };
                if ieee == coordinator || !valid_ieee(ieee) {
                    continue;
                }
                if !child_shorts.insert(short)
                    || devices
                        .insert(
                            ieee.into(),
                            Device {
                                short: Some(short),
                                child: true,
                                key: None,
                            },
                        )
                        .is_some()
                {
                    return Err(BackupError::DuplicateDevice.into());
                }
            }
            Err(crate::Error::Status(crate::error::Status::Ember(Ok(
                ember::Status::NotJoined,
            )))) => (),
            Err(crate::Error::Status(crate::error::Status::Ember(Ok(
                ember::Status::IndexOutOfRange,
            )))) => break,
            Err(error) => return Err(error.into()),
        }
    }

    let mut hints = BTreeMap::new();
    for index in table_indices(target, config::Id::AddressTableSize).await? {
        let Some(short) =
            short_id::Device::new(target.get_address_table_remote_node_id(index).await?)
        else {
            continue;
        };
        let ieee = target.get_address_table_remote_eui64(index).await?;
        if ieee == coordinator || !valid_ieee(ieee) || devices.contains_key(&ieee.into()) {
            continue;
        }
        let candidates: &mut BTreeSet<_> = hints.entry(IeeeAddress::from(ieee)).or_default();
        candidates.insert(short);
    }
    let mut counts = BTreeMap::new();
    for short in hints.values().flatten() {
        *counts.entry(*short).or_insert(0usize) += 1;
    }
    for (ieee, candidates) in hints {
        let short = if candidates.len() == 1 {
            candidates
                .into_iter()
                .next()
                .filter(|short| !child_shorts.contains(short) && counts.get(short) == Some(&1))
        } else {
            None
        };
        devices.insert(
            ieee,
            Device {
                short,
                ..Device::default()
            },
        );
    }
    Ok(devices)
}

fn add_link_keys(
    devices: &mut BTreeMap<IeeeAddress, Device>,
    keys: Vec<key::Struct>,
    coordinator: ember::Eui64,
    tc_key: &key::Struct,
) -> Result<u32, BackupError> {
    let mut aps_counter = tc_key.outgoing_frame_counter();
    for key in keys {
        let partner = key.partner_eui64();
        if key.typ() != Ok(key::Type::ApplicationLinkKey)
            || !key.has_flag(key::Bitmask::HasPartnerEui64)
            || partner == coordinator
            || !valid_ieee(partner)
        {
            return Err(BackupError::Incompatible(INVALID_KEY));
        }
        validate_key(key.key())?;
        let tx = metadata_counter(
            &key,
            key::Bitmask::HasOutgoingFrameCounter,
            key::Struct::outgoing_frame_counter,
        );
        let rx = metadata_counter(
            &key,
            key::Bitmask::HasIncomingFrameCounter,
            key::Struct::incoming_frame_counter,
        );
        aps_counter = aps_counter.max(tx);
        let device = devices.entry(partner.into()).or_default();
        let link = LinkKeyState::new(NetworkKey::new(*key.key()), tx, rx);
        if device.key.replace(link).is_some() {
            return Err(BackupError::DuplicateDevice);
        }
    }
    Ok(aps_counter)
}

fn metadata_counter(
    key: &key::Struct,
    flag: key::Bitmask,
    counter: impl FnOnce(&key::Struct) -> u32,
) -> u32 {
    if key.has_flag(flag) {
        counter(key)
    } else {
        u32::MIN
    }
}
