mod extended;
mod extended_frame_control;
mod low_byte;
mod valid_control;

pub use extended::Extended;
pub use extended_frame_control::{ExtendedFrameControl, FrameFormatVersion};
use le_stream::derive::{FromLeStream, ToLeStream};
pub use low_byte::{CallbackType, Command, Response, SleepMode};
pub use valid_control::ValidControl;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct Control<T>
where
    T: ValidControl,
{
    inner: T,
}

impl Control<Command> {
    #[must_use]
    pub const fn new(command: Command) -> Self {
        Self { inner: command }
    }

    /// Returns `true` if the command is a response else `false`.
    pub fn is_response(&self) -> bool {
        self.inner.is_response()
    }

    /// Returns the network index.
    pub fn network_index(&self) -> u8 {
        self.inner.network_index()
    }

    /// Returns the sleep mode.
    pub fn sleep_mode(&self) -> SleepMode {
        self.inner.sleep_mode()
    }
}

impl Control<Extended<Command>> {
    #[must_use]
    pub const fn new(command: Command, extended_frame_control: ExtendedFrameControl) -> Self {
        Self {
            inner: Extended {
                low_byte: command,
                high_byte: extended_frame_control,
            },
        }
    }

    /// Returns `true` if the command is a response else `false`.
    pub fn is_response(&self) -> bool {
        self.inner.low_byte.is_response()
    }

    /// Returns the network index.
    pub fn network_index(&self) -> u8 {
        self.inner.low_byte.network_index()
    }

    /// Returns the sleep mode.
    pub fn sleep_mode(&self) -> SleepMode {
        self.inner.low_byte.sleep_mode()
    }

    /// Returns `true` if security is enabled else `false`.
    pub fn is_security_enabled(&self) -> bool {
        self.inner.high_byte.is_security_enabled()
    }

    /// Returns `true` if padding is enabled else `false`.
    pub fn is_padding_enabled(&self) -> bool {
        self.inner.high_byte.is_padding_enabled()
    }

    /// Returns the frame format version.
    pub fn frame_format_version(&self) -> FrameFormatVersion {
        self.inner.high_byte.frame_format_version()
    }
}

impl Control<Response> {
    #[must_use]
    pub const fn new(response: Response) -> Self {
        Self { inner: response }
    }

    /// Returns `true` if the response is a response else `false`.
    pub fn is_response(&self) -> bool {
        self.inner.is_response()
    }

    /// Returns the network index.
    pub fn network_index(&self) -> u8 {
        self.inner.network_index()
    }

    /// Returns the callback type.
    ///
    /// Returns `None` if this is not a callback.
    pub fn callback_type(&self) -> Option<CallbackType> {
        self.inner.callback_type()
    }

    /// Returns `true` if a callback is pending on the NCP.
    ///
    /// If this response is a callback, at least one more callback is available.
    pub fn is_callback_pending(&self) -> bool {
        self.inner.is_callback_pending()
    }

    /// Returns `true` if the response is truncated else `false`.
    pub fn is_truncated(&self) -> bool {
        self.inner.is_truncated()
    }

    /// Returns `true` if the response has overflowed else `false`.
    pub fn has_overflowed(&self) -> bool {
        self.inner.contains(Response::OVERFLOW)
    }
}

impl Control<Extended<Response>> {
    #[must_use]
    pub const fn new(response: Response, extended_frame_control: ExtendedFrameControl) -> Self {
        Self {
            inner: Extended {
                low_byte: response,
                high_byte: extended_frame_control,
            },
        }
    }

    /// Returns `true` if the response is a response else `false`.
    pub fn is_response(&self) -> bool {
        self.inner.low_byte.is_response()
    }

    /// Returns the network index.
    pub fn network_index(&self) -> u8 {
        self.inner.low_byte.network_index()
    }

    /// Returns the callback type.
    ///
    /// Returns `None` if this is not a callback.
    pub fn callback_type(&self) -> Option<CallbackType> {
        self.inner.low_byte.callback_type()
    }

    /// Returns `true` if a callback is pending on the NCP.
    ///
    /// If this response is a callback, at least one more callback is available.
    pub fn is_callback_pending(&self) -> bool {
        self.inner.low_byte.is_callback_pending()
    }

    /// Returns `true` if the response is truncated else `false`.
    pub fn is_truncated(&self) -> bool {
        self.inner.low_byte.is_truncated()
    }

    /// Returns `true` if the response has overflowed else `false`.
    pub fn has_overflowed(&self) -> bool {
        self.inner.low_byte.has_overflowed()
    }

    /// Returns `true` if security is enabled else `false`.
    pub fn is_security_enabled(&self) -> bool {
        self.inner.high_byte.is_security_enabled()
    }

    /// Returns `true` if padding is enabled else `false`.
    pub fn is_padding_enabled(&self) -> bool {
        self.inner.high_byte.is_padding_enabled()
    }

    /// Returns the frame format version.
    pub fn frame_format_version(&self) -> FrameFormatVersion {
        self.inner.high_byte.frame_format_version()
    }
}
