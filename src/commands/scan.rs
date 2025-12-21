use clap::Parser;
use futures::{pin_mut, stream::StreamExt};
use services::{led::LED_STATUS_CHAR_UUID, health::HEALTH_STATUS_CHAR_UUID};

use crate::services::central::Central;

const IOT_LOCAL_NAME: &str = "TrouBLE [Trouble Example]";

#[derive(Debug, Parser)]
pub struct ScanCmd;

impl ScanCmd {
    pub async fn handle(self) -> Result<(), ScanError> {
        let central = Central::new().await.unwrap();
        let peripheral = central.find_peripheral(IOT_LOCAL_NAME).await.unwrap();

        // let stream = central
        //     .subscribe(&peripheral, HEALTH_STATUS_CHAR_UUID)
        //     .await
        //     .unwrap();
        // pin_mut!(stream);
        // while let Some(i) = stream.next().await {
        //     let value: bool = i.value[0] == 1;
        //     println!("Value: {:?}", value);
        // }

        central.write(&peripheral, LED_STATUS_CHAR_UUID, &vec![]).await.unwrap();
        println!("Successfully wrote to device");

        // std::thread::sleep(std::time::Duration::from_secs(100));
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ScanError {
    #[error("{source}")]
    BtlePlug {
        #[from]
        source: btleplug::Error,
    },
}
