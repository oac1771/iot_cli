use std::pin::Pin;

use btleplug::{
    api::{Central, CentralEvent, Manager, Peripheral, ScanFilter, WriteType},
    platform::{Adapter, Manager as PlatformManager, Peripheral as PlatformPeripheral},
};
use clap::Parser;
use futures::{Stream, stream::StreamExt};
use tracing::info;

const IOT_LOCAL_NAME: &str = "TrouBLE [Trouble Example]";
const IOT_BATTERY_SERVICE_STATUS_UUID: &str = "408813df-5dd4-1f87-ec11-cdb001100000";

#[derive(Debug, Parser)]
pub struct ScanCmd {
    #[arg(long, default_value = "8000")]
    port: String,
}

impl ScanCmd {
    pub async fn handle(self) -> Result<(), ScanError> {
        let manager = PlatformManager::new().await?;
        let adapters = manager.adapters().await?;
        let central = adapters
            .into_iter()
            .nth(0)
            .ok_or(ScanError::AdapterNotFound)?;

        let central_state = central.adapter_state().await?;
        info!("CentralState: {:?}", central_state);

        let mut events = central.events().await?;

        central.start_scan(ScanFilter::default()).await?;

        let peripheral = loop {
            if let Some(peripheral) = get_iot_peripheral(&mut events, &central).await {
                // let properties = peripheral.properties().await.unwrap().unwrap();
                // info!("Found device: {:?}", properties);
                break peripheral;
            }
        };

        if let Err(err) = peripheral.connect().await {
            eprintln!("Error connecting: {}", err);
        }
        peripheral.discover_services().await.unwrap();
        let characteristics = peripheral.characteristics();
        let status_characteristic = characteristics
            .iter()
            .find(|c| c.uuid.to_string() == IOT_BATTERY_SERVICE_STATUS_UUID)
            .unwrap();
        println!("{:?}", status_characteristic);

        let status = peripheral.read(status_characteristic).await.unwrap();
        println!("Status: {:?}", status);

        let return_status = vec![false as u8];
        peripheral.write(status_characteristic, &return_status, WriteType::WithoutResponse).await.unwrap();
        println!("Successfully wrote to device");

        std::thread::sleep(std::time::Duration::from_secs(100));
        Ok(())
    }
}

async fn get_iot_peripheral(
    events: &mut Pin<Box<dyn Stream<Item = CentralEvent> + Send>>,
    central: &Adapter,
) -> Option<PlatformPeripheral> {
    if let Some(CentralEvent::DeviceDiscovered(id)) = events.next().await {
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
