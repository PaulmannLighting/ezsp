use std::fmt::{Debug, Display};

use crate::{Callback, DefragmentedMessage};

/// Event type accepted by the high-level NCP callback handler.
///
/// Implementors must translate both ordinary EZSP callbacks and complete
/// incoming APS messages produced by defragmentation. A blanket implementation
/// is provided when both conversions exist and their errors are displayable and
/// sendable.
pub trait TranslatableEvent:
    Debug
    + TryFrom<Callback, Error: Display + Send>
    + TryFrom<DefragmentedMessage, Error: Display + Send>
    + Send
{
}

impl<T> TranslatableEvent for T
where
    T: Debug + TryFrom<Callback> + TryFrom<DefragmentedMessage> + Send,
    <T as TryFrom<Callback>>::Error: Display + Send,
    <T as TryFrom<DefragmentedMessage>>::Error: Display + Send,
{
}
