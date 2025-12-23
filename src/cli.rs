use crate::commands::{led::LedCmd, status::StatusCmd};
use clap::{Parser, Subcommand};
use tracing::{error, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, fmt, prelude::*, reload::Layer};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Status(StatusCmd),
    Led(LedCmd),
}

pub async fn run() {
    let args = Cli::parse();

    if let Err(err) = init_tracing() {
        error!("Error: {}", err);
        return;
    }

    let result = match args.command {
        Command::Status(cmd) => cmd.handle().await.map_err(|err| err.to_string()),
        Command::Led(cmd) => cmd.handle().await.map_err(|err| err.to_string()),
    };

    if let Err(err) = result {
        error!("Error: {}", err)
    }
}

fn init_tracing() -> Result<(), String> {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env()
        .map_err(|err| err.to_string())?;
    let (layer, _) = Layer::new(filter);
    tracing_subscriber::registry()
        .with(layer)
        .with(fmt::Layer::default())
        .init();

    Ok(())
}
