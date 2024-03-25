#[cfg(feature = "ashv2")]
mod ashv2;
mod ezsp;

#[cfg(feature = "ashv2")]
pub use ashv2::Ashv2;
pub use ezsp::Ezsp;
use le_stream::ToLeBytes;

pub trait Transport {
    fn next_command<T>(&mut self, frame_id: u16, parameters: T) -> Vec<u8>
    where
        T: ToLeBytes;
}
