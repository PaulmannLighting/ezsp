//! Checked custom-EUI64 writes, following the zsmartsystems EZSP implementation.

use std::fmt::{self, Display, Formatter};
use std::io::{self, ErrorKind};

use le_stream::{FromLeStream, ToLeStream};

use crate::ember::Eui64;
use crate::ember::network::Status;
use crate::ezsp::mfg_token::{Id, Mfg};
use crate::types::ByteSizedVec;
use crate::{Error, Networking, Utilities};

const EUI64_LENGTH: usize = 8;
const EMPTY_ADDRESS: [u8; EUI64_LENGTH] = [0; EUI64_LENGTH];
const ERASED_TOKEN: [u8; EUI64_LENGTH] = [u8::MAX; EUI64_LENGTH];
const CUSTOM_EUI64: Id = Id::Mfg(Mfg::CustomEui64);

/// Result of an explicit identity maintenance request, without starting the network.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdentityChange {
    /// The operational address already matched; storage was not written.
    Unchanged,

    /// The token and operational address match; reset persistence is not yet verified.
    Applied,

    /// The token matches, but the operational address still requires a reset.
    RestartRequired,
}

/// Failure boundary of an identity maintenance request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdentityStage {
    /// Only reads have occurred.
    Preflight,

    /// A persistent write was attempted.
    Write,

    /// A write was acknowledged and subsequent verification failed.
    Readback,
}

/// Identity maintenance failure retaining its backend error and mutation boundary.
#[derive(Debug, thiserror::Error)]
#[error("identity maintenance failed during {stage}: {source}")]
pub struct IdentityError {
    stage: IdentityStage,
    #[source]
    source: Error,
}

impl IdentityError {
    /// Stage at which maintenance failed.
    #[must_use]
    pub const fn stage(&self) -> IdentityStage {
        self.stage
    }

    /// Whether a persistent write may have changed the target.
    #[must_use]
    pub const fn storage_may_have_changed(&self) -> bool {
        !matches!(self.stage, IdentityStage::Preflight)
    }

    const fn new(stage: IdentityStage, source: Error) -> Self {
        Self { stage, source }
    }
}

impl Display for IdentityStage {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Preflight => "preflight",
            Self::Write => "write",
            Self::Readback => "readback",
        })
    }
}

/// Read and restore a custom IEEE address using standard manufacturing-token commands.
///
/// Callers must exclusively own the NCP and keep it offline throughout maintenance.
/// No factory reset, network leave, reboot, or automatic startup is performed.
/// Firmware which uses a restored-EUI64 stack token needs a different profile.
pub trait Identity {
    /// Read the custom manufacturing address in display byte order.
    ///
    /// # Errors
    /// Forwards command failures and rejects responses whose length is not eight bytes.
    fn get_custom_eui64(&mut self) -> impl Future<Output = Result<Eui64, Error>> + Send;

    /// Restore an address, avoiding redundant writes and refusing to overwrite a non-erased token.
    ///
    /// Verifies the token after writing and checks the operational address.
    /// `Applied` does not establish persistence across reset; `RestartRequired`
    /// explicitly defers operational verification until caller-owned reconnection.
    ///
    /// # Errors
    /// Rejects non-offline targets and unwritable tokens before mutation. Failures
    /// retain their stage, including possible mutation on a lost write response.
    fn set_custom_eui64(
        &mut self,
        address: Eui64,
    ) -> impl Future<Output = Result<IdentityChange, IdentityError>> + Send;
}

impl<T> Identity for T
where
    T: Utilities + Networking + Send,
{
    async fn get_custom_eui64(&mut self) -> Result<Eui64, Error> {
        let data = self.get_mfg_token(CUSTOM_EUI64).await?;
        if data.len() != EUI64_LENGTH {
            return Err(invalid("custom EUI64 token must contain eight bytes"));
        }
        Eui64::from_le_stream(data.into_iter()).ok_or_else(|| invalid("invalid custom EUI64 token"))
    }

    async fn set_custom_eui64(&mut self, address: Eui64) -> Result<IdentityChange, IdentityError> {
        if let Some(result) = preflight(self, address)
            .await
            .map_err(|error| IdentityError::new(IdentityStage::Preflight, error))?
        {
            return Ok(result);
        }
        write(self, address).await
    }
}

