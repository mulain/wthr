mod cli;
mod unit;
mod coords;
mod weather;

use clap::Parser;
use cli::CliArgs;
use reqwest::Client;
use weather::fetch_weather;
use coords::get_coordinates;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();
    let client = Client::new();

    let (lat, lon) = match get_coordinates(&client, &args.city).await {
        Ok(coords) => coords,
        Err(_) => {
            eprintln!("Error: Could not find city '{}'", args.city);
            return;
        }
    };

    let weather = match fetch_weather(&client, lat, lon, args.unit).await {
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
        args.unit.temp_unit(),
        args.unit.wind_speed(weather.windspeed),
        args.unit.wind_unit()
    );
}
