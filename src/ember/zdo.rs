//! Zigbee Device Object (ZDO) API.

pub mod configuration {
    //! ZDO configuration.

    use num_derive::{FromPrimitive, ToPrimitive};

    /// Flags for controlling which incoming ZDO requests are passed to the application.
    ///
    /// To see if the application is required to send a ZDO response to an incoming message,
    /// the application must check the APS options bitfield within the
    /// [`IncomingMessage`](crate::parameters::messaging::handler::IncomingMessage)
    /// callback to see if the
    /// [`ZdoResponseRequired`](crate::ember::aps::Option::ZdoResponseRequired)
    /// flag is set.
    #[allow(clippy::enum_variant_names)]
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromPrimitive, ToPrimitive)]
    pub enum Flags {
        /// The application receives supported ZDO requests.
        ReceivesSupportedZdoRequests = 0x01,
        /// The application handles unsupported ZDO requests.
        HandlesUnsupportedZdoRequests = 0x02,
        /// The application handles ZDO endpoint requests.
        HandlesZdoEndpointRequests = 0x04,
        /// The application handles ZDO binding requests.
        HandlesZdoBindingRequests = 0x08,
    }
}
