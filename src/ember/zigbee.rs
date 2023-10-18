use std::array::IntoIter;
use std::io::Read;
use std::iter::{once, Chain, Once};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Network {
    channel: u8,
    pan_id: u16,
    extended_pan_id: u64,
    allowing_join: bool,
    stack_profile: u8,
    nwk_update_id: u8,
}

impl Network {
    #[must_use]
    pub const fn new(
        channel: u8,
        pan_id: u16,
        extended_pan_id: u64,
        allowing_join: bool,
        stack_profile: u8,
        nwk_update_id: u8,
    ) -> Self {
        Self {
            channel,
            pan_id,
            extended_pan_id,
            allowing_join,
            stack_profile,
            nwk_update_id,
        }
    }

    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }

    #[must_use]
    pub const fn pan_id(&self) -> u16 {
        self.pan_id
    }

    #[must_use]
    pub const fn extended_pan_id(&self) -> u64 {
        self.extended_pan_id
    }

    #[must_use]
    pub const fn allowing_join(&self) -> bool {
        self.allowing_join
    }

    #[must_use]
    pub const fn stack_profile(&self) -> u8 {
        self.stack_profile
    }

    #[must_use]
    pub const fn nwk_update_id(&self) -> u8 {
        self.nwk_update_id
    }

    /// Reads the struct from a reader
    ///
    /// # Errors
    /// Returns an [`std::io::Error`] on read errors.
    pub fn read_from<R>(src: &mut R) -> std::io::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [channel] = [0; 1];
        src.read_exact(&mut buffer)?;
        let mut pan_id = [0; 2];
        src.read_exact(&mut pan_id)?;
        let mut extended_pan_id = [0; 8];
        src.read_exact(&mut extended_pan_id)?;
        let mut buffer @ [allowing_join, stack_profile, nwk_update_id] = [0; 3];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            channel,
            pan_id: u16::from_be_bytes(pan_id),
            extended_pan_id: u64::from_be_bytes(extended_pan_id),
            allowing_join: allowing_join != 0,
            stack_profile,
            nwk_update_id,
        })
    }
}

impl IntoIterator for Network {
    type Item = u8;
    type IntoIter = Chain<
        Chain<
            Chain<
                Chain<Chain<Once<Self::Item>, IntoIter<Self::Item, 2>>, IntoIter<Self::Item, 8>>,
                Once<Self::Item>,
            >,
            IntoIter<Self::Item, 1>,
        >,
        IntoIter<Self::Item, 1>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        once(self.channel)
            .chain(self.pan_id.to_be_bytes())
            .chain(self.extended_pan_id.to_be_bytes())
            .chain(once(self.allowing_join.into()))
            .chain(self.stack_profile.to_be_bytes())
            .chain(self.nwk_update_id.to_be_bytes())
    }
}
