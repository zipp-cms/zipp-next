use std::borrow::Cow;

// this error
#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("unknown error: {0}")]
	Unknown(Cow<'static, str>),
}
