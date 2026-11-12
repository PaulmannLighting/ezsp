//! Explicit firmware layouts for offline, complete token-image backup.

use std::collections::BTreeSet;

use apis_saltans_hw::core::IeeeAddress;
use apis_saltans_hw::core::short_id::Device;
use apis_saltans_hw::{
    BackupError, BackupSource, Channel, ChannelMask, DeviceBackup, LinkKeyState, NetworkBackup,
    NetworkKey, NetworkKeyState, NetworkParameters,
};

use super::image::TokenImage;
use super::{EMBER_BACKUP_STACK, EmberSecurityState};
use crate::ember::security::initial::Bitmask;
use crate::ember::token::{Info, MAX_TOKEN_DATA_SIZE};

const KEY_LENGTH: usize = 16;
const IEEE_LENGTH: usize = 8;
const EMPTY_IEEE: [u8; IEEE_LENGTH] = [0; IEEE_LENGTH];
const ERASED_IEEE: [u8; IEEE_LENGTH] = [u8::MAX; IEEE_LENGTH];
const MAX_PROFILE_ID: usize = 64;
pub(super) const MAX_RECORDS: usize = 4096;

/// Location of a fixed-width field in a token element. Integers and IEEE addresses
/// use EZSP little-endian wire order; offsets are bytes from the element's start.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TokenField {
    /// NVM3 token key.
    pub key: u32,
    /// Array element index; zero for basic tokens.
    pub index: u32,
    /// Byte offset within the element.
    pub offset: u8,
}

/// Locations of the normalized primary-network state in an offline token image.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NetworkTokenLayout {
    /// Short PAN identifier (u16).
    pub pan_id: TokenField,
    /// Extended PAN identifier (eight little-endian bytes).
    pub extended_pan_id: TokenField,
    /// Operating channel (u8).
    pub channel: TokenField,
    /// Permitted channels (u32).
    pub channel_mask: TokenField,
    /// Network update ID (u8).
    pub update_id: TokenField,
    /// Security level (u8).
    pub security_level: TokenField,
    /// Active network key (16 bytes).
    pub key: TokenField,
    /// Active key sequence (u8).
    pub key_sequence: TokenField,
    /// Outgoing NWK security counter (u32).
    pub tx_counter: TokenField,
}

/// Locations of coordinator Trust Center state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SecurityTokenLayout {
    /// Initial-security flags (u16), including global/hashed-key mode.
    pub flags: TokenField,
    /// Trust Center key or hashed-key seed (16 bytes).
    pub key: TokenField,
    /// Global outgoing APS security counter (u32).
    pub tx_counter: TokenField,
}

/// Locations of an occupied device slot's individual link-key state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LinkKeyTokenLayout {
    /// Individual key (16 bytes).
    pub key: TokenField,
    /// Outgoing counter (u32); may alias the global APS counter exactly.
    pub tx_counter: TokenField,
    /// Incoming replay counter (u32).
    pub rx_counter: TokenField,
}

/// One persistent device slot.
///
/// All-zero/all-erased IEEE addresses denote unused slots. Every occupied slot has the configured fields, including its link key
/// when `link_key` is present. Other child/key metadata is retained as opaque bytes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeviceTokenLayout {
    /// Remote IEEE address (eight little-endian bytes).
    pub ieee: TokenField,
    /// Last short address (u16); reserved values become `None`.
    pub nwk_address: Option<TokenField>,
    /// Direct-child marker (u8, exactly zero or one); absent means false.
    pub is_child: Option<TokenField>,
    /// Individual security state, when every occupied slot in this table has a key.
    pub link_key: Option<LinkKeyTokenLayout>,
    /// Persistent incoming NWK replay counter (u32), when maintained.
    pub nwk_rx_counter: Option<TokenField>,
}

