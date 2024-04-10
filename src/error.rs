use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Date provided is out of range. Valid range: {0} - {1}")]
	DateOutOfRange(String, String),
	#[error("Hour is out of range. Valid range: 0 - 24")]
	HourOutOfRange,
	#[error("Reqwest error: {0:#?}")]
	Reqwest(#[from] reqwest::Error),
	#[error("Invalid date format, must be in YYYY-MM-DD")]
	InvalidDate(#[from] chrono::ParseError),
}
