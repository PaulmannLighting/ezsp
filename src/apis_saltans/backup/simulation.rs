//! Stateful fake NCP: command replies are derived from persistent token storage.

use std::future::poll_fn;
use std::num::NonZeroU8;
use std::pin::pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};

use apis_saltans_hw::{
    BackupError, Driver, Error as HardwareError, NetworkDump, NetworkRestore, RestoreFailureState,
};
use le_stream::{FromLeStream, ToLeStream};
use tokio::sync::{RwLock, mpsc};

use super::image::{TokenImage, TokenRecord};
use super::profile::{
    DeviceTokenLayout, LinkKeyTokenLayout, NetworkTokenLayout, OfflineTokenProfile,
    SecurityTokenLayout, TokenField,
};
use super::tests::{backup, options, with_actor};
use crate::api::Message;
use crate::ember::aps::Options;
use crate::ember::security::initial::Bitmask;
use crate::ember::token::{Data, Info};
use crate::test_support::ready;
use crate::{Connection, Error, Ncp, Parameters, Parsable, Utilities};

const CAPACITY: usize = 32;
const POLL_LIMIT: usize = 8192;
const NETWORK: u32 = 0x1001;
const SECURITY: u32 = 0x1002;
const DEVICES: u32 = 0x1003;
const ADDRESS: crate::ember::Eui64 = crate::ember::Eui64::new(1, 2, 3, 4, 5, 6, 7, 8);
const OTHER_ADDRESS: crate::ember::Eui64 = crate::ember::Eui64::new(8, 7, 6, 5, 4, 3, 2, 1);
const FIRMWARE: &str = "7.4.2-4660 (0 / 0)";

fn field(key: u32, index: u32, offset: u8) -> TokenField {
    TokenField { key, index, offset }
}

fn profile(shared_counter: bool) -> OfflineTokenProfile {
    let devices = (0..2)
        .map(|index| DeviceTokenLayout {
            ieee: field(DEVICES, index, 0),
            nwk_address: Some(field(DEVICES, index, 8)),
            is_child: Some(field(DEVICES, index, 10)),
            link_key: Some(LinkKeyTokenLayout {
                key: field(DEVICES, index, 11),
                tx_counter: if shared_counter {
                    field(SECURITY, 0, 18)
                } else {
                    field(DEVICES, index, 27)
                },
                rx_counter: field(DEVICES, index, 31),
            }),
            nwk_rx_counter: Some(field(DEVICES, index, 35)),
        })
        .collect();
    OfflineTokenProfile::new(
        "simulated-v13-offline-v1".into(),
        FIRMWARE.into(),
        vec![
            Info::new(NETWORK, false, false, 40, 1),
            Info::new(SECURITY, false, false, 24, 1),
            Info::new(DEVICES, false, true, 40, 2),
        ],
        NetworkTokenLayout {
            pan_id: field(NETWORK, 0, 0),
            extended_pan_id: field(NETWORK, 0, 2),
            channel: field(NETWORK, 0, 10),
            channel_mask: field(NETWORK, 0, 11),
            update_id: field(NETWORK, 0, 15),
            security_level: field(NETWORK, 0, 16),
            key: field(NETWORK, 0, 17),
            key_sequence: field(NETWORK, 0, 33),
            tx_counter: field(NETWORK, 0, 34),
        },
        SecurityTokenLayout {
            flags: field(SECURITY, 0, 0),
            key: field(SECURITY, 0, 2),
            tx_counter: field(SECURITY, 0, 18),
        },
        devices,
    )
    .unwrap()
}

fn record(key: u32, index: u32, bytes: &[u8]) -> TokenRecord {
    let mut storage = [0; 64];
    storage[..bytes.len()].copy_from_slice(bytes);
    TokenRecord {
        key,
        index,
        data: Data::new(u32::try_from(bytes.len()).unwrap(), storage),
    }
}

struct Model {
    catalog: Vec<Info>,
    image: TokenImage,
    address: crate::ember::Eui64,
    custom: crate::ember::Eui64,
    state: u8,
    writes: usize,
    fail_write: Option<usize>,
    corrupt_readback: bool,
    change_capture: bool,
    reads: usize,
    commands: Vec<u16>,
    capacities: u16,
    deferred_identity: bool,
}

