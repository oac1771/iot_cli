use btleplug::{
    api::{Central, CentralEvent, Manager, Peripheral, ScanFilter, bleuuid::BleUuid},
    platform::Manager as PlatformManager,
};
use clap::Parser;
use futures::stream::StreamExt;
use tracing::info;

// const IOT_LOCAL_NAME: &str = "iot";

#[derive(Debug, Parser)]
pub struct ScanCmd {
    #[arg(long, default_value = "8000")]
    port: String,
}

impl ScanCmd {
    pub async fn handle(self) -> Result<(), ScanError> {
        let manager = PlatformManager::new().await?;
        let adapters = manager.adapters().await?;
        let central = adapters.into_iter().nth(0).ok_or(ScanError::AdapterNotFound)?;

        let central_state = central.adapter_state().await?;
        info!("CentralState: {:?}", central_state);

        let mut events = central.events().await?;

        central.start_scan(ScanFilter::default()).await?;

        while let Some(event) = events.next().await {
            match event {
                CentralEvent::DeviceDiscovered(id) => {
                    let peripheral = central.peripheral(&id).await.unwrap();
                    let properties = peripheral.properties().await.unwrap();
                    let name = properties
                        .and_then(|p| p.local_name)
                        .map(|local_name| format!("Name: {local_name}"))
                        .unwrap_or_default();
                    info!("DeviceDiscovered: {:?} {}", id, name);
                }
                CentralEvent::StateUpdate(state) => {
                    info!("AdapterStatusUpdate {:?}", state);
                }
                CentralEvent::DeviceConnected(id) => {
                    info!("DeviceConnected: {:?}", id);
                }
                CentralEvent::DeviceDisconnected(id) => {
                    info!("DeviceDisconnected: {:?}", id);
                }
                CentralEvent::ServiceDataAdvertisement { id, service_data } => {
                    info!("ServiceDataAdvertisement: {:?}, {:?}", id, service_data);
                }
                CentralEvent::ServicesAdvertisement { id, services } => {
                    let services: Vec<String> =
                        services.into_iter().map(|s| s.to_short_string()).collect();
                    info!("ServicesAdvertisement: {:?}, {:?}", id, services);
                }
                _ => {}
            }
        }
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

    #[error("")]
    AdapterNotFound
}
