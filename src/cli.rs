use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short, long, default_value_t = 1000)]
    app_tick_rate: u64,
}