impl Model {
    fn source(profile: &OfflineTokenProfile) -> Self {
        let mut network = vec![0x34, 0x12];
        network.extend(ADDRESS.to_le_stream());
        network.push(11);
        network.extend(0x07FF_F800_u32.to_le_bytes());
        network.extend([7, 5]);
        network.extend([0xAB; 16]);
        network.push(9);
        network.extend(100_u32.to_le_bytes());
        network.extend([0xBA, 0xDC]);
        let flags = Bitmask::HAVE_NETWORK_KEY
            | Bitmask::HAVE_PRECONFIGURED_KEY
            | Bitmask::TRUST_CENTER_USES_HASHED_LINK_KEY;
        let mut security = flags.bits().to_le_bytes().to_vec();
        security.extend([0xEF; 16]);
        security.extend(200_u32.to_le_bytes());
        security.extend([0x55, 0x66]);
        let mut device: Vec<_> = OTHER_ADDRESS.to_le_stream().collect();
        device.extend(0x3344_u16.to_le_bytes());
        device.push(1);
        device.extend([0xCD; 16]);
        device.extend(300_u32.to_le_bytes());
        device.extend(400_u32.to_le_bytes());
        device.extend(500_u32.to_le_bytes());
        device.push(0x99);
        Self {
            catalog: profile.tokens.clone(),
            image: TokenImage {
                id: profile.id.clone(),
                records: vec![
                    record(NETWORK, 0, &network),
                    record(SECURITY, 0, &security),
                    record(DEVICES, 0, &device),
                    record(DEVICES, 1, &[0xFF; 40]),
                ],
            },
            address: ADDRESS,
            custom: crate::ember::Eui64::from([0xFF; 8]),
            state: 0,
            writes: 0,
            fail_write: None,
            corrupt_readback: false,
            change_capture: false,
            reads: 0,
            commands: vec![],
            capacities: 8,
            deferred_identity: false,
        }
    }

    fn target(profile: &OfflineTokenProfile) -> Self {
        let mut model = Self::source(profile);
        model.address = OTHER_ADDRESS;
        for token in &mut model.image.records {
            token.data = Data::new(token.data.size(), [0; 64]);
        }
        model
    }

    #[expect(
        clippy::panic_in_result_fn,
        clippy::unwrap_in_result,
        reason = "Simulated firmware asserts the exact wire contract"
    )]
    fn reply(&mut self, command: crate::Commands) -> Result<Parameters, Error> {
        let id = command.id();
        let request: Vec<_> = command.to_le_stream().collect();
        self.commands.push(id);
        let response = match id {
            0x0018 => vec![self.state],
            0x00AA => match request.as_slice() {
                [0x44] => vec![0, 1, 0],
                [0x11] => vec![0, 7, 0x34, 0x12, 7, 4, 2, 0, 0],
                _ => panic!("unexpected getValue"),
            },
            0x0052 => {
                assert!(matches!(request.as_slice(), [0x1E | 0x11]));
                std::iter::once(0)
                    .chain(self.capacities.to_le_bytes())
                    .collect()
            }
            0x0026 => self.address.to_le_stream().collect(),
            0x000B => {
                assert_eq!(request, [0x0C]);
                std::iter::once(8)
                    .chain(self.custom.to_le_stream())
                    .collect()
            }
            0x000C => {
                assert_eq!(&request[..2], &[0x0C, 8]);
                self.custom =
                    crate::ember::Eui64::from_le_stream_exact(request[2..].iter().copied())
                        .unwrap();
                if !self.deferred_identity {
                    self.address = self.custom;
                }
                vec![0]
            }
            0x0100 => vec![u8::try_from(self.catalog.len()).unwrap()],
            0x0101 => std::iter::once(0)
                .chain(self.catalog[usize::from(request[0])].clone().to_le_stream())
                .collect(),
            0x0102 | 0x0103 => {
                let mut bytes = request.into_iter();
                let key = u32::from_le_stream(&mut bytes).unwrap();
                let index = u32::from_le_stream(&mut bytes).unwrap();
                let record_count = self.image.records.len();
                let token = self
                    .image
                    .records
                    .iter_mut()
                    .find(|record| record.key == key && record.index == index)
                    .unwrap();
                if id == 0x0103 {
                    self.writes += 1;
                    token.data = Data::from_le_stream_exact(bytes).unwrap();
                    if self.fail_write == Some(self.writes) {
                        return Err(Error::ChannelClosed);
                    }
                    vec![0]
                } else {
                    self.reads += 1;
                    if self.change_capture && self.reads > record_count {
                        let mut data = *token.data.data();
                        data[0] ^= 1;
                        token.data = Data::new(token.data.size(), data);
                    }
                    let mut data: Vec<_> = std::iter::once(0)
                        .chain(token.data.clone().to_le_stream())
                        .collect();
                    if self.corrupt_readback && self.writes > 0 {
                        data[5] ^= 1;
                    }
                    data
                }
            }
            _ => {
                panic!("unexpected command {id:#06X}; network activation and resets are forbidden")
            }
        };
        Parameters::parse_from_le_stream(id, response.into_iter()).map_err(Into::into)
    }
}

