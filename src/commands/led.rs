use super::IOT_LOCAL_NAME;
use clap::Parser;
use services::led::LED_STATUS_CHAR_UUID;

use crate::services::central::Central;

#[derive(Debug, Parser)]
pub struct LedCmd;

impl LedCmd {
    pub async fn handle(self) -> Result<(), Error> {
        let central = Central::new().await.unwrap();
        let peripheral = central.find_peripheral(IOT_LOCAL_NAME).await?;

        central
            .write(&peripheral, LED_STATUS_CHAR_UUID, &vec![])
            .await?;
        println!("Successfully wrote to device");

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{source}")]
    BtlePlug {
        #[from]
        source: btleplug::Error,
    },

    #[error("{source}")]
    Central {
        #[from]
        source: crate::services::central::Error,
    },
}
