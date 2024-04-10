use serde::Deserialize;

use super::{forecast::Forecast, Location};

#[derive(Debug, Deserialize)]

pub struct HistoryResponse {
	pub location: Location,
	pub forecast: Forecast,
}