/// Opt-in layout and behavioral contract for an offline NCP firmware build.
///
/// No real firmware profile is bundled or certified. Selecting a profile asserts
/// that its entire token catalog is portable core state, including all required
/// child/key/replay/policy state, and that firmware is quiescent while `NoNetwork`.
/// Every required device slot and outgoing security counter must be mapped.
/// No queued scan, pending transition, or second transport client may mutate state.
/// Token writes/readback must access synchronous persistent storage. Startup must
/// reload token state before network activation, with matching runtime configuration.
/// Storage readback is verified; reset persistence and RAM reload require external
/// qualification and caller-owned reset/reconnection. RAM-only state, PSA handles,
/// device-specific tokens, and pending alternate-key transitions are unsupported.
/// These assumptions must be verified for a real build; matching metadata alone
/// cannot prove them. Exact firmware identity and catalog are checked at runtime.
#[derive(Clone, Debug)]
pub struct OfflineTokenProfile {
    pub(super) id: String,
    pub(super) firmware: String,
    pub(super) tokens: Vec<Info>,
    pub(super) network: NetworkTokenLayout,
    pub(super) security: SecurityTokenLayout,
    pub(super) devices: Vec<DeviceTokenLayout>,
}

impl OfflineTokenProfile {
    /// Define an explicit firmware contract and validate its layout before I/O.
    /// Use a new profile ID whenever offsets, state coverage, or firmware semantics change.
    ///
    /// # Errors
    /// Rejects invalid identifiers, duplicate tokens, excessive catalogs, out-of-range
    /// fields, and overlapping fields. Exact sharing of outgoing APS counters is allowed.
    pub fn new(
        id: String,
        firmware: String,
        tokens: Vec<Info>,
        network: NetworkTokenLayout,
        security: SecurityTokenLayout,
        devices: Vec<DeviceTokenLayout>,
    ) -> Result<Self, BackupError> {
        if id.is_empty()
            || id.len() > MAX_PROFILE_ID
            || !id
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_'))
            || firmware.trim().is_empty()
            || tokens.is_empty()
            || tokens.len() > usize::from(u8::MAX)
        {
            return Err(incompatible(
                "invalid offline token profile identity or catalog",
            ));
        }
        let mut keys = BTreeSet::new();
        let mut records = 0;
        for token in &tokens {
            if !keys.insert(token.nvm3_key())
                || token.size() == 0
                || usize::from(token.size()) > MAX_TOKEN_DATA_SIZE
                || token.array_size() == 0
                || (!token.is_idx() && token.array_size() != 1)
            {
                return Err(incompatible("invalid or duplicate token description"));
            }
            records += usize::from(token.array_size());
        }
        if records > MAX_RECORDS {
            return Err(incompatible("token catalog exceeds backup limits"));
        }
        let profile = Self {
            id,
            firmware,
            tokens,
            network,
            security,
            devices,
        };
        profile.validate_fields()?;
        Ok(profile)
    }

