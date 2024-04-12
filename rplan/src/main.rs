mod analyzer;
mod context;
mod db;
mod edz;
mod error;
mod model;
mod queue;
mod trace_layer;

use clap::Parser;
use std::net::IpAddr;
use std::path::PathBuf;
use url::Url;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, env = "RPLAN_ADDRESS", default_value_t = IpAddr::from([127, 0, 0, 1]))]
    addr: IpAddr,
    #[arg(short, long, env = "RPLAN_PORT", default_value_t = 80)]
    port: u16,
    #[arg(short, long, env = "RPLAN_BODY_LIMIT", default_value_t = 1024 * 1024 * 1024)]
    body_limit: usize,
    #[arg(long, env = "RPLAN_ANALYZER_TASKS", value_parser = clap::value_parser!(u8).range(1..), default_value_t = 3)]
    analyzer_tasks: u8,
    #[arg(short, long, env = "RPLAN_URL")]
    url: Url,
    #[arg(short, long, env = "RPLAN_MONGODB_URI", default_value_t = String::from("mongodb://localhost:27017"))]
    mongodb_uri: String,
    #[arg(short, long, env = "RPLAN_DATA_PATH", value_name = "DIRECTORY")]
    data_path: PathBuf,
}

fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    tracing_subscriber::registry()
        .with(fmt::layer().with_target(false))
        .with(EnvFilter::from_default_env())
        .with(ErrorLayer::default())
        .init();
}

fn main() {
    let cli = Cli::parse();

    install_tracing();
}
