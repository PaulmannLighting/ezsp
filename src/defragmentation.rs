use std::collections::BTreeMap;

use log::{error, trace, warn};

pub use self::defragmentation_error::DefragmentationError;
pub use self::defragmented_message::DefragmentedMessage;
pub use self::transaction::Transaction;
use crate::parameters::messaging::handler::IncomingMessage;

mod defragmentation_error;
mod defragmented_message;
mod transaction;

const BACKPRESSURE_THRESHOLD: usize = 2;

/// Defragment potentially fragmented EZSP frames.
pub trait Defragment {
    /// Attempt to defragment the given frame.
    ///
    /// # Returns
    ///
    /// - `Ok(Some(DefragmentedMessage))` if the defragmentation was completed successfully.
    /// - `Ok(None)` if the defragmentation is incomplete.
    /// - `Err(DefragmentationError)` if an error occurred during defragmentation.
    ///
    /// # Errors
    ///
    /// - [`DefragmentationError::StrayFragment`] if a follow-up fragment is received without an initial fragment.
    /// - [`DefragmentationError::DuplicateFragment`] if a duplicate fragment is received.
    fn defragment(
        &mut self,
        incoming_message: IncomingMessage,
    ) -> Result<Option<DefragmentedMessage>, DefragmentationError>;
}

impl Defragment for BTreeMap<u8, Transaction> {
    fn defragment(
        &mut self,
        incoming_message: IncomingMessage,
    ) -> Result<Option<DefragmentedMessage>, DefragmentationError> {
        if self.len() > BACKPRESSURE_THRESHOLD {
            warn!(
                "Defragmenter backpressure threshold exceeded: {} transactions in progress.",
                self.len()
            );
        }

        let seq = incoming_message.aps_frame().sequence();

        match incoming_message.aps_frame().fragmentation() {
            Some((index, Some(size))) => {
                trace!("Received initial fragment.");

                if size > 1 {
                    self.insert(seq, Transaction::new(size, index, incoming_message));
                    return Ok(None);
                }

                trace!("Received fragmented message with size 1, treating as non-fragmented.");
                Ok(Some(DefragmentedMessage::new(incoming_message)))
            }
            Some((index, None)) => {
                trace!("Received follow-up fragment.");

                let Some(transaction) = self.get_mut(&seq) else {
                    return Err(DefragmentationError::StrayFragment { seq, index });
                };

                if let Some(message) = transaction.update(index, incoming_message) {
                    error!("Duplicate fragment received. Previous was: {message:?}");
                    self.remove(&seq);
                    return Err(DefragmentationError::DuplicateFragment { seq, index });
                }

                let Some(defragmented_message) = transaction.finalize() else {
                    trace!(
                        "Defragmentation incomplete. seq: {seq}, fragment: {index}/{}",
                        transaction.size()
                    );
                    return Ok(None);
                };

                self.remove(&seq);
                Ok(Some(defragmented_message))
            }
            None => {
                trace!("Received non-fragmented frame.");
                Ok(Some(DefragmentedMessage::new(incoming_message)))
            }
        }
    }
}
