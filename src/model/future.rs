use serde::Deserialize;

use super::{forecast::Forecast, Location};

#[derive(Debug, Deserialize)]

pub struct FutureResponse {
	pub location: Location,
	pub forecast: Forecast,
}
