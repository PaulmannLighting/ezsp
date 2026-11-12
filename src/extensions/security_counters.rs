//! Typed access to persistent outgoing security counters.

use crate::ember::network::Status;
use crate::ezsp::value::Id;
use crate::types::ByteSizedVec;
use crate::{Configuration, Error, Networking, ValueError};

const COUNTER_LENGTH: usize = size_of::<u32>();

/// Read outgoing security counters and set them on an exclusively owned offline NCP.
///
/// These are security counters, not APS correlation sequences or diagnostic
/// counters. Writes do not advance values automatically. Callers must prepare
/// safe values and prevent concurrent commands between the offline check and
/// write. Successful writes do not prove persistence across a reset.
pub trait SecurityCounters {
    /// Read the outgoing NWK security counter.
    ///
    /// # Errors
    /// Returns command failures or an error for a response other than four bytes.
    fn get_nwk_frame_counter(&mut self) -> impl Future<Output = Result<u32, Error>> + Send;

    /// Read the global outgoing APS security counter.
    ///
    /// # Errors
    /// Returns command failures or an error for a response other than four bytes.
    fn get_aps_frame_counter(&mut self) -> impl Future<Output = Result<u32, Error>> + Send;

    /// Set an already prepared outgoing NWK counter on an offline target.
    ///
    /// # Errors
    /// Rejects a joined, joining, or leaving target before writing, and forwards
    /// state-query and write failures. Exclusive ownership remains the caller's responsibility.
    fn set_nwk_frame_counter(
        &mut self,
        counter: u32,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Set an already prepared global outgoing APS counter on an offline target.
    ///
    /// # Errors
    /// Rejects a joined, joining, or leaving target before writing, and forwards
    /// state-query and write failures. Exclusive ownership remains the caller's responsibility.
    fn set_aps_frame_counter(
        &mut self,
        counter: u32,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl<T> SecurityCounters for T
where
    T: Configuration + Networking + Send,
{
    async fn get_nwk_frame_counter(&mut self) -> Result<u32, Error> {
        read(self, Id::NwkFrameCounter).await
    }

    async fn get_aps_frame_counter(&mut self) -> Result<u32, Error> {
        read(self, Id::ApsFrameCounter).await
    }

    async fn set_nwk_frame_counter(&mut self, counter: u32) -> Result<(), Error> {
        write(self, Id::NwkFrameCounter, counter).await
    }

    async fn set_aps_frame_counter(&mut self, counter: u32) -> Result<(), Error> {
        write(self, Id::ApsFrameCounter, counter).await
    }
}

async fn read<T>(target: &mut T, id: Id) -> Result<u32, Error>
where
    T: Configuration + Send,
{
    let value = target.get_value(id).await?;
    let bytes: [u8; COUNTER_LENGTH] = value
        .as_slice()
        .try_into()
        .map_err(|_| ValueError::InvalidSecurityCounterLength(value.len()))?;
    Ok(u32::from_le_bytes(bytes))
}

async fn write<T>(target: &mut T, id: Id, counter: u32) -> Result<(), Error>
where
    T: Configuration + Networking + Send,
{
    let state = target.network_state().await?;
    if state != Status::NoNetwork {
        return Err(ValueError::SecurityCounterWriteWhileOnline(state.into()).into());
    }
    let bytes = ByteSizedVec::from_slice(&counter.to_le_bytes())
        .expect("a four-byte counter fits in an EZSP value");
    target.set_value(id, bytes).await
}

#[cfg(test)]
mod tests {
    use crate::test_support::{Script, Transaction, ready};
    use crate::{Error, SecurityCounters, ValueError};

    const GET_VALUE: u16 = 0x00AA;
    const SET_VALUE: u16 = 0x00AB;
    const NETWORK_STATE: u16 = 0x0018;
    const COUNTER: u32 = 0x1234_5678;

    #[test]
    fn reads_both_counters_in_little_endian_order() {
        let mut target = Script::new([
            Transaction::reply(GET_VALUE, &[0x23], &[0, 4, 0x78, 0x56, 0x34, 0x12]),
            Transaction::reply(GET_VALUE, &[0x24], &[0, 4, 0xFF, 0xFF, 0xFF, 0xFF]),
        ]);
        assert_eq!(ready(target.get_nwk_frame_counter()).unwrap(), COUNTER);
        assert_eq!(ready(target.get_aps_frame_counter()).unwrap(), u32::MAX);
        target.assert_finished();
    }

    #[test]
    fn rejects_short_and_long_counter_values() {
        for length in [0, 1, 3, 5] {
            let mut response = vec![0, length];
            response.extend(std::iter::repeat_n(0, usize::from(length)));
            let mut target = Script::new([Transaction::reply(GET_VALUE, &[0x23], &response)]);
            assert!(matches!(ready(target.get_nwk_frame_counter()),
                Err(Error::ValueError(ValueError::InvalidSecurityCounterLength(actual))) if actual == usize::from(length)));
            target.assert_finished();
        }
    }

    #[test]
    fn writes_prepared_values_without_advancing_them() {
        let mut target = Script::new([
            Transaction::reply(NETWORK_STATE, &[], &[0]),
            Transaction::reply(SET_VALUE, &[0x23, 4, 0x78, 0x56, 0x34, 0x12], &[0]),
            Transaction::reply(NETWORK_STATE, &[], &[0]),
            Transaction::reply(SET_VALUE, &[0x24, 4, 0xFF, 0xFF, 0xFF, 0xFF], &[0]),
        ]);
        ready(target.set_nwk_frame_counter(COUNTER)).unwrap();
        ready(target.set_aps_frame_counter(u32::MAX)).unwrap();
        target.assert_finished();
    }

    #[test]
    fn rejects_every_non_offline_state_without_writing() {
        for state in [1, 2, 3, 4] {
            for aps in [false, true] {
                let mut target = Script::new([Transaction::reply(NETWORK_STATE, &[], &[state])]);
                let result = if aps {
                    ready(target.set_aps_frame_counter(COUNTER))
                } else {
                    ready(target.set_nwk_frame_counter(COUNTER))
                };
                assert!(matches!(result,
                    Err(Error::ValueError(ValueError::SecurityCounterWriteWhileOnline(actual))) if actual == state));
                target.assert_finished();
            }
        }
    }

    #[test]
    fn propagates_read_state_and_write_failures() {
        let mut target = Script::new([Transaction::fail(GET_VALUE, &[0x24])]);
        assert!(matches!(
            ready(target.get_aps_frame_counter()),
            Err(Error::ChannelClosed)
        ));
        target.assert_finished();
        let mut target = Script::new([Transaction::fail(NETWORK_STATE, &[])]);
        assert!(matches!(
            ready(target.set_nwk_frame_counter(COUNTER)),
            Err(Error::ChannelClosed)
        ));
        target.assert_finished();
        let mut target = Script::new([
            Transaction::reply(NETWORK_STATE, &[], &[0]),
            Transaction::reply(SET_VALUE, &[0x24, 4, 0x78, 0x56, 0x34, 0x12], &[0x31]),
        ]);
        assert!(matches!(
            ready(target.set_aps_frame_counter(COUNTER)),
            Err(Error::Status(_))
        ));
        target.assert_finished();
    }
}
