use std::collections::BTreeMap;
use std::fmt::Display;

use enum_iterator::all;

use crate::extensions::Displayable;
use crate::ezsp::{decision, policy};
use crate::{Configuration, Error};

/// Extension trait for retrieving all policy values.
pub trait PolicyExt {
    /// Retrieves all policy values in a `BTreeMap`.
    fn get_policies(
        &mut self,
    ) -> impl Future<Output = Result<BTreeMap<policy::Id, decision::Id>, Error>>;
}

impl<T> PolicyExt for T
where
    T: Configuration + Send,
{
    async fn get_policies(&mut self) -> Result<BTreeMap<policy::Id, decision::Id>, Error> {
        let mut policy = BTreeMap::new();

        for id in all::<policy::Id>() {
            policy.insert(id, self.get_policy(id).await?);
        }

        Ok(policy)
    }
}

impl Displayable for BTreeMap<policy::Id, decision::Id> {
    type Displayable = DisplayablePolicy;

    fn displayable(self) -> Self::Displayable {
        DisplayablePolicy { inner: self }
    }
}

/// Represents the EZSP policy as a mapping from policy IDs to their decisions.
#[derive(Debug)]
pub struct DisplayablePolicy {
    inner: BTreeMap<policy::Id, decision::Id>,
}

impl Display for DisplayablePolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (id, decision) in &self.inner {
            writeln!(f, "{id:?}: {decision:?}")?;
        }

        Ok(())
    }
}
