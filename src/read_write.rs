use std::io::{BufRead, ErrorKind, Read, Write};

pub trait Readable: Sized {
    /// Read data from a reader.
    ///
    /// # Errors
    /// Returns an [`anyhow::Error`] on I/O or parsing errors.
    fn try_read<R>(src: &mut R) -> Result<Self, crate::Error>
    where
        R: Read;

    /// Read data from a reader.
    ///
    /// # Errors
    /// Returns an [`anyhow::Error`] on I/O or parsing errors.  
    /// Additionally error out, if there is data left in the reader after reading.
    fn try_read_exact<R>(src: &mut R) -> Result<Self, crate::Error>
    where
        R: BufRead,
    {
        let result = Self::try_read(src)?;

        if src.has_data_left()? {
            Err(std::io::Error::new(ErrorKind::InvalidData, "data left over").into())
        } else {
            Ok(result)
        }
    }
}

pub trait Writable {
    /// Write parameters to a writer
    ///
    /// # Errors
    /// Returns an [`std::io::Error`] on errors.
    fn write_to<W>(self, dst: &mut W) -> std::io::Result<()>
    where
        W: Write;
}

impl<T> Writable for T
where
    T: IntoIterator<Item = u8>,
{
    fn write_to<W>(self, dst: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        for byte in self {
            dst.write_all(&[byte])?;
        }

        Ok(())
    }
}