async fn preflight<T>(target: &mut T, address: Eui64) -> Result<Option<IdentityChange>, Error>
where
    T: Utilities + Networking + Send,
{
    if address.into_array() == EMPTY_ADDRESS || address.into_array() == ERASED_TOKEN {
        return Err(invalid("cannot restore an unspecified IEEE address"));
    }
    if target.network_state().await? != Status::NoNetwork {
        return Err(invalid("identity restoration requires an offline NCP"));
    }
    if target.get_eui64().await? == address {
        return Ok(Some(IdentityChange::Unchanged));
    }
    let token = target.get_custom_eui64().await?;
    if token == address {
        return Ok(Some(IdentityChange::RestartRequired));
    }
    if token.into_array() != ERASED_TOKEN {
        return Err(invalid(
            "custom EUI64 token is not erased; factory reset is not performed implicitly",
        ));
    }
    Ok(None)
}

async fn write<T>(target: &mut T, address: Eui64) -> Result<IdentityChange, IdentityError>
where
    T: Utilities + Networking + Send,
{
    let data: ByteSizedVec<u8> = address.to_le_stream().collect();
    target
        .set_mfg_token(CUSTOM_EUI64, data)
        .await
        .map_err(|error| IdentityError::new(IdentityStage::Write, error))?;
    let token = target
        .get_custom_eui64()
        .await
        .map_err(|error| IdentityError::new(IdentityStage::Readback, error))?;
    if token != address {
        return Err(IdentityError::new(
            IdentityStage::Readback,
            invalid("custom EUI64 readback mismatch"),
        ));
    }
    let active = target
        .get_eui64()
        .await
        .map_err(|error| IdentityError::new(IdentityStage::Readback, error))?;
    Ok(if active == address {
        IdentityChange::Applied
    } else {
        IdentityChange::RestartRequired
    })
}

fn invalid(message: &'static str) -> Error {
    io::Error::new(ErrorKind::InvalidData, message).into()
}

#[cfg(test)]
mod tests {
    use super::{Identity, IdentityChange, IdentityStage};
    use crate::ember::Eui64;
    use crate::test_support::{Script, Transaction, ready};

