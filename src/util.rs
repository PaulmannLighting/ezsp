use num_traits::FromBytes;
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

    /// Reads an boolean of one byte
    ///
    /// Refer to [`Read::read_exact`] for further semantics.
    ///
    /// # Errors
    /// Returns an [`std::io::Error`] on read errors.
    fn read_bool(&mut self) -> Result<bool> {
        self.read_array_exact::<1>().map(|[byte]| byte != 0)
    }

    /// Reads an arbitrary number from big-endian bytes
    ///
    /// Refer to [`Read::read_exact`] for further semantics.
    ///
    /// # Errors
    /// Returns an [`std::io::Error`] on read errors.
    fn read_num_be<N, const SIZE: usize>(&mut self) -> Result<N>
    where
        N: FromBytes<Bytes = [u8; SIZE]>,
    {
        self.read_array_exact::<SIZE>()
            .map(|bytes| N::from_be_bytes(&bytes))
    }

    /// Reads an arbitrary number from little-endian bytes
    ///
    /// Refer to [`Read::read_exact`] for further semantics.
    ///
    /// # Errors
    /// Returns an [`std::io::Error`] on read errors.
    fn read_num_le<N, const SIZE: usize>(&mut self) -> Result<N>
    where
        N: FromBytes<Bytes = [u8; SIZE]>,
    {
        self.read_array_exact::<SIZE>()
            .map(|bytes| N::from_le_bytes(&bytes))
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
