//! Pure restore preparation shared by direct callers and actor-dispatched traits.

use apis_saltans_hw::{BackupCoverage, BackupError, NetworkBackup, RestoreOptions};

use super::image::{TOKEN_EXTENSION, TokenImage};
use super::{
    EMBER_BACKUP_NAMESPACE, EMBER_BACKUP_STACK, EMBER_SECURITY_EXTENSION, EmberSecurityState,
};

/// Structurally checked snapshot with normalized and Ember-extension counters advanced once.
///
/// This is a software plan, not a target-compatibility check or permission to
/// write hardware. Profile-driven execution requires a verified firmware contract.
/// Incoming replay counters and secret keys are retained. Embedded token-image
/// counters remain original here; the configured backend patches their offsets.
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

    /// Snapshot whose normalized counters and security-extension APS counter are advanced.
    ///
    /// Opaque token-image counters remain original until the backend applies its layout.
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
        if backup.coverage() != BackupCoverage::Complete {
            return Err(BackupError::Incompatible(
                "offline token restore requires complete coverage",
            ));
        }
        if backup.source().stack() != EMBER_BACKUP_STACK {
            return Err(BackupError::Incompatible("backup is not from EmberZNet"));
        }
        if !backup.additional_network_keys().is_empty() {
            return Err(BackupError::Incompatible(
                "alternate network-key transitions are not implemented",
            ));
        }
        let extension = backup
            .extensions()
            .iter()
            .find(|extension| {
                extension.namespace() == EMBER_BACKUP_NAMESPACE
                    && extension.name() == EMBER_SECURITY_EXTENSION
            })
            .ok_or(BackupError::Incompatible(
                "Ember security extension is required",
            ))?;
        let mut tokens = None;
        for extra in backup.extensions() {
            if extra.namespace() == EMBER_BACKUP_NAMESPACE
                && extra.name() == EMBER_SECURITY_EXTENSION
            {
                continue;
            }
            if extra.namespace() != EMBER_BACKUP_NAMESPACE || extra.name() != TOKEN_EXTENSION {
                return Err(BackupError::Incompatible(
                    "unknown required Ember extension",
                ));
            }
            TokenImage::decode(extra)?;
            tokens = Some(extra.clone());
        }
        let mut security = EmberSecurityState::from_extension(extension)?;
        security.advance(options.link_counter_advance().get())?;
        let mut extensions = vec![security.to_extension()?];
        extensions.extend(tokens);
        let prepared = NetworkBackup::new(
            backup.source().clone(),
            backup.network().clone(),
            backup.network_key().clone(),
            backup.additional_network_keys().to_vec(),
            backup.devices().to_vec(),
            extensions,
        )?;
        Ok(Self {
            backup: prepared,
            security,
        })
    }
}
