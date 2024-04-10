use dotenvy::var;

pub fn weather_api_token() -> String {
	var("WEATHER_API_TOKEN").expect("WEATHER_API_TOKEN must be set")
}
