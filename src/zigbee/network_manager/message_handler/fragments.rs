use std::collections::BTreeMap;

/// Structure for managing message fragments.
///
/// TODO: Implement fragment buffering.
#[derive(Debug)]
pub struct Fragments {
    buffer: BTreeMap<u8, Box<[u8]>>,
}
