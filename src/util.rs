//! Utility functions and types.

use std::sync::{PoisonError, RwLock};

/// A non-poisoning `RwLock`.
#[derive(Debug)]
pub struct NpNwLock<T>(RwLock<T>);

impl<T> NpNwLock<T> {
    /// Crate a new `NpRwLock` with the given value.
    pub const fn new(value: T) -> Self {
        Self(RwLock::new(value))
    }

    /// Read the value.
    pub fn read(&self) -> std::sync::RwLockReadGuard<'_, T> {
        self.0.read().unwrap_or_else(PoisonError::into_inner)
    }

    /// Write the value.
    pub fn write(&self) -> std::sync::RwLockWriteGuard<'_, T> {
        self.0.write().unwrap_or_else(PoisonError::into_inner)
    }
}
