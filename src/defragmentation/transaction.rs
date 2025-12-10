use std::collections::BTreeMap;

use super::defragmented_message::DefragmentedMessage;
use crate::parameters::messaging::handler::IncomingMessage;

/// Represents a defragmentation consisting of multiple incoming message parts.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Transaction {
    size: u8,
    parts: BTreeMap<u8, IncomingMessage>,
}

impl Transaction {
    /// Creates a new defragmentation with the specified size.
    #[must_use]
    pub fn new(size: u8, index: u8, incoming_message: IncomingMessage) -> Self {
        let mut parts = BTreeMap::new();
        parts.insert(index, incoming_message);
        Self { size, parts }
    }

    /// Return the amount of expected fragments.
    #[must_use]
    pub const fn size(&self) -> u8 {
        self.size
    }

    /// Update the transaction with the given fragment.
    #[must_use]
    pub fn update(
        &mut self,
        index: u8,
        incoming_message: IncomingMessage,
    ) -> Option<IncomingMessage> {
        self.parts.insert(index, incoming_message)
    }

    /// Attempt to finalize the transaction.
    ///
    /// # Returns
    ///
    /// - `Some(DefragmentedMessage)` if the transaction is complete.
    /// - `None` if there are still fragments missing.
    #[must_use]
    pub fn finalize(&self) -> Option<DefragmentedMessage> {
        if self.parts.len() != usize::from(self.size) {
            return None;
        }

        let first_part = self.parts.get(&0)?;

        let mut message = Vec::with_capacity(
            self.parts
                .values()
                .map(|message| message.message().len())
                .sum(),
        );

        for index in 0..self.size {
            message.extend_from_slice(self.parts.get(&index)?.message());
        }

        Some(DefragmentedMessage::new_with_message(
            first_part.clone(),
            message,
        ))
    }
}
