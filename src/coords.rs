use reqwest::Client;
use serde::Deserialize;
use anyhow::anyhow;

#[derive(Deserialize)]
struct GeoApiResponse {
    results: Vec<GeoResult>,
}

#[derive(Deserialize)]
struct GeoResult {
    latitude: f64,
    longitude: f64,
}

pub async fn get_coordinates(client: &Client, city: &str) -> Result<(f64, f64), anyhow::Error> {
    let url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1",
        city
    );

    let response: GeoApiResponse = client
        .get(&url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let coords = response
        .results
        .get(0)
        .ok_or_else(|| anyhow!("City not found: {}", city))?;

    Ok((coords.latitude, coords.longitude))
}

#[derive(Deserialize)]
struct IpApiResponse {
    lat: f64,
    lon: f64,
}

pub async fn get_coordinates_from_ip(client: &Client) -> Result<(f64, f64), anyhow::Error> {
    let url = "http://ip-api.com/json/?fields=lat,lon";

    let response: IpApiResponse = client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok((response.lat, response.lon))
}
