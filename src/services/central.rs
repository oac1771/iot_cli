use btleplug::{
    api::{
        Central as ApiCentral, CentralEvent, Manager, Peripheral, ScanFilter, ValueNotification,
    },
    platform::{Adapter, Manager as PlatformManager, Peripheral as PlatformPeripheral},
};
use futures::{Stream, StreamExt};
use uuid::Uuid;

pub struct Central(Adapter);

impl Central {
    pub async fn new() -> Result<Self, Error> {
        let manager = PlatformManager::new().await?;
        let adapters = manager.adapters().await?;
        let central = adapters.into_iter().nth(0).ok_or(Error::AdapterNotFound)?;

        Ok(Self(central))
    }

    pub async fn find_peripheral(&self, local_name: &str) -> Result<PlatformPeripheral, Error> {
        let mut events = self.0.events().await?;
        self.0.start_scan(ScanFilter::default()).await?;

        let peripheral = loop {
            if let Some(CentralEvent::DeviceUpdated(id)) = events.next().await {
                let peripheral = self.0.peripheral(&id).await?;
                let properties = peripheral
                    .properties()
                    .await?
                    .ok_or_else(|| Error::PeripheralPropertiesNotFound)?;

                if properties
                    .local_name
                    .ok_or_else(|| Error::LocalNameNotFound)?
                    == local_name
                {
                    break peripheral;
                }
            }
        };

        peripheral.connect().await?;
        peripheral.discover_services().await?;

        Ok(peripheral)
    }

    pub async fn subscribe(
        &self,
        peripheral: &PlatformPeripheral,
        characteristic_uuid: Uuid,
    ) -> Result<impl Stream<Item = ValueNotification>, Error> {
        let characteristics = peripheral.characteristics();
        let characteristic = characteristics
            .iter()
            .find(|c| Uuid::from_bytes(*c.uuid.as_bytes()) == characteristic_uuid)
            .ok_or_else(|| Error::CharacteristicNotFound)?;

        peripheral.subscribe(characteristic).await?;

        let stream = peripheral.notifications().await?.filter(move |n| {
            let notif_uuid = Uuid::from_bytes(*n.uuid.as_bytes());
            let characteristic_uuid = characteristic_uuid;

            async move { notif_uuid == characteristic_uuid }
        });

        Ok(stream)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{source}")]
    BtlePlug {
        #[from]
        source: btleplug::Error,
    },

    #[error("")]
    AdapterNotFound,

    #[error("")]
    PeripheralPropertiesNotFound,

    #[error("")]
    LocalNameNotFound,

    #[error("")]
    CharacteristicNotFound,
}
