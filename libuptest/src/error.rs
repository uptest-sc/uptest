extern crate alloc;
use alloc::{string::String};//boxed::Box, 

//pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	SerdeJson(serde_json::error::Error),
	MpscSend(String),
	InvalidUrl(String),
	RecvError(String),
	Io(String),
	HexError(hex::FromHexError),
	MaxConnectionAttemptsExceeded,
	/// connection to endpoint closed
	ConnectionClosed,
	/// connection Handshake failure
	ConnectionHandshakefailed,
	/// could not remove connection subscription
	Unsubscribefail,
	AsyncNextError,
	ConnectionSubscriptionProblem,
	//Other(Box<dyn core::error::Error + Send + Sync + 'static>),
//	Client(Box<dyn core::error::Error + Send + Sync + 'static>),
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