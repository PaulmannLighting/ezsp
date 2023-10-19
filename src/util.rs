use std::io::{Read, Result};

pub trait ReadExt: Read {
    /// Reads an array of a constant size
    ///
    /// Refer to [`Read::read_exact`] for further semantics.
    ///
    /// # Errors
    /// Returns an [`std::io::Error`] on read errors.
    fn read_array_exact<const SIZE: usize>(&mut self) -> Result<[u8; SIZE]> {
        let mut buffer = [0; SIZE];
        self.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    /// Reads a boolean value of one byte
    ///
    /// # Errors
    /// Returns an [`std::io::Error`] on read errors.
    fn read_bool(&mut self) -> Result<bool> {
        Ok(self.read_u8()? != 0)
    }

    /// Reads an `i8`
    ///
    /// # Errors
    /// Returns an [`std::io::Error`] on read errors.
    fn read_i8(&mut self) -> Result<i8> {
        self.read_array_exact().map(i8::from_be_bytes)
    }

    /// Reads an `u8`
    ///
    /// # Errors
    /// Returns an [`std::io::Error`] on read errors.
    fn read_u8(&mut self) -> Result<u8> {
        self.read_array_exact().map(u8::from_be_bytes)
    }

    /// Reads an `u16`
    ///
    /// # Errors
    /// Returns an [`std::io::Error`] on read errors.
    fn read_u16_be(&mut self) -> Result<u16> {
        self.read_array_exact().map(u16::from_be_bytes)
    }

    /// Reads an `u32`
    ///
    /// # Errors
    /// Returns an [`std::io::Error`] on read errors.
    fn read_u32_be(&mut self) -> Result<u32> {
        self.read_array_exact().map(u32::from_be_bytes)
    }

    /// Reads an `u64`
    ///
    /// # Errors
    /// Returns an [`std::io::Error`] on read errors.
    fn read_u64_be(&mut self) -> Result<u64> {
        self.read_array_exact().map(u64::from_be_bytes)
    }

    /// Reads a `Vec<u8>` of a given size
    ///
    /// Refer to [`Read::read_exact`] for further semantics.
    ///
    /// # Errors
    /// Returns an [`std::io::Error`] on read errors.
    fn read_vec_exact<S>(&mut self, size: S) -> Result<Vec<u8>>
    where
        S: Into<usize>,
    {
        let mut buffer = vec![0; size.into()];
        self.read_exact(&mut buffer)?;
        Ok(buffer)
    }
}

impl<T> ReadExt for T where T: Read {}
