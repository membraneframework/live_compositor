pub(crate) mod api;
pub(crate) mod enums;
pub(crate) mod input_callback;

mod info;

use api::into_video_io_support;

pub use enums::ffi::FlagAttributeId;
pub use enums::ffi::FloatAttributeId;
pub use enums::ffi::IntegerAttributeId;
pub use enums::ffi::StringAttributeId;

pub use enums::ffi::AudioSampleType;
pub use enums::ffi::DetectedVideoInputFormatFlags;
pub use enums::ffi::DisplayModeType;
pub use enums::ffi::PixelFormat;
pub use enums::ffi::SupportedVideoModeFlags;
pub use enums::ffi::VideoConnection;
pub use enums::ffi::VideoInputConversionMode;
pub use enums::ffi::VideoInputFlags;
pub use enums::ffi::VideoInputFormatChangedEvents;

pub use api::input::AudioInputPacket;
pub use api::input::Input;
pub use api::input::VideoInputFrame;
pub use api::DeckLink;
pub use api::DisplayMode;
pub use api::VideoIOSupport;
pub use input_callback::InputCallback;
pub use input_callback::InputCallbackResult;

pub use api::get_decklinks;

#[derive(Debug, thiserror::Error)]
pub enum DeckLinkError {
    #[error("Unknown error: {0:#}")]
    UnknownError(#[from] cxx::Exception),

    #[error("ProfileAttribute error: {0}")]
    ProfileAttributeError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Decklink error code: {0:#x}")]
    HResultError(u32),
}

impl From<i64> for VideoIOSupport {
    fn from(value: i64) -> Self {
        into_video_io_support(value)
    }
}
