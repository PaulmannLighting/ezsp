use le_stream::FromLeStream;

pub fn parse_response<T>(bytes: &[u8]) -> Result<T, crate::Error>
where
    T: FromLeStream,
{
    Ok(T::from_le_stream_exact(bytes.iter().copied())?)
}
