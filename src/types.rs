pub type ByteSizedVec<T> = heapless::Vec<T, { u8::MAX as usize }>;
