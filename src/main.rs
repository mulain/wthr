use clap::Parser;
use cli::CliArgs;
use reqwest::Client;
use serde::{Deserialize, Serialize};

fn main() {
    let args = CliArgs::parse();
    let client = Client::new();
    let lat = 52.52;
    let lon = 13.41;

    let url = format!(
        "https://api.open-meteo.com/v1/forecast?current_weather=true&latitude={}&longitude={}",
        lat, lon
    );
}
