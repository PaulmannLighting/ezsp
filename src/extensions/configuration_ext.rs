use std::collections::BTreeMap;
use std::fmt::Display;

use enum_iterator::all;

use crate::extensions::Displayable;
use crate::ezsp::config;
use crate::{Configuration, Error};

/// Extension trait for retrieving all configuration values.
pub trait ConfigurationExt {
    /// Retrieves all configuration values in a `BTreeMap`.
    fn get_configuration(
        &mut self,
    ) -> impl Future<Output = Result<BTreeMap<config::Id, u16>, Error>>;
}

impl<T> ConfigurationExt for T
where
    T: Configuration + Send,
{
    async fn get_configuration(&mut self) -> Result<BTreeMap<config::Id, u16>, Error> {
        let mut configuration = BTreeMap::new();

        for id in all::<config::Id>() {
            configuration.insert(id, self.get_configuration_value(id).await?);
        }

        Ok(configuration)
    }
}

impl Displayable for BTreeMap<config::Id, u16> {
    type Displayable = DisplayableConfiguration;

    fn displayable(self) -> Self::Displayable {
        DisplayableConfiguration { inner: self }
    }
}

/// Represents the EZSP configuration as a mapping from configuration IDs to their values.
#[derive(Debug)]
pub struct DisplayableConfiguration {
    inner: BTreeMap<config::Id, u16>,
}

impl Display for DisplayableConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (id, value) in &self.inner {
            writeln!(f, "{id:?}: {value:#06X}")?;
        }

        Ok(())
    }
}
