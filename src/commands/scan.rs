use clap::Parser;

use super::error::CommandError;

#[derive(Debug, Parser)]
pub struct ScanCmd {
    #[arg(long, default_value = "8000")]
    port: String,
}

impl ScanCmd {
    pub async fn handle(self) -> Result<(), CommandError> {
        tracing::info!("Scanning...");
        Ok(())
    }
}
