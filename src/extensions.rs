use std::collections::BTreeMap;
use std::fmt::{self, Formatter};

use enum_iterator::all;

use crate::ezsp::{config, decision, policy};
use crate::{Configuration, Error};

/// Extension trait for retrieving all configuration values.
pub trait ConfigurationExt {
    /// Retrieves all configuration values in a `BTreeMap`.
    fn get_config(&mut self) -> impl Future<Output = Result<BTreeMap<config::Id, u16>, Error>>;
}

impl<T> ConfigurationExt for T
where
    T: Configuration + Send,
{
    async fn get_config(&mut self) -> Result<BTreeMap<config::Id, u16>, Error> {
        let mut configuration = BTreeMap::new();

        for id in all::<config::Id>() {
            configuration.insert(id, self.get_configuration_value(id).await?);
        }

        Ok(configuration)
    }
}

/// Extension trait for retrieving all policy values.
pub trait PolicyExt {
    /// Retrieves all policy values in a `BTreeMap`.
    fn get_policy(
        &mut self,
    ) -> impl Future<Output = Result<BTreeMap<policy::Id, decision::Id>, Error>>;
}

impl<T> PolicyExt for T
where
    T: Configuration + Send,
{
    async fn get_policy(&mut self) -> Result<BTreeMap<policy::Id, decision::Id>, Error> {
        let mut policy = BTreeMap::new();

        for id in all::<policy::Id>() {
            policy.insert(id, self.get_policy(id).await?);
        }

        Ok(policy)
    }
}

/// Displays the configuration in a human-readable format.
///
/// # Errors
///
/// Returns an [`Error`](fmt::Error) if writing to the formatter fails.
pub fn display_configuration(
    configuration: BTreeMap<config::Id, u16>,
    f: &mut Formatter<'_>,
) -> fmt::Result {
    for (id, value) in configuration {
        writeln!(f, "{id:?} => {value:#06X}")?;
    }

    Ok(())
}

/// Displays the policy in a human-readable format.
///
/// # Errors
///
/// Returns an [`Error`](fmt::Error) if writing to the formatter fails.
pub fn display_policy(
    policy: BTreeMap<policy::Id, decision::Id>,
    f: &mut Formatter<'_>,
) -> fmt::Result {
    for (id, decision) in policy {
        writeln!(f, "{id:?} => {decision:?}")?;
    }

    Ok(())
}
