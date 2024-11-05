/// Disambiguation is used to differentiate between different types of frames that have the same frame ID assigned.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Disambiguation {
    OverrideCurrentChannel,
    SetRadioIeee802154CcaMode,
}
