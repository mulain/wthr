use clap::ValueEnum;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Unit {
    Metric,
    Imperial,
}

impl Unit {
    pub fn temp_unit(&self) -> &'static str {
        match self {
            Unit::Metric => "°C",
            Unit::Imperial => "°F",
        }
    }

    pub fn wind_speed(&self, kmh: f64) -> f64 {
        match self {
            Unit::Metric => kmh,
            Unit::Imperial => kmh * 0.621371,
        }
    }

    pub fn wind_unit(&self) -> &'static str {
        match self {
            Unit::Metric => "km/h",
            Unit::Imperial => "mph",
        }
    }

    pub fn as_api_param(&self) -> &'static str {
        match self {
            Unit::Metric => "celsius",
            Unit::Imperial => "fahrenheit",
        }
    }
}
