use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "wthr",
    about = "A simple weather app"
)]
pub struct CliArgs {
    city: String,
    #[arg(short, long, default_value_t = Unit::Metric)]
    unit: Unit,

}

