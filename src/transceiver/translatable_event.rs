use std::fmt::Display;

use crate::{Callback, DefragmentedMessage};

pub trait TranslatableEvent:
    TryFrom<Callback, Error: Display + Send>
    + TryFrom<DefragmentedMessage, Error: Display + Send>
    + Send
{
}

impl<T> TranslatableEvent for T
where
    T: TryFrom<Callback> + TryFrom<DefragmentedMessage> + Send,
    <T as TryFrom<Callback>>::Error: Display + Send,
    <T as TryFrom<DefragmentedMessage>>::Error: Display + Send,
{
}
