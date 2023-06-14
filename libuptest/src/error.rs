extern crate alloc;

use alloc::{boxed::Box, string::String};
use serde::{Deserialize, Serialize}; //boxed::Box,

//pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    SerdeJson(serde_json::error::Error),
    MpscSend(String),
    InvalidUrl(String),
    RecvError(String),
    Io(String),
    FileIOerror(std::io::Error),
    Stderror(String),
    /// error getting block events
    ErrorEvent,
    /// Could not find event in the latest blocks
    EventNotFound,
    /// hex crate error
    HexError(hex::FromHexError),
    /// H256 fast hash hex decoding error
    FixedHashHexError(fixed_hash::rustc_hex::FromHexError),
    MaxConnectionAttemptsExceeded,
    /// connection to endpoint closed
    ConnectionClosed,
    /// connection Handshake failure
    ConnectionHandshakefailed,
    /// could not remove connection subscription
    Unsubscribefail,
    AsyncNextError,
    ConnectionSubscriptionProblem,
    Client(Box<dyn core::error::Error + Send + Sync + 'static>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorEvent {
    /// Reason of the error.
    pub error: String,
}

// add serde
impl From<serde_json::error::Error> for Error {
    fn from(error: serde_json::error::Error) -> Self {
        Self::SerdeJson(error)
    }
}
// add hex | solve the trait `From<hex::FromHexError>` is not implemented for `error::Error`
impl From<hex::FromHexError> for Error {
    fn from(value: hex::FromHexError) -> Self {
        Self::HexError(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::FileIOerror(value)
    }
}

// fixed-hash hex error
impl From<fixed_hash::rustc_hex::FromHexError> for Error {
    fn from(value: fixed_hash::rustc_hex::FromHexError) -> Self {
        Self::FixedHashHexError(value)
    }
}

//
impl From<&dyn std::error::Error> for Error {
    fn from(error: &dyn std::error::Error) -> Self {
        Self::Stderror(format!("{error:?}"))
    }
}
