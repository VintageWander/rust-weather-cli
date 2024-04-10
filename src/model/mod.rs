pub mod current;
pub mod forecast;
pub mod future;
pub mod history;

use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Location {
	pub name: String,
	pub region: String,
	pub country: String,
	pub lat: f32,
	pub lon: f32,
	#[serde(rename = "tz_id")]
	pub timezone: String,
	pub localtime_epoch: u64,
	pub localtime: String,
}
impl Display for Location {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{self}"))
	}
}

#[derive(Debug, Deserialize)]

pub struct Current {
	pub last_updated_epoch: u64,
	pub last_updated: String,
	pub temp_c: f32,
	pub temp_f: f32,
	pub is_day: u8,
	pub condition: Condition,
	pub wind_mph: f32,
	pub wind_kph: f32,
	pub wind_degree: u8,
	pub wind_dir: String,
	pub pressure_mb: f32,
	pub pressure_in: f32,
	pub precip_mm: f32,
	pub precip_in: f32,
	pub humidity: u16,
	pub cloud: u8,
	pub feelslike_c: f32,
	pub feelslike_f: f32,
	pub vis_km: f32,
	pub vis_miles: f32,
	pub uv: f32,
	pub gust_mph: f32,
	pub gust_kph: f32,
}
impl Display for Current {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{self}"))
	}
}

#[derive(Debug, Deserialize)]

pub struct Condition {
	pub text: String,
	pub icon: String,
	pub code: u32,
}

impl Display for Condition {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{self}"))
	}
}

#[derive(Debug, Deserialize)]

pub struct ForecastDay {
	pub date: String,
	pub date_epoch: u64,
	pub day: DaySpec,
	pub astro: AstroSpec,
	pub hour: Vec<Hour>,
}
impl Display for ForecastDay {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{self}"))
	}
}

#[derive(Debug, Deserialize)]

pub struct DaySpec {
	pub maxtemp_c: f32,
	pub maxtemp_f: f32,
	pub mintemp_c: f32,
	pub mintemp_f: f32,
	pub avgtemp_c: f32,
	pub avgtemp_f: f32,
	pub maxwind_mph: f32,
	pub maxwind_kph: f32,
	pub totalprecip_mm: f32,
	pub totalprecip_in: f32,
	pub totalsnow_cm: f32,
	pub avgvis_km: f32,
	pub avgvis_miles: f32,
	pub avghumidity: f32,
	pub daily_will_it_rain: u32,
	pub daily_chance_of_rain: u32,
	pub daily_will_it_snow: u32,
	pub daily_chance_of_snow: u32,
	pub condition: Condition,
	pub uv: f32,
}

impl Display for DaySpec {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{self}"))
	}
}

#[derive(Debug, Deserialize)]

pub struct AstroSpec {
	pub sunrise: String,
	pub sunset: String,
	pub moonrise: String,
	pub moonset: String,
	pub moon_phase: String,
	pub moon_illumination: u32,
	pub is_moon_up: u8,
	pub is_sun_up: u8,
}
impl Display for AstroSpec {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{self}"))
	}
}

#[derive(Debug, Deserialize)]

pub struct Hour {
	pub time_epoch: u64,
	pub time: String,
	pub temp_c: f32,
	pub temp_f: f32,
	pub is_day: u8,
	pub condition: Condition,
	pub wind_mph: f32,
	pub wind_kph: f32,
	pub wind_degree: u8,
	pub wind_dir: String,
	pub pressure_mb: f32,
	pub pressure_in: f32,
	pub precip_mm: f32,
	pub precip_in: f32,
	pub snow_cm: f32,
	pub humidity: u16,
	pub cloud: u8,
	pub feelslike_c: f32,
	pub feelslike_f: f32,
	pub windchill_c: f32,
	pub windchill_f: f32,
	pub heatindex_c: f32,
	pub heatindex_f: f32,
	pub dewpoint_c: f32,
	pub dewpoint_f: f32,
	pub will_it_rain: u8,
	pub chance_of_rain: f32,
	pub will_it_snow: u8,
	pub chance_of_snow: f32,
	pub vis_km: f32,
	pub vis_miles: f32,
	pub gust_mph: f32,
	pub gust_kph: f32,
	pub uv: f32,
}
impl Display for Hour {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{self}"))
	}
}
