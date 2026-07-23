use apis_saltans_hw::TxOptions;

use crate::ember::aps::Options;

impl From<TxOptions> for Options {
    fn from(tx_options: TxOptions) -> Self {
        let mut options = Self::empty();

        options.set(
            Self::RETRY,
            tx_options.contains(TxOptions::ACKNOWLEDGED_TRANSMISSION),
        );

        options.set(
            Self::ENCRYPTION,
            tx_options.contains(TxOptions::SECURITY_ENABLED),
        );

        options
    }
}
