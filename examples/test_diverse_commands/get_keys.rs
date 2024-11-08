//! Get diverse keys from the device.

use log::{error, info};

use ezsp::ember::key::Type;
use ezsp::uart::Uart;
use ezsp::Security;

/// Get diverse keys from the device.
#[allow(deprecated)]
pub async fn get_keys(ezsp: &mut Uart) {
    match ezsp.get_key(Type::TrustCenterLinkKey).await {
        Ok(key) => {
            info!("Current trust center link key: {key:?}");
        }
        Err(error) => {
            error!("Error getting trust center link key: {error}");
        }
    }

    match ezsp.get_key(Type::TrustCenterMasterKey).await {
        Ok(key) => {
            info!("Current trust center master key: {key:?}");
        }
        Err(error) => {
            error!("Error getting trust center master key: {error}");
        }
    }

    match ezsp.get_key(Type::CurrentNetworkKey).await {
        Ok(key) => {
            info!("Current network key: {key:?}");
        }
        Err(error) => {
            error!("Error getting current network key: {error}");
        }
    }

    match ezsp.get_key(Type::NextNetworkKey).await {
        Ok(key) => {
            info!("Next network key: {key:?}");
        }
        Err(error) => {
            error!("Error getting next network key: {error}");
        }
    }

    match ezsp.get_key(Type::ApplicationLinkKey).await {
        Ok(key) => {
            info!("Application link key: {key:?}");
        }
        Err(error) => {
            error!("Error getting application link key: {error}");
        }
    }

    match ezsp.get_key(Type::ApplicationMasterKey).await {
        Ok(key) => {
            info!("Application master key: {key:?}");
        }
        Err(error) => {
            error!("Error getting application master key: {error}");
        }
    }
}
