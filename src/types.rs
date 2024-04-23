pub type ByteSizedVec<T> = heapless::Vec<T, { u8::MAX as usize }>;
pub type UintT = u16; // TODO: is this the correct type?
