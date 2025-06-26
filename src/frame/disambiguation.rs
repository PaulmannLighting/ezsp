/// Disambiguation is used to differentiate between different types of frames that have the same frame ID assigned.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Disambiguation {
    /// No disambiguation is needed.
    #[default]
    None,
    /// Disambiguation for the `OverrideCurrentChannel` frame.
    OverrideCurrentChannel,
    /// Disambiguation for the `SetRadioIeee802154CcaMode` frame.
    SetRadioIeee802154CcaMode,
}
