extern crate alloc;
use alloc::{boxed::Box, string::String};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	SerdeJson(serde_json::error::Error),
	MpscSend(String),
	InvalidUrl(String),
	RecvError(String),
	Io(String),
	MaxConnectionAttemptsExceeded,
	/// connection to endpoint closed
	ConnectionClosed,
	/// connection Handshake failure
	ConnectionHandshakefailed,
	/// could not remove connection subscription
	Unsubscribefail,
	AsyncNextError,
	ConnectionSubscriptionProblem,
//	Client(Box<dyn core::error::Error + Send + Sync + 'static>),
}

impl From<serde_json::error::Error> for Error {
	fn from(error: serde_json::error::Error) -> Self {
		Self::SerdeJson(error)
	}
}