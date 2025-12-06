use std::pin::Pin;

use btleplug::{
    api::{Central, CentralEvent, Manager, Peripheral, ScanFilter},
    platform::{Adapter, Manager as PlatformManager, Peripheral as PlatformPeripheral},
};
use clap::Parser;
use futures::{Stream, stream::StreamExt};
use tracing::info;

const IOT_LOCAL_NAME: &str = "TrouBLE [Trouble Example]";

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
                let properties = peripheral.properties().await.unwrap().unwrap();
                info!("Found device: {:?}", properties);
                break peripheral;
            }
        };

        peripheral.connect().await.unwrap();

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
