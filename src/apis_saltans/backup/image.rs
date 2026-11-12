//! Bounded opaque token-image encoding; all secret-bearing bytes are redacted.

use apis_saltans_hw::{BackupError, BackupExtension};
use le_stream::FromLeStream;

use super::EMBER_BACKUP_NAMESPACE;
use super::profile::{MAX_RECORDS, OfflineTokenProfile, TokenField, incompatible};
use crate::ember::token::{Data, MAX_TOKEN_DATA_SIZE};

pub(super) const TOKEN_EXTENSION: &str = "tokens";
const TOKEN_VERSION: u32 = 1;
const MAX_IMAGE_BYTES: usize = 256 * 1024;
const MAX_PROFILE_ID: usize = 64;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct TokenRecord {
    pub key: u32,
    pub index: u32,
    pub data: Data,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct TokenImage {
    pub id: String,
    pub records: Vec<TokenRecord>,
}

impl TokenImage {
    pub fn validate(&self, profile: &OfflineTokenProfile) -> Result<(), BackupError> {
        if self.id != profile.id {
            return Err(incompatible("offline token profile ID differs"));
        }
        let expected = profile.tokens.iter().flat_map(|token| {
            (0..u32::from(token.array_size()))
                .map(move |index| (token.nvm3_key(), index, token.size()))
        });
        let mut records = self.records.iter();
        for (key, index, size) in expected {
            let record = records
                .next()
                .ok_or(incompatible("token image is incomplete"))?;
            if record.key != key || record.index != index || record.data.size() != u32::from(size) {
                return Err(incompatible(
                    "token image does not match the firmware catalog",
                ));
            }
        }
        if records.next().is_some() {
            return Err(incompatible("token image contains extra records"));
        }
        Ok(())
    }

    pub fn read<T>(&self, field: TokenField) -> Result<T, BackupError>
    where
        T: FromLeStream,
    {
        let record = self
            .records
            .iter()
            .find(|record| record.key == field.key && record.index == field.index)
            .ok_or(incompatible("token field is absent"))?;
        let bytes = record
            .data
            .as_slice()
            .get(usize::from(field.offset)..)
            .ok_or(incompatible("token field offset is invalid"))?;
        T::from_le_stream(bytes.iter().copied()).ok_or(incompatible("token field is truncated"))
    }

    pub fn advance(&mut self, field: TokenField, amount: u32) -> Result<(), BackupError> {
        let value: u32 = self.read(field)?;
        let value = value
            .checked_add(amount)
            .filter(|value| *value < u32::MAX)
            .ok_or(BackupError::CounterExhausted)?;
        let record = self
            .records
            .iter_mut()
            .find(|record| record.key == field.key && record.index == field.index)
            .ok_or(incompatible("counter token is absent"))?;
        let mut bytes = *record.data.data();
        let start = usize::from(field.offset);
        bytes
            .get_mut(start..start + size_of::<u32>())
            .ok_or(incompatible("counter field is truncated"))?
            .copy_from_slice(&value.to_le_bytes());
        record.data = Data::new(record.data.size(), bytes);
        Ok(())
    }

    pub fn to_extension(&self) -> Result<BackupExtension, BackupError> {
        let id_length =
            u8::try_from(self.id.len()).map_err(|_| incompatible("profile ID is too long"))?;
        let count = u16::try_from(self.records.len())
            .map_err(|_| incompatible("too many token records"))?;
        let mut payload = vec![id_length];
        payload.extend(self.id.bytes());
        payload.extend(count.to_le_bytes());
        for record in &self.records {
            payload.extend(record.key.to_le_bytes());
            payload.extend(record.index.to_le_bytes());
            payload.push(
                u8::try_from(record.data.size()).map_err(|_| incompatible("oversized token"))?,
            );
            payload.extend(record.data.as_slice());
        }
        if payload.len() > MAX_IMAGE_BYTES {
            return Err(incompatible("token image exceeds size limit"));
        }
        BackupExtension::new(
            EMBER_BACKUP_NAMESPACE.into(),
            TOKEN_EXTENSION.into(),
            TOKEN_VERSION,
            payload,
        )
    }

    pub fn decode(extension: &BackupExtension) -> Result<Self, BackupError> {
        if extension.namespace() != EMBER_BACKUP_NAMESPACE
            || extension.name() != TOKEN_EXTENSION
            || extension.version() != TOKEN_VERSION
            || extension.payload().len() > MAX_IMAGE_BYTES
        {
            return Err(incompatible("unknown or oversized token extension"));
        }
        let mut bytes = extension.payload().iter().copied();
        let id_length = usize::from(bytes.next().ok_or(incompatible("missing profile ID"))?);
        if id_length == 0 || id_length > MAX_PROFILE_ID {
            return Err(incompatible("invalid profile ID length"));
        }
        let id: Vec<u8> = bytes.by_ref().take(id_length).collect();
        if id.len() != id_length {
            return Err(incompatible("truncated profile ID"));
        }
        let id = String::from_utf8(id).map_err(|_| incompatible("profile ID is not UTF-8"))?;
        let count = usize::from(
            u16::from_le_stream(&mut bytes).ok_or(incompatible("missing token count"))?,
        );
        if count > MAX_RECORDS {
            return Err(incompatible("too many token records"));
        }
        let mut records = Vec::with_capacity(count);
        for _ in 0..count {
            let key = u32::from_le_stream(&mut bytes).ok_or(incompatible("missing token key"))?;
            let index =
                u32::from_le_stream(&mut bytes).ok_or(incompatible("missing token index"))?;
            let size = bytes.next().ok_or(incompatible("missing token length"))?;
            if size == 0 || usize::from(size) > MAX_TOKEN_DATA_SIZE {
                return Err(incompatible("invalid token length"));
            }
            let mut data = [0; MAX_TOKEN_DATA_SIZE];
            for value in &mut data[..usize::from(size)] {
                *value = bytes.next().ok_or(incompatible("truncated token data"))?;
            }
            records.push(TokenRecord {
                key,
                index,
                data: Data::new(u32::from(size), data),
            });
        }
        if bytes.next().is_some() {
            return Err(incompatible("trailing token image bytes"));
        }
        Ok(Self { id, records })
    }
}
