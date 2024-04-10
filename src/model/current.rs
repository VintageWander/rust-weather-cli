use serde::Deserialize;

use super::{Current, Location};

#[derive(Debug, Deserialize)]

pub struct CurrentResponse {
	pub location: Location,
	pub current: Current,
}
