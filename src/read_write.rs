use std::io::{Read, Write};

pub trait Readable: Sized {
    /// Read parameters from a reader
    ///
    /// # Errors
    /// Returns an [`anyhow::Error`] on errors.
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read;
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
