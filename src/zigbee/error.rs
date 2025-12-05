use std::sync::Arc;

impl From<crate::Error> for zigbee_nwk::Error {
    fn from(error: crate::Error) -> Self {
        Self::Implementation(Arc::new(error))
    }
}