struct Simulator {
    inbox: mpsc::Receiver<Message>,
    model: Model,
}

impl Simulator {
    fn ncp(model: Model, profile: OfflineTokenProfile) -> (Ncp, Self) {
        let (handle, inbox) = mpsc::channel(CAPACITY);
        let connection = Connection {
            desired_version: NonZeroU8::new(13).unwrap(),
            handle,
            gate: Arc::new(RwLock::new(())),
        };
        let (events, _receiver) = mpsc::channel(CAPACITY);
        let ncp = ready(Ncp::new(connection, Box::new([]), events, Options::NONE))
            .unwrap()
            .with_offline_token_backup(profile);
        (ncp, Self { inbox, model })
    }

    fn run<F>(&mut self, future: F) -> F::Output
    where
        F: Future,
    {
        let mut future = pin!(future);
        let mut context = Context::from_waker(Waker::noop());
        for _ in 0..POLL_LIMIT {
            if let Poll::Ready(output) = future.as_mut().poll(&mut context) {
                return output;
            }
            match self.inbox.try_recv() {
                Ok(Message::Command {
                    command,
                    response,
                    guard,
                }) => {
                    response.send(self.model.reply(command)).unwrap();
                    drop(guard);
                }
                Ok(_) => panic!("unexpected actor message"),
                Err(_) => {}
            }
        }
        panic!("simulated operation stalled");
    }
}

fn captured(profile: &OfflineTokenProfile) -> apis_saltans_hw::NetworkBackup {
    let (mut ncp, mut simulator) = Simulator::ncp(Model::source(profile), profile.clone());
    let backup = simulator.run(ncp.dump_network()).unwrap();
    assert_eq!(simulator.model.writes, 0);
    backup
}

#[test]
fn capture_and_restore_through_hardware_actors_preserves_complete_image_and_advances_counters() {
    let profile = profile(false);
    let (source, mut simulator) = Simulator::ncp(Model::source(&profile), profile.clone());
    let (handle, actor) = source
        .actor_builder(std::num::NonZeroUsize::MIN)
        .with_network_dump()
        .build();
    let backup = simulator
        .run(with_actor(actor, handle.dump_network()))
        .unwrap();
    assert_eq!(backup.network().extended_pan_id(), ADDRESS.into_array());
    assert_eq!(backup.network().nwk_update_id(), 7);
    assert_eq!(backup.network_key().sequence_number(), 9);
    assert_eq!(backup.devices().len(), 1);
    assert_eq!(backup.devices()[0].nwk_rx_counter(), Some(500));
    let (target, mut simulator) = Simulator::ncp(Model::target(&profile), profile.clone());
    let (handle, actor) = target
        .actor_builder(std::num::NonZeroUsize::MIN)
        .with_network_restore()
        .build();
    let report = simulator
        .run(with_actor(
            actor,
            handle.restore_network(backup.clone(), options()),
        ))
        .unwrap();
    assert_eq!(report.coordinator_ieee(), ADDRESS.into());
    assert_eq!(report.network_counters(), &[(9, 110)]);
    assert_eq!(report.link_counters(), &[(OTHER_ADDRESS.into(), 320)]);
    assert_eq!(
        simulator
            .model
            .image
            .read::<u32>(profile.network.tx_counter)
            .unwrap(),
        110
    );
    assert_eq!(
        simulator
            .model
            .image
            .read::<u32>(profile.security.tx_counter)
            .unwrap(),
        220
    );
    assert_eq!(
        simulator
            .model
            .image
            .read::<u32>(field(DEVICES, 0, 31))
            .unwrap(),
        400
    );
    assert_eq!(
        simulator
            .model
            .image
            .read::<u32>(field(DEVICES, 0, 35))
            .unwrap(),
        500
    );
    assert_eq!(
        simulator.model.image.records[0].data.as_slice()[38..],
        [0xBA, 0xDC]
    );
    assert_eq!(
        simulator.model.image.records[1].data.as_slice()[22..],
        [0x55, 0x66]
    );
    assert_eq!(simulator.model.image.records[2].data.as_slice()[39], 0x99);
    assert_eq!(
        simulator.model.image.records[3].data.as_slice(),
        &[0xFF; 40]
    );
    assert_eq!(simulator.model.state, 0);
    assert_eq!(simulator.model.address, ADDRESS);
    assert_eq!(simulator.model.writes, 4);
    assert_eq!(backup.network_key().tx_counter(), 100);
}

