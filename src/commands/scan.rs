use std::pin::Pin;

use btleplug::{
    api::{Central, CentralEvent, Manager, Peripheral, ScanFilter},
    platform::{Adapter, Manager as PlatformManager, Peripheral as PlatformPeripheral },
};
use clap::Parser;
use futures::{Stream, stream::StreamExt};
use tracing::info;

const IOT_LOCAL_NAME: &str = "Trouble Advert";

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

        loop {
            if let Some(peripheral) = get_iot_peripheral(&mut events, &central).await {
                let properties = peripheral.properties().await.unwrap().unwrap();
                info!("Found device: {:?}", properties);
                break 
            }
        };

        Ok(())
    }
}

async fn get_iot_peripheral(events: &mut Pin<Box<dyn Stream<Item = CentralEvent> + Send>>, central: &Adapter) -> Option<PlatformPeripheral> {
    if let Some(event) = events.next().await {
        if let CentralEvent::DeviceDiscovered(id) = event {
            let peripheral = central.peripheral(&id).await.ok()?;
            let properties = peripheral.properties().await.ok()??;
            let local_name = properties.local_name.unwrap_or_default();

            if local_name == IOT_LOCAL_NAME {
                return Some(peripheral)
            }
            return None
        } else {
            return None
        }
    } else {
        return None
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
