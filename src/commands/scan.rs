use btleplug::api::Peripheral;
use clap::Parser;
use futures::stream::StreamExt;
use services::health::HEALTH_STATUS_CHAR_UUID;
use uuid::Uuid;

use crate::services::central::Central;

const IOT_LOCAL_NAME: &str = "TrouBLE [Trouble Example]";

#[derive(Debug, Parser)]
pub struct ScanCmd;

impl ScanCmd {
    pub async fn handle(self) -> Result<(), ScanError> {
        let central = Central::new().await.unwrap();
        let peripheral = central.find_peripheral(IOT_LOCAL_NAME).await.unwrap();

        if let Err(err) = peripheral.connect().await {
            eprintln!("Error connecting: {}", err);
        }
        peripheral.discover_services().await.unwrap();

        let characteristics = peripheral.characteristics();
        let status_characteristic = characteristics
            .iter()
            .find(|c| Uuid::from_bytes(*c.uuid.as_bytes()) == HEALTH_STATUS_CHAR_UUID)
            .unwrap();

        println!("{:?}", status_characteristic);

        peripheral.subscribe(status_characteristic).await.unwrap();
        while let Some(notification) = peripheral.notifications().await.unwrap().next().await {
            let value: bool = notification.value[0] == 1;
            println!("Value: {:?}", value);
        }

        // let status = peripheral.read(status_characteristic).await.unwrap();
        // println!("Status: {:?}", status);

        // let return_status = vec![false as u8];
        // peripheral
        //     .write(
        //         status_characteristic,
        //         &return_status,
        //         btleplug::api::WriteType::WithoutResponse,
        //     )
        //     .await
        //     .unwrap();
        // println!("Successfully wrote to device");

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
