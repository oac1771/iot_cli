use btleplug::{
    api::{Central, CentralEvent, Manager, Peripheral, ScanFilter},
    platform::{Adapter, Manager as PlatformManager, Peripheral as PlatformPeripheral},
};
use clap::Parser;
use futures::{Stream, stream::StreamExt};
use services::health::HEALTH_STATUS_CHAR_UUID;
use std::pin::Pin;
use uuid::Uuid;

const IOT_LOCAL_NAME: &str = "TrouBLE [Trouble Example]";

#[derive(Debug, Parser)]
pub struct ScanCmd;

impl ScanCmd {
    pub async fn handle(self) -> Result<(), ScanError> {

        println!("HealthServer Characteristic UUID: {:?}", HEALTH_STATUS_CHAR_UUID);

        let manager = PlatformManager::new().await?;
        let adapters = manager.adapters().await?;
        let central = adapters
            .into_iter()
            .nth(0)
            .ok_or(ScanError::AdapterNotFound)?;

        let mut events = central.events().await?;

        central.start_scan(ScanFilter::default()).await?;

        let peripheral = loop {
            if let Some(peripheral) = get_iot_peripheral(&mut events, &central).await {
                break peripheral;
            }
        };

        if let Err(err) = peripheral.connect().await {
            eprintln!("Error connecting: {}", err);
        }
        peripheral.discover_services().await.unwrap();

        for c in peripheral.characteristics() {
            let uuid = Uuid::from_bytes(*c.uuid.as_bytes());
            println!("{}", uuid);
        
            if uuid == HEALTH_STATUS_CHAR_UUID {
                println!("✅ Matched Health Status characteristic");
            }
        }

        // let characteristics = peripheral.characteristics();
        // let status_characteristic = characteristics
        //     .iter()
        //     .find(|c| c.uuid.as_bytes() == HEALTH_STATUS_CHARACTERISTICS.as_raw())
        //     .unwrap();

        // println!("{:?}", status_characteristic);

        // let status = peripheral.read(status_characteristic).await.unwrap();
        // println!("Status: {:?}", status);

        // let return_status = vec![false as u8];
        // peripheral
        //     .write(
        //         status_characteristic,
        //         &return_status,
        //         WriteType::WithoutResponse,
        //     )
        //     .await
        //     .unwrap();
        // println!("Successfully wrote to device");

        // std::thread::sleep(std::time::Duration::from_secs(100));
        Ok(())
    }
}

async fn get_iot_peripheral(
    events: &mut Pin<Box<dyn Stream<Item = CentralEvent> + Send>>,
    central: &Adapter,
) -> Option<PlatformPeripheral> {
    if let Some(CentralEvent::DeviceUpdated(id)) = events.next().await {
        let peripheral = central.peripheral(&id).await.ok()?;
        let properties = peripheral.properties().await.ok()??;
        let local_name = properties.local_name.unwrap_or_default();
        if local_name == IOT_LOCAL_NAME {
            return Some(peripheral);
        }
        None
    } else {
        None
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ScanError {
    #[error("{source}")]
    BtlePlug {
        #[from]
        source: btleplug::Error,
    },

    #[error("")]
    AdapterNotFound,
}
