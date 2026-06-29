use le_stream::FromLeStream;
use log::warn;

/// Payload that is resilient against size mismatches,
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ResilientPayload {
    size: u8,
    payload: Box<[u8]>,
}

impl ResilientPayload {
    /// Returns the size.
    #[must_use]
    pub const fn size(&self) -> u8 {
        self.size
    }

    /// Returns the payload.
    #[must_use]
    pub fn into_payload(self) -> Box<[u8]> {
        self.payload
    }
}

impl AsRef<[u8]> for ResilientPayload {
    fn as_ref(&self) -> &[u8] {
        &self.payload
    }
}

impl FromLeStream for ResilientPayload {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let size = u8::from_le_stream(&mut bytes)?;
        let payload = Box::<[u8]>::from_le_stream(bytes)?;

        if usize::from(size) != payload.len() {
            warn!("Size mismatch: expected {size}, got {}", payload.len());
        }

        Some(Self { size, payload })
    }
}
