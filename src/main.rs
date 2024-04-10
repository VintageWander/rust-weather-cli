mod env;
mod error;
// Currently not needed
// mod model;

use crate::env::weather_api_token;
use chrono::{Days, NaiveDate, Utc};
use clap::Parser;
use error::Error;
use json_to_table::json_to_table;
use tabled::settings::Style;

#[derive(Parser)]
#[command(name = "Rust Weather CLI tool")]
#[command(version = "1.0")]
#[command(about = "Handy dandy tool to check the weather forecast", long_about = None)]
enum WeatherCli {
	/// Check the current weather
	Current {
		/// Pass US Zipcode, UK Postcode, Canada Postalcode, IP address, Latitude/Longitude (decimal degree) or city name.
		#[arg(short, long)]
		query: String,
	},
	/// Check the forecast weather. Going from 1 - 14 days in the future from now
	Forecast {
		/// Pass US Zipcode, UK Postcode, Canada Postalcode, IP address, Latitude/Longitude (decimal degree) or city name.
		#[arg(short, long)]
		query: String,
		/// Number of days of weather forecast. Value ranges from 1 to 14
		#[arg(long)]
		days: u8,
		/// Date should be between today and next 14 day in yyyy-MM-dd format. e.g. '2015-01-01'
		#[arg(long)]
		date: Option<String>,
		/// Must be in 24 hour. For example 5 pm should be hour=17, 6 am as hour=6
		#[arg(long)]
		hour: Option<u8>,
		/// Enable/Disable Air Quality data in forecast API output. Example, aqi=true or aqi=false
		#[arg(long)]
		aqi: Option<bool>,
	},
	/// Check the forecast weather. Going from 14 - 300 days in the future from now
	Future {
		/// Pass US Zipcode, UK Postcode, Canada Postalcode, IP address, Latitude/Longitude (decimal degree) or city name.
		#[arg(short, long)]
		query: String,
		/// Date should be between 14 days and 300 days from today in the future in yyyy-MM-dd format (i.e. dt=2023-01-01)
		#[arg(long)]
		date: Option<String>,
	},
	/// Check the forecast history
	History {
		/// Pass US Zipcode, UK Postcode, Canada Postalcode, IP address, Latitude/Longitude (decimal degree) or city name.
		#[arg(short, long)]
		query: String,
		/// Date on or after 1st Jan, 2015 in yyyy-MM-dd format
		#[arg(long)]
		date: String,
		/// Date on or after 1st Jan, 2015 in yyyy-MM-dd format
		/// 'end_date' should be greater than 'date' parameter and difference should not be more than 30 days between the two dates.
		#[arg(long)]
		end_date: Option<String>,
		/// Must be in 24 hour. For example 5 pm should be hour=17, 6 am as hour=6
		#[arg(long)]
		hour: Option<u8>,
	},
}

#[tokio::main]
async fn main() -> Result<(), Error> {
	// Retrieve WeatherAPI.com token to use
	let token = weather_api_token();

	// Setup base url
	let url = "https://api.weatherapi.com/v1";

	// Construct http client for requesting
	let http_client = reqwest::Client::new();

	// Parse CLI arguments
	let cli = WeatherCli::parse();

	match cli {
		WeatherCli::Current { query } => {
			let response = http_client
				.get(format!("{url}/current.json?q={query}&key={token}"))
				.send()
				.await?
				.json()
				.await?;
			let table = json_to_table(&response).with(Style::rounded()).to_string();
			println!("{table}");
		}
		WeatherCli::Forecast {
			query,
			days,
			date,
			hour,
			aqi,
		} => {
			// If 1 > days or days > 14 -> Out of range
			if 1 > days || days > 14 {
				return Err(Error::DateOutOfRange("1".into(), "14".into()));
			}
			let mut request_endpoint =
				format!("{url}/forecast.json?key={token}&q={query}&days={days}");
			if let Some(date) = date {
				// Converts string into date of format yyyy-mm-dd
				let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")?;
				// Get the current date
				let now = Utc::now().date_naive();
				// Add 14 days to the current date
				let now_add_14 = now.checked_add_days(Days::new(14)).unwrap();
				if now > date || date > now_add_14 {
					return Err(Error::DateOutOfRange(
						now.to_string(),
						now_add_14.to_string(),
					));
				}
				request_endpoint.push_str(&format!("&dt={date}"));
			}
			if let Some(hour) = hour {
				// If hour > 24 -> Invalid hour
				if hour > 24 {
					return Err(Error::HourOutOfRange);
				}
				request_endpoint.push_str(&format!("&hour={hour}"));
			}
			// If the arguments provided aqi
			if let Some(aqi) = aqi {
				request_endpoint.push_str(&format!("&aqi={}", if aqi { "yes" } else { "no" }));
			}
			let response = http_client
				.get(request_endpoint)
				.send()
				.await?
				.json()
				.await?;
			let table = json_to_table(&response).with(Style::rounded()).to_string();
			println!("{table}");
		}
		WeatherCli::Future { query, date } => {
			let mut request_endpoint = format!("{url}/future.json?key={token}&query={query}");
			if let Some(date) = date {
				// Converts string into date in format of yyyy-mm-dd
				let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")?;
				// Get current date
				let now = Utc::now().date_naive();
				// now + 14 days
				let min_date = now.checked_add_days(Days::new(14)).unwrap();
				// now + 300 days
				let max_date = now.checked_add_days(Days::new(300)).unwrap();
				// If (now + 14) > date or date > (now + 300) -> Out of range
				if min_date > date || date > max_date {
					return Err(Error::DateOutOfRange(
						"14 days after now".into(),
						"300 days after now".into(),
					));
				}
				request_endpoint.push_str(&format!("&dt={date}"))
			}
			let response = http_client
				.get(request_endpoint)
				.send()
				.await?
				.json()
				.await?;
			let table = json_to_table(&response).with(Style::rounded()).to_string();
			println!("{table}");
		}
		WeatherCli::History {
			query,
			date,
			end_date,
			hour,
		} => {
			// Converts string into date in format yyyy-mm-dd
			let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")?;
			// 2015-01-01 - Jan 1st 2015
			let min_allowed_date = NaiveDate::from_ymd_opt(2015, 01, 01).unwrap();
			// Get current date
			let now = Utc::now().date_naive();
			// if 2015-01-01 > date or date > now -> Out of range
			if min_allowed_date > date || date > now {
				return Err(Error::DateOutOfRange("2015-01-01".into(), now.to_string()));
			}
			let mut request_endpoint =
				format!("{url}/history.json?key={token}&query={query}&dt={date}");
			if let Some(end_date) = end_date {
				// Converts string into date in format yyyy-mm-dd
				let end_date = NaiveDate::parse_from_str(&end_date, "%Y-%m-%d")?;
				// date + 30
				let date_add_30_days = date.checked_add_days(Days::new(30)).unwrap();
				// If (date + 30) > end_date or end_date > now -> Out of range
				if date_add_30_days > end_date || end_date > now {
					return Err(Error::DateOutOfRange(
						date_add_30_days.to_string(),
						now.to_string(),
					));
				}
				request_endpoint.push_str(&format!("&end_dt={end_date}"));
			}
			if let Some(hour) = hour {
				// If hour > 24 -> Invalid hour
				if hour > 24 {
					return Err(Error::HourOutOfRange);
				}
				request_endpoint.push_str(&format!("&hour={hour}"));
			}
			let response = http_client
				.get(request_endpoint)
				.send()
				.await?
				.json()
				.await?;
			let table = json_to_table(&response).with(Style::rounded()).to_string();
			println!("{table}");
		}
	};
	Ok(())
}