#[test]
fn shared_global_and_device_aps_counter_is_advanced_only_once() {
    let profile = profile(true);
    let backup = captured(&profile);
    assert_eq!(backup.devices()[0].link_key().unwrap().tx_counter(), 200);
    let (target, mut simulator) = Simulator::ncp(Model::target(&profile), profile.clone());
    let (handle, actor) = target
        .actor_builder(std::num::NonZeroUsize::MIN)
        .with_network_restore()
        .build();
    let report = simulator
        .run(with_actor(actor, handle.restore_network(backup, options())))
        .unwrap();
    assert_eq!(report.link_counters(), &[(OTHER_ADDRESS.into(), 220)]);
    assert_eq!(
        simulator
            .model
            .image
            .read::<u32>(profile.security.tx_counter)
            .unwrap(),
        220
    );
}

#[test]
fn dropped_restore_receiver_does_not_interrupt_apply_and_recapture() {
    let profile = profile(false);
    let backup = captured(&profile);
    let (target, mut simulator) = Simulator::ncp(Model::target(&profile), profile);
    let (handle, actor) = target
        .actor_builder(std::num::NonZeroUsize::MIN)
        .with_network_dump()
        .with_network_restore()
        .build();
    let request = async {
        {
            let mut canceled = pin!(handle.restore_network(backup, options()));
            poll_fn(|context| {
                assert!(canceled.as_mut().poll(context).is_pending());
                Poll::Ready(())
            })
            .await;
        }
        handle.dump_network().await
    };
    let restored = simulator.run(with_actor(actor, request)).unwrap();
    assert_eq!(restored.network_key().tx_counter(), 110);
    let device = &restored.devices()[0];
    assert_eq!(device.link_key().unwrap().tx_counter(), 320);
    assert_eq!(device.link_key().unwrap().rx_counter(), 400);
    assert_eq!(device.nwk_rx_counter(), Some(500));
    assert_eq!(simulator.model.writes, 4);
    assert_eq!(simulator.model.state, 0);
}

#[test]
fn failed_or_corrupt_persistent_write_never_returns_a_success_report() {
    let profile = profile(false);
    let backup = captured(&profile);
    for corrupt in [false, true] {
        let mut model = Model::target(&profile);
        model.address = ADDRESS;
        model.corrupt_readback = corrupt;
        model.fail_write = (!corrupt).then_some(2);
        let (mut target, mut simulator) = Simulator::ncp(model, profile.clone());
        let prepared = backup.prepare_restore(options()).unwrap();
        let result = simulator.run(target.restore_network(prepared, options()));
        assert!(matches!(
            result,
            Err(HardwareError::Backup(BackupError::RestoreFailed {
                state: RestoreFailureState::PossiblyModified,
                ..
            }))
        ));
        assert_eq!(simulator.model.state, 0);
        assert!(simulator.model.writes > 0);
    }
}

#[test]
fn live_capture_and_unstable_offline_capture_are_rejected() {
    for live in [false, true] {
        let profile = profile(false);
        let mut model = Model::source(&profile);
        model.state = u8::from(live) * 2;
        model.change_capture = !live;
        let (mut ncp, mut simulator) = Simulator::ncp(model, profile);
        assert!(simulator.run(ncp.dump_network()).is_err());
        assert_eq!(simulator.model.writes, 0);
    }
}

#[test]
fn missing_image_wrong_catalog_and_normalized_token_disagreement_fail_before_writes() {
    let profile = profile(false);
    let original = captured(&profile);
    let inconsistent = apis_saltans_hw::NetworkBackup::new(
        original.source().clone(),
        original.network().clone(),
        apis_saltans_hw::NetworkKeyState::new(apis_saltans_hw::NetworkKey::new([1; 16]), 9, 100),
        vec![],
        original.devices().to_vec(),
        original.extensions().to_vec(),
    )
    .unwrap();
    for (index, backup) in [backup(), inconsistent, original].into_iter().enumerate() {
        let mut model = Model::target(&profile);
        if index == 2 {
            model.catalog[0] = Info::new(NETWORK, false, false, 39, 1);
        }
        let (mut target, mut simulator) = Simulator::ncp(model, profile.clone());
        let prepared = backup.prepare_restore(options()).unwrap();
        assert!(
            simulator
                .run(target.restore_network(prepared, options()))
                .is_err()
        );
        assert_eq!(simulator.model.writes, 0);
    }
}

