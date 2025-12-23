use super::IOT_LOCAL_NAME;
use clap::Parser;
use futures::{pin_mut, stream::StreamExt};
use services::health::HEALTH_STATUS_CHAR_UUID;

use crate::services::central::Central;

#[derive(Debug, Parser)]
pub struct StatusCmd;

impl StatusCmd {
    pub async fn handle(self) -> Result<(), Error> {
        let central = Central::new().await?;
        let peripheral = central.find_peripheral(IOT_LOCAL_NAME).await?;

        central.read(&peripheral, HEALTH_STATUS_CHAR_UUID).await?;
        println!("Successfully read from device");

        let stream = central
            .subscribe(&peripheral, HEALTH_STATUS_CHAR_UUID)
            .await?;
        pin_mut!(stream);
        while let Some(i) = stream.next().await {
            let value: bool = i.value[0] == 1;
            println!("Value: {:?}", value);
        }

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
