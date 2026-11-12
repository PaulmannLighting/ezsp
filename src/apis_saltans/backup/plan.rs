//! Pure restore preparation shared by direct callers and actor-dispatched traits.

use apis_saltans_hw::{BackupError, NetworkBackup, RestoreOptions};

use super::{EMBER_BACKUP_STACK, EmberSecurityState};

/// Structurally checked snapshot with normalized and Ember-extension counters advanced once.
///
/// This is a software plan, not a target-compatibility check or permission to
/// write hardware. No currently qualified firmware can execute this plan through
/// the complete-backup traits. Incoming replay counters and secret keys are retained.
#[derive(Debug)]
pub struct RestorePlan {
    backup: NetworkBackup,
    security: EmberSecurityState,
}

impl RestorePlan {
    /// Prepare an original snapshot without performing any I/O.
    ///
    /// Do not pass a snapshot already prepared by the hardware actor: that actor
    /// advances normalized counters before invoking `NetworkRestore` itself.
    ///
    /// # Errors
    /// Rejects malformed snapshots, foreign stacks, missing/unknown extensions,
    /// unsupported alternate-key transitions, and counter exhaustion.
    pub fn prepare(backup: &NetworkBackup, options: RestoreOptions) -> Result<Self, BackupError> {
        backup.validate()?;
        Self::from_prepared(&backup.prepare_restore(options)?, options)
    }

    /// Snapshot whose normalized counters and extension APS counter have been advanced.
    #[must_use]
    pub const fn backup(&self) -> &NetworkBackup {
        &self.backup
    }

    /// Prepared Ember security state, including the advanced global APS counter.
    #[must_use]
    pub const fn security(&self) -> &EmberSecurityState {
        &self.security
    }

    pub(super) fn from_prepared(
        backup: &NetworkBackup,
        options: RestoreOptions,
    ) -> Result<Self, BackupError> {
        backup.validate()?;
        if backup.source().stack() != EMBER_BACKUP_STACK {
            return Err(BackupError::Incompatible("backup is not from EmberZNet"));
        }
        if !backup.additional_network_keys().is_empty() {
            return Err(BackupError::Incompatible(
                "alternate network-key transitions are not implemented",
            ));
        }
        let [extension] = backup.extensions() else {
            return Err(BackupError::Incompatible(
                "exactly one known Ember security extension is required",
            ));
        };
        let mut security = EmberSecurityState::from_extension(extension)?;
        security.advance(options.link_counter_advance().get())?;
        let prepared = NetworkBackup::new(
            backup.source().clone(),
            backup.network().clone(),
            backup.network_key().clone(),
            backup.additional_network_keys().to_vec(),
            backup.devices().to_vec(),
            vec![security.to_extension()?],
        )?;
        Ok(Self {
            backup: prepared,
            security,
        })
    }
}
