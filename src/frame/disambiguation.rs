/// Disambiguation is used to differentiate between different types of frames that have the same frame ID assigned.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Disambiguation {
    #[default]
    None,
    OverrideCurrentChannel,
    SetRadioIeee802154CcaMode,
}
