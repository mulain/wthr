use crate::unit::Unit;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "wthr", about = "Minimal weather CLI using Open-Meteo API")]
pub struct CliArgs {
    /// City to retrieve information for (optional - will use geolocation if not provided)
    pub city: Option<String>,

    /// Output in imperial units (default: metric)
    #[arg(short = 'i', long)]
    pub imperial: bool,
}

impl CliArgs {
    pub fn unit(&self) -> Unit {
        if self.imperial {
            Unit::Imperial
        } else {
            Unit::Metric
        }
    }
}
