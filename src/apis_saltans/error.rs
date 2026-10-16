//! Portable networking classifications with the original EZSP error retained as a source.

use apis_saltans_hw::{AcknowledgementLayer, NetworkError};

use crate::ember::{Mac, Phy, Status};
use crate::error::Status as ErrorStatus;

impl TryFrom<Status> for NetworkError {
    type Error = ();

    /// Classifies a known Ember networking failure.
    ///
    /// # Errors
    ///
    /// Returns `()` for success and statuses without a portable classification.
    fn try_from(status: Status) -> Result<Self, Self::Error> {
        Ok(match status {
            Status::DeliveryFailed => Self::DeliveryFailed,
            Status::Mac(Mac::NoAckReceived) => Self::NoAcknowledgement {
                layer: AcknowledgementLayer::Mac,
            },
            Status::Mac(Mac::IndirectTimeout) => Self::IndirectTimeout,
            Status::Phy(Phy::TxCcaFail) => Self::ChannelAccessFailure,
            Status::NetworkBusy => Self::NetworkBusy,
            Status::NoBuffers
            | Status::MaxMessageLimitReached
            | Status::Mac(Mac::TransmitQueueFull) => Self::ResourcesExhausted,
            Status::NetworkDown => Self::NetworkDown,
            Status::NotJoined => Self::NotJoined,
            Status::SourceRouteFailure => Self::SourceRouteFailure,
            Status::ManyToOneRouteFailure => Self::ManyToOneRouteFailure,
            Status::MessageTooLong => Self::MessageTooLong,
            Status::ApsEncryptionError
            | Status::KeyNotAuthorized
            | Status::SecurityStateNotSet
            | Status::SecurityConfigurationInvalid
            | Status::SecurityDataInvalid => Self::SecurityFailure,
            Status::TableFull => Self::TableFull,
            _ => return Err(()),
        })
    }
}

impl From<crate::Error> for apis_saltans_hw::Error {
    fn from(error: crate::Error) -> Self {
        if let crate::Error::Status(ErrorStatus::Ember(Ok(status))) = &error
            && let Ok(kind) = NetworkError::try_from(*status)
        {
            return Self::network(kind, error);
        }
        Self::backend(error)
    }
}

#[cfg(test)]
mod tests {
    use apis_saltans_hw::{AcknowledgementLayer, Error, NetworkError};

    use crate::ember::{Mac, Phy, Status};
    use crate::error::Status as ErrorStatus;

    #[test]
    fn delivery_failure_retains_its_backend_source() {
        let error = Error::from(crate::Error::from(ErrorStatus::from(
            Status::DeliveryFailed,
        )));
        assert_eq!(error.network_error(), Some(NetworkError::DeliveryFailed));
        let Error::Network {
            source: Some(source),
            ..
        } = error
        else {
            panic!("missing source")
        };
        assert!(matches!(
            source.downcast_ref::<crate::Error>(),
            Some(crate::Error::Status(ErrorStatus::Ember(Ok(
                Status::DeliveryFailed
            ))))
        ));
    }

    #[test]
    fn common_failures_have_distinct_classifications() {
        for (status, expected) in [
            (Status::NetworkDown, NetworkError::NetworkDown),
            (Status::NotJoined, NetworkError::NotJoined),
            (Status::NetworkBusy, NetworkError::NetworkBusy),
            (Status::NoBuffers, NetworkError::ResourcesExhausted),
            (
                Status::MaxMessageLimitReached,
                NetworkError::ResourcesExhausted,
            ),
            (
                Status::Mac(Mac::TransmitQueueFull),
                NetworkError::ResourcesExhausted,
            ),
            (
                Status::Mac(Mac::NoAckReceived),
                NetworkError::NoAcknowledgement {
                    layer: AcknowledgementLayer::Mac,
                },
            ),
            (
                Status::Mac(Mac::IndirectTimeout),
                NetworkError::IndirectTimeout,
            ),
            (
                Status::Phy(Phy::TxCcaFail),
                NetworkError::ChannelAccessFailure,
            ),
            (Status::SourceRouteFailure, NetworkError::SourceRouteFailure),
            (
                Status::ManyToOneRouteFailure,
                NetworkError::ManyToOneRouteFailure,
            ),
            (Status::MessageTooLong, NetworkError::MessageTooLong),
            (Status::ApsEncryptionError, NetworkError::SecurityFailure),
            (Status::KeyNotAuthorized, NetworkError::SecurityFailure),
            (Status::SecurityStateNotSet, NetworkError::SecurityFailure),
            (
                Status::SecurityConfigurationInvalid,
                NetworkError::SecurityFailure,
            ),
            (Status::SecurityDataInvalid, NetworkError::SecurityFailure),
            (Status::TableFull, NetworkError::TableFull),
        ] {
            let error = Error::from(crate::Error::from(ErrorStatus::from(status)));
            assert_eq!(error.network_error(), Some(expected));
        }
    }

    #[test]
    fn other_status_namespaces_retain_their_source() {
        const DELIVERY_FAILED: u8 = 0x66;

        for status in [
            ErrorStatus::Ezsp(Err(DELIVERY_FAILED)),
            ErrorStatus::Sl(Err(u32::from(DELIVERY_FAILED))),
            ErrorStatus::from(crate::ezsp::Status::NoTxSpace),
            ErrorStatus::from(Status::ErrFatal),
        ] {
            let original = crate::Error::from(status);
            let original_message = original.to_string();
            let error = Error::from(original);
            assert_eq!(error.network_error(), None);
            let Error::Backend(source) = error else {
                panic!("expected backend error");
            };
            assert!(source.downcast_ref::<crate::Error>().is_some());
            assert_eq!(source.to_string(), original_message);
        }
    }

    #[test]
    fn unknown_status_is_not_classified() {
        let error = Error::from(crate::Error::from(ErrorStatus::Ember(Err(u8::MAX))));
        assert!(matches!(error, Error::Backend(_)));
        assert_eq!(NetworkError::try_from(Status::Success), Err(()));
        assert_eq!(NetworkError::try_from(Status::ErrFatal), Err(()));
    }
}
