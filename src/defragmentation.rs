use std::collections::BTreeMap;

use log::{error, trace};

pub use self::defragmentation_error::DefragmentationError;
pub use self::defragmented_message::DefragmentedMessage;
use self::transaction::Transaction;
use crate::parameters::messaging::handler::IncomingMessage;

mod defragmentation_error;
mod defragmented_message;
mod transaction;

/// Defragments potentially fragmented EZSP frames.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Defragmenter {
    inner: BTreeMap<u8, Transaction>,
}

impl Defragmenter {
    /// Attempt to defragment the given frame.
    ///
    /// # Returns
    ///
    /// - `Some(DefragmentedMessage)` if the defragmentation was completed successfully.
    /// - `None` if the defragmentation is incomplete.
    pub fn defragment(
        &mut self,
        incoming_message: IncomingMessage,
    ) -> Result<Option<DefragmentedMessage>, DefragmentationError> {
        let seq = incoming_message.aps_frame().sequence();

        match incoming_message.aps_frame().fragmentation() {
            Some((index, Some(size))) => {
                trace!("Received initial fragment.");

                if size > 1 {
                    self.inner
                        .insert(seq, Transaction::new(size, index, incoming_message));
                    return Ok(None);
                }

                trace!("Received fragmented message with size 1, treating as non-fragmented.");
                Ok(Some(DefragmentedMessage::new(incoming_message)))
            }
            Some((index, None)) => {
                trace!("Received follow-up fragment.");

                let Some(transaction) = self.inner.get_mut(&seq) else {
                    return Err(DefragmentationError::StrayFragment { seq, index });
                };

                if let Some(message) = transaction.update(index, incoming_message) {
                    error!("Duplicate fragment received. Previous was: {message:?}");
                    self.inner.remove(&seq);
                    return Err(DefragmentationError::DuplicateFragment { seq, index });
                }

                let Some(defragmented_message) = transaction.finalize() else {
                    trace!(
                        "Defragmentation incomplete. seq: {seq}, fragment: {index}/{}",
                        transaction.size()
                    );
                    return Ok(None);
                };

                self.inner.remove(&seq);
                Ok(Some(defragmented_message))
            }
            None => {
                trace!("Received non-fragmented frame.");
                Ok(Some(DefragmentedMessage::new(incoming_message)))
            }
        }
    }
}
