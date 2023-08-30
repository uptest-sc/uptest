/*
Copyright © 2023 Rust Syndicate LLC <flipchan@rustsyndi.cat>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

*/

extern crate alloc;

use alloc::{boxed::Box, string::String};
use serde::{Deserialize, Serialize}; //boxed::Box,

//pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// Serde Json error
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
    /// Could not find the storage item
    StorageItemNotFound,
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
    /// Could not get next item in async loop
    AsyncNextError,
    /// Could not establish a subscription connection
    ConnectionSubscriptionProblem,
    Client(Box<dyn core::error::Error + Send + Sync + 'static>),
    Anyhow(anyhow::Error),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorEvent {
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

impl From<anyhow::Error> for Error {
    fn from(src: anyhow::Error) -> Error {
        Error::Anyhow(src)
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

impl From<&dyn std::error::Error> for Error {
    fn from(error: &dyn std::error::Error) -> Self {
        Self::Stderror(format!("{error:?}"))
    }
}
