use clap::Parser;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, fmt, prelude::*, reload::Layer};

use super::error::CommandError;

#[derive(Debug, Parser)]
pub struct StartCmd {
    #[arg(long, default_value = "8000")]
    rpc_port: String,

    #[arg(long, default_value = "0.0.0.0")]
    rpc_ip: String,
}

impl StartCmd {
    pub async fn handle(self) -> Result<(), CommandError> {
        Self::init_tracing()?;

        Ok(())
    }

    fn init_tracing() -> Result<(), CommandError> {
        let filter = EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env()?;
        let (layer, _) = Layer::new(filter);
        tracing_subscriber::registry()
            .with(layer)
            .with(fmt::Layer::default())
            .init();

        Ok(())
    }

}