    const ADDRESS: Eui64 = Eui64::new(1, 2, 3, 4, 5, 6, 7, 8);
    const GET_EUI64: u16 = 0x0026;
    const GET_MFG_TOKEN: u16 = 0x000B;
    const SET_MFG_TOKEN: u16 = 0x000C;
    const NETWORK_STATE: u16 = 0x0018;
    const CURRENT: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8];
    const DESIRED: &[u8] = &[8, 7, 6, 5, 4, 3, 2, 1];
    const TOKEN: &[u8] = &[8, 8, 7, 6, 5, 4, 3, 2, 1];
    const ERASED: &[u8] = &[8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    const WRITE: &[u8] = &[0x0C, 8, 8, 7, 6, 5, 4, 3, 2, 1];

    fn prefix() -> Vec<Transaction> {
        vec![
            Transaction::reply(NETWORK_STATE, &[], &[0]),
            Transaction::reply(GET_EUI64, &[], CURRENT),
            Transaction::reply(GET_MFG_TOKEN, &[0x0C], ERASED),
        ]
    }

    #[test]
    fn skips_an_already_active_identity_without_reading_or_writing_tokens() {
        let mut target = Script::new([
            Transaction::reply(NETWORK_STATE, &[], &[0]),
            Transaction::reply(GET_EUI64, &[], DESIRED),
        ]);
        assert_eq!(
            ready(target.set_custom_eui64(ADDRESS)).unwrap(),
            IdentityChange::Unchanged
        );
        target.assert_finished();
    }

    #[test]
    fn custom_token_uses_protocol_order_and_detects_required_restarts() {
        for (active, expected) in [
            (DESIRED, IdentityChange::Applied),
            (CURRENT, IdentityChange::RestartRequired),
        ] {
            let mut steps = prefix();
            steps.extend([
                Transaction::reply(SET_MFG_TOKEN, WRITE, &[0]),
                Transaction::reply(GET_MFG_TOKEN, &[0x0C], TOKEN),
                Transaction::reply(GET_EUI64, &[], active),
            ]);
            let mut target = Script::new(steps);
            assert_eq!(ready(target.set_custom_eui64(ADDRESS)).unwrap(), expected);
            target.assert_finished();
        }
    }

    #[test]
    fn matching_token_awaiting_reset_is_not_written_twice() {
        let mut target = Script::new([
            Transaction::reply(NETWORK_STATE, &[], &[0]),
            Transaction::reply(GET_EUI64, &[], CURRENT),
            Transaction::reply(GET_MFG_TOKEN, &[0x0C], TOKEN),
        ]);
        assert_eq!(
            ready(target.set_custom_eui64(ADDRESS)).unwrap(),
            IdentityChange::RestartRequired
        );
        target.assert_finished();
    }

    #[test]
    fn rejects_live_targets_before_identity_access() {
        for state in [1, 2, 3, 4] {
            let mut target = Script::new([Transaction::reply(NETWORK_STATE, &[], &[state])]);
            let error = ready(target.set_custom_eui64(ADDRESS)).unwrap_err();
            assert_eq!(error.stage(), IdentityStage::Preflight);
            assert!(!error.storage_may_have_changed());
            target.assert_finished();
        }
    }

    #[test]
    fn rejects_sentinel_addresses_without_io() {
        for address in [Eui64::default(), Eui64::from([u8::MAX; 8])] {
            let mut target = Script::new([]);
            assert!(
                !ready(target.set_custom_eui64(address))
                    .unwrap_err()
                    .storage_may_have_changed()
            );
            target.assert_finished();
        }
    }

    #[test]
    fn malformed_or_non_erased_tokens_are_rejected_before_writes() {
        for token in [&[1, 0xFF][..], &[8, 1, 1, 1, 1, 1, 1, 1, 1][..]] {
            let mut target = Script::new([
                Transaction::reply(NETWORK_STATE, &[], &[0]),
                Transaction::reply(GET_EUI64, &[], CURRENT),
                Transaction::reply(GET_MFG_TOKEN, &[0x0C], token),
            ]);
            assert!(
                !ready(target.set_custom_eui64(ADDRESS))
                    .unwrap_err()
                    .storage_may_have_changed()
            );
            target.assert_finished();
        }
    }

    #[test]
    fn write_transport_or_status_failures_mark_possible_mutation() {
        for write in [
            Transaction::fail(SET_MFG_TOKEN, WRITE),
            Transaction::reply(SET_MFG_TOKEN, WRITE, &[0x02]),
        ] {
            let mut steps = prefix();
            steps.push(write);
            let mut target = Script::new(steps);
            let error = ready(target.set_custom_eui64(ADDRESS)).unwrap_err();
            assert_eq!(error.stage(), IdentityStage::Write);
            assert!(error.storage_may_have_changed());
            target.assert_finished();
        }
    }

    #[test]
    fn readback_mismatch_or_failure_preserves_mutation_boundary() {
        for readback in [
            Transaction::fail(GET_MFG_TOKEN, &[0x0C]),
            Transaction::reply(GET_MFG_TOKEN, &[0x0C], ERASED),
        ] {
            let mut steps = prefix();
            steps.extend([Transaction::reply(SET_MFG_TOKEN, WRITE, &[0]), readback]);
            let mut target = Script::new(steps);
            let error = ready(target.set_custom_eui64(ADDRESS)).unwrap_err();
            assert_eq!(error.stage(), IdentityStage::Readback);
            assert!(error.storage_may_have_changed());
            target.assert_finished();
        }
    }
}
