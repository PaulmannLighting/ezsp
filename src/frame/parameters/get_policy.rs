use crate::types::{EzspDecisionId, EzspPolicyId, EzspStatus};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x0056;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    policy_id: EzspPolicyId,
}

impl Command {
    #[must_use]
    pub const fn new(policy_id: EzspPolicyId) -> Self {
        Self { policy_id }
    }

    #[must_use]
    pub const fn policy_id(&self) -> EzspPolicyId {
        self.policy_id
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    status: EzspStatus,
    decision_id: EzspDecisionId,
}

impl Response {
    #[must_use]
    pub const fn new(status: EzspStatus, decision_id: EzspDecisionId) -> Self {
        Self {
            status,
            decision_id,
        }
    }

    #[must_use]
    pub const fn status(&self) -> EzspStatus {
        self.status
    }

    #[must_use]
    pub const fn decision_id(&self) -> EzspDecisionId {
        self.decision_id
    }
}
