use serde::Deserialize;
use std::fmt::Display;

use super::{Current, ForecastDay, Location};

#[derive(Debug, Deserialize)]

pub struct ForecastResponse {
	pub location: Location,
	pub current: Current,
	pub forecast: Forecast,
}

#[derive(Debug, Deserialize)]

pub struct Forecast {
	pub forecastday: Vec<ForecastDay>,
}
impl Display for Forecast {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Forecast days:\n")?;
		for v in &self.forecastday {
			write!(f, "\t{}", v)?;
		}
		Ok(())
	}
}