#[test]
fn maintenance_session_blocks_borrowed_commands_until_drop() {
    let profile = profile(false);
    let (ncp, mut simulator) = Simulator::ncp(Model::source(&profile), profile);
    let mut connection = ncp.connection();
    let session = ready(ncp.connection.maintenance());
    {
        let mut operation = pin!(connection.get_eui64());
        let mut context = Context::from_waker(Waker::noop());
        assert!(operation.as_mut().poll(&mut context).is_pending());
        assert!(simulator.inbox.try_recv().is_err());
        drop(session);
        assert_eq!(simulator.run(operation).unwrap(), ADDRESS);
    }
    let mut owned = ncp.connection().clone();
    drop(ncp);
    assert_eq!(simulator.run(owned.get_eui64()).unwrap(), ADDRESS);
}

#[test]
fn deferred_identity_requires_explicit_reset_and_never_writes_network_tokens() {
    let profile = profile(false);
    let backup = captured(&profile).prepare_restore(options()).unwrap();
    let mut model = Model::target(&profile);
    model.deferred_identity = true;
    let (mut target, mut simulator) = Simulator::ncp(model, profile);
    assert!(matches!(
        simulator.run(target.restore_network(backup, options())),
        Err(HardwareError::Backup(BackupError::RestoreFailed {
            state: RestoreFailureState::PossiblyModified,
            ..
        }))
    ));
    assert_eq!(simulator.model.custom, ADDRESS);
    assert_eq!(simulator.model.address, OTHER_ADDRESS);
    assert_eq!(simulator.model.writes, 0);
    assert_eq!(simulator.model.state, 0);
}

#[test]
fn occupied_identity_and_insufficient_capacity_are_rejected_without_mutation() {
    let profile = profile(false);
    let backup = captured(&profile).prepare_restore(options()).unwrap();
    for occupied in [false, true] {
        let mut model = Model::target(&profile);
        if occupied {
            model.custom = OTHER_ADDRESS;
        } else {
            model.capacities = 0;
        }
        let (mut target, mut simulator) = Simulator::ncp(model, profile.clone());
        assert!(matches!(
            simulator.run(target.restore_network(backup.clone(), options())),
            Err(HardwareError::Backup(BackupError::RestoreFailed {
                state: RestoreFailureState::Unchanged,
                ..
            }))
        ));
        assert!(!simulator.model.commands.contains(&0x000C));
        assert_eq!(simulator.model.writes, 0);
    }
}

#[test]
fn malformed_token_extensions_and_layouts_are_rejected() {
    let profile = profile(false);
    let model = Model::source(&profile);
    let extension = model.image.to_extension().unwrap();
    assert_eq!(TokenImage::decode(&extension).unwrap(), model.image);
    // Every truncation is invalid; record count and field widths are mandatory.
    for end in 0..extension.payload().len() {
        let truncated = apis_saltans_hw::BackupExtension::new(
            extension.namespace().into(),
            extension.name().into(),
            extension.version(),
            extension.payload()[..end].to_vec(),
        )
        .unwrap();
        assert!(TokenImage::decode(&truncated).is_err());
    }
    let mut records = model.image;
    records.records.swap(0, 1);
    assert!(records.validate(&profile).is_err());
    for invalid in [
        field(NETWORK, 0, 0),
        field(NETWORK, 0, 39),
        field(NETWORK, 1, 34),
        field(0, 0, 0),
    ] {
        let mut network = profile.network.clone();
        network.tx_counter = invalid;
        assert!(
            OfflineTokenProfile::new(
                profile.id.clone(),
                profile.firmware.clone(),
                profile.tokens.clone(),
                network,
                profile.security.clone(),
                profile.devices.clone(),
            )
            .is_err()
        );
    }
}

#[test]
fn embedded_counter_exhaustion_fails_before_target_io() {
    let profile = profile(false);
    let mut model = Model::source(&profile);
    // The APS extension counter is separate from normalized device counters.
    model
        .image
        .advance(profile.security.tx_counter, u32::MAX - 202)
        .unwrap();
    let (mut source, mut simulator) = Simulator::ncp(model, profile.clone());
    let backup = simulator.run(source.dump_network()).unwrap();
    let prepared = backup.prepare_restore(options()).unwrap();
    let (mut target, mut simulator) = Simulator::ncp(Model::target(&profile), profile);
    assert!(
        simulator
            .run(target.restore_network(prepared, options()))
            .is_err()
    );
    assert_eq!(simulator.model.commands, Vec::<u16>::new());
}