    /// Stable profile identifier embedded in the token extension.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    fn validate_fields(&self) -> Result<(), BackupError> {
        // The third tuple value identifies fields which may share exactly the
        // same global APS counter. Every other overlap is rejected.
        let n = &self.network;
        let s = &self.security;
        let mut fields = vec![
            (n.pan_id, size_of::<u16>(), false),
            (n.extended_pan_id, IEEE_LENGTH, false),
            (n.channel, size_of::<u8>(), false),
            (n.channel_mask, size_of::<u32>(), false),
            (n.update_id, size_of::<u8>(), false),
            (n.security_level, size_of::<u8>(), false),
            (n.key, KEY_LENGTH, false),
            (n.key_sequence, size_of::<u8>(), false),
            (n.tx_counter, size_of::<u32>(), false),
            (s.flags, size_of::<u16>(), false),
            (s.key, KEY_LENGTH, false),
            (s.tx_counter, size_of::<u32>(), true),
        ];
        for device in &self.devices {
            fields.push((device.ieee, IEEE_LENGTH, false));
            if let Some(field) = device.nwk_address {
                fields.push((field, size_of::<u16>(), false));
            }
            if let Some(field) = device.is_child {
                fields.push((field, size_of::<u8>(), false));
            }
            if let Some(field) = device.nwk_rx_counter {
                fields.push((field, size_of::<u32>(), false));
            }
            if let Some(key) = &device.link_key {
                fields.extend([
                    (key.key, KEY_LENGTH, false),
                    (key.tx_counter, size_of::<u32>(), true),
                    (key.rx_counter, size_of::<u32>(), false),
                ]);
            }
        }
        for (index, &(field, width, shared)) in fields.iter().enumerate() {
            let token = self
                .tokens
                .iter()
                .find(|token| token.nvm3_key() == field.key)
                .ok_or(incompatible("field references an unknown token"))?;
            if field.index >= u32::from(token.array_size())
                || usize::from(field.offset) + width > usize::from(token.size())
            {
                return Err(incompatible("field lies outside its token element"));
            }
            for &(other, other_width, other_shared) in &fields[..index] {
                if field.key == other.key
                    && field.index == other.index
                    && usize::from(field.offset) < usize::from(other.offset) + other_width
                    && usize::from(other.offset) < usize::from(field.offset) + width
                    && !(shared && other_shared && field == other && width == other_width)
                {
                    return Err(incompatible("token fields overlap"));
                }
            }
        }
        Ok(())
    }

    pub(super) fn snapshot(
        &self,
        image: &TokenImage,
        ieee: IeeeAddress,
    ) -> Result<NetworkBackup, BackupError> {
        image.validate(self)?;
        let n = &self.network;
        let network = NetworkParameters::new(
            ieee,
            image.read(n.pan_id)?,
            image
                .read::<crate::ember::Eui64>(n.extended_pan_id)?
                .into_array(),
            Channel::new(image.read(n.channel)?)
                .ok_or(incompatible("invalid operating channel"))?,
            ChannelMask::new(image.read(n.channel_mask)?)
                .ok_or(incompatible("invalid channel mask"))?,
            image.read(n.update_id)?,
            image.read(n.security_level)?,
        )?;
        let key = NetworkKeyState::new(
            NetworkKey::new(image.read::<[u8; KEY_LENGTH]>(n.key)?),
            image.read(n.key_sequence)?,
            image.read(n.tx_counter)?,
        );
        let security = self.read_security(image)?;
        let mut devices = Vec::new();
        for layout in &self.devices {
            let address: crate::ember::Eui64 = image.read(layout.ieee)?;
            if matches!(address.into_array(), EMPTY_IEEE | ERASED_IEEE) {
                continue;
            }
            let short = layout
                .nwk_address
                .map(|field| image.read::<u16>(field))
                .transpose()?
                .and_then(Device::new);
            let child = layout
                .is_child
                .map(|field| image.read::<u8>(field))
                .transpose()?
                .unwrap_or_default();
            if child > 1 {
                return Err(incompatible("invalid child marker"));
            }
            let link = layout
                .link_key
                .as_ref()
                .map(|layout| {
                    Ok::<_, BackupError>(LinkKeyState::new(
                        NetworkKey::new(image.read::<[u8; KEY_LENGTH]>(layout.key)?),
                        image.read(layout.tx_counter)?,
                        image.read(layout.rx_counter)?,
                    ))
                })
                .transpose()?;
            devices.push(DeviceBackup::new(
                address.into(),
                short,
                child != 0,
                link,
                layout
                    .nwk_rx_counter
                    .map(|field| image.read(field))
                    .transpose()?,
            ));
        }
        NetworkBackup::new(
            BackupSource::new(EMBER_BACKUP_STACK.into(), self.firmware.clone())?,
            network,
            key,
            vec![],
            devices,
            vec![security.to_extension()?, image.to_extension()?],
        )
    }

    pub(super) fn read_security(
        &self,
        image: &TokenImage,
    ) -> Result<EmberSecurityState, BackupError> {
        let flags = Bitmask::from_bits(image.read(self.security.flags)?)
            .ok_or(incompatible("unknown token security flags"))?;
        EmberSecurityState::new(
            flags,
            NetworkKey::new(image.read::<[u8; KEY_LENGTH]>(self.security.key)?),
            image.read(self.security.tx_counter)?,
        )
    }
}

pub(super) const fn incompatible(message: &'static str) -> BackupError {
    BackupError::Incompatible(message)
}
