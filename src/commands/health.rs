use super::IOT_LOCAL_NAME;
use btleplug::platform::Peripheral;
use clap::{Parser, Subcommand};
use futures::{pin_mut, stream::StreamExt};
use services::{trouble_host::types::gatt_traits::FromGatt, health::{HEALTH_PING_CHAR_UUID, HEALTH_STATUS_CHAR_UUID, Pong}};

use crate::services::central::Central;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct HealthCmd {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Status,
    Ping,
}

impl HealthCmd {
    pub async fn handle(self) -> Result<(), Error> {
        let central = Central::new().await?;
        let peripheral = central.find_peripheral(IOT_LOCAL_NAME).await?;

        match self.command {
            Command::Status => self.handle_status(central, &peripheral).await,
            Command::Ping => self.handle_ping(central, &peripheral).await
        }
    }

    async fn handle_status(&self, central: Central, peripheral: &Peripheral) -> Result<(), Error> {
        let data = central.read(peripheral, HEALTH_STATUS_CHAR_UUID).await?;
        let value = bool::from_gatt(&data).unwrap();
        println!("Successfully read from device: {}", value);

        Ok(())
    }

    async fn handle_ping(&self, central: Central, peripheral: &Peripheral) -> Result<(), Error> {
        let stream = central
            .subscribe(&peripheral, HEALTH_PING_CHAR_UUID)
            .await?;
        pin_mut!(stream);
        while let Some(i) = stream.next().await {
            let value = Pong::from_gatt(&i.value).unwrap();
            println!("Value: {}", value);
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
