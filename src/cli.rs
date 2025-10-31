use crate::unit::Unit;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "wthr", about = "Minimal weather CLI using Open-Meteo API")]
pub struct CliArgs {
    /// City to retrieve information for (optional - will use geolocation if not provided)
    pub city: Option<String>,

    /// Unit for temperature
    #[arg(short, long, value_enum, default_value_t = Unit::Metric)]
    pub unit: Unit,
}
