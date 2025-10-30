use crate::unit::Unit;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CurrentWeather {
    pub temperature: f64,
    pub windspeed: f64,
    pub weathercode: i32,
    #[serde(skip)]
    pub icon: String,
}

pub async fn fetch_weather(
    client: &Client,
    lat: f64,
    lon: f64,
    unit: Unit,
) -> Result<CurrentWeather, reqwest::Error> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?current_weather=true&latitude={}&longitude={}&temperature_unit={}",
        lat,
        lon,
        unit.as_api_param()
    );

    // Define local type alias just for deserialization...
    // There has to be a more elegant way to do this, but just want it to run for now
    // anonymous struct is not allowed by compiler, I don't yet understand why
    #[derive(Deserialize)]
    struct ApiResponse {
        current_weather: CurrentWeather
    }

    let mut response = client
        .get(&url)
        .send()
        .await?
        .error_for_status()? // return Err if not 200 OK
        .json::<ApiResponse>()
        .await?;

    response.current_weather.icon = get_icon(response.current_weather.weathercode).to_string();

    Ok(response.current_weather)
}

pub fn get_icon(code: i32) -> &'static str {
    match code {
        0 => "☀️  Clear sky",
        1..=3 => "🌤  Partly cloudy",
        45 | 48 => "🌫  Fog",
        51..=67 => "🌦  Drizzle or rain",
        71..=77 => "❄️  Snow",
        80..=82 => "🌧  Rain showers",
        _ => "❔  Unknown weather code",
    }
}
