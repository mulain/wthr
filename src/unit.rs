#[derive(Debug, Clone, Copy)]
pub enum Unit {
    Metric,
    Imperial,
    Kelvin,
}

impl Unit {
    pubfn to_string(&self) -> &str {
        match self {
            Unit::Metric => "metric",
            Unit::Imperial => "imperial",
            Unit::Kelvin => "kelvin",
        }
    }
    pub fn as_api_param(&self) -> &'static str {
        match self {
            Unit::Metric => "celsius",
            Unit::Imperial => "fahrenheit",
            Unit::Kelvin => "kelvin",
        }
    }
}