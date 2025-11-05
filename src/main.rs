mod cli;
mod coords;
mod unit;
mod weather;

use clap::Parser;
use cli::CliArgs;
use coords::{get_coordinates, get_coordinates_from_ip};
use reqwest::Client;
use weather::fetch_weather;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();
    let client = Client::new();

    let (lat, lon) = if let Some(city) = &args.city {
        match get_coordinates(&client, city.as_str()).await {
            Ok(coords) => coords,
            Err(_) => {
                eprintln!("Error: Could not find city '{}'", city);
                return;
            }
        }
    } else {
        match get_coordinates_from_ip(&client).await {
            Ok(coords) => {
                println!(
                    "Using geolocation based on your IP address: {}°, {}°",
                    coords.0, coords.1
                );
                coords
            }
            Err(e) => {
                eprintln!("Error: Could not determine location from IP address: {}", e);
                return;
            }
        }
    };

    let unit = args.unit();
    let weather = match fetch_weather(&client, lat, lon, unit).await {
        Ok(weather) => weather,
        Err(e) => {
            eprintln!("Error fetching weather data: {}", e);
            return;
        }
    };

    println!(
        "{}. temp {:.1}{}, wind {:.1} {}",
        weather.icon,
        weather.temperature,
        unit.temp_unit(),
        unit.wind_speed(weather.windspeed),
        unit.wind_unit()
    );
}
