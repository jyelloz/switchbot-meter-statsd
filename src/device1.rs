use std::{
    collections::HashMap,
    convert::{
        TryFrom,
        TryInto,
    },
};
use zbus::dbus_proxy;
use zvariant::{
    Dict,
    OwnedValue,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceData {
    data: HashMap<String, Vec<u8>>,
}

impl ServiceData {
    pub fn get(&self, key: &str) -> Option<&Vec<u8>> {
        self.data.get(key)
    }
}

impl TryFrom<OwnedValue> for ServiceData {
    type Error = <u32 as TryFrom<OwnedValue>>::Error;
    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        let data: Dict<'_, '_> = value.try_into()?;
        let data = data.try_into()?;
        Ok(ServiceData { data })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManufacturerData {
    data: HashMap<u16, Vec<u8>>,
}

impl TryFrom<OwnedValue> for ManufacturerData {
    type Error = <u32 as TryFrom<OwnedValue>>::Error;
    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        let data: Dict<'_, '_> = value.try_into()?;
        let data = data.try_into()?;
        Ok(ManufacturerData { data })
    }
}

#[dbus_proxy(
    interface = "org.bluez.Device1",
    default_service = "org.bluez",
)]
pub trait Device1 {
    /// CancelPairing method
    fn cancel_pairing(&self) -> zbus::Result<()>;

    /// Connect method
    fn connect(&self) -> zbus::Result<()>;

    /// ConnectProfile method
    fn connect_profile(&self, uuid: &str) -> zbus::Result<()>;

    /// Disconnect method
    fn disconnect(&self) -> zbus::Result<()>;

    /// DisconnectProfile method
    fn disconnect_profile(&self, uuid: &str) -> zbus::Result<()>;

    /// Pair method
    fn pair(&self) -> zbus::Result<()>;

    /// Adapter property
    #[dbus_proxy(property)]
    fn adapter(&self) -> zbus::fdo::Result<zvariant::ObjectPath>;

    /// Address property
    #[dbus_proxy(property)]
    fn address(&self) -> zbus::fdo::Result<String>;

    /// AddressType property
    #[dbus_proxy(property)]
    fn address_type(&self) -> zbus::fdo::Result<String>;

    /// Alias property
    #[dbus_proxy(property)]
    fn alias(&self) -> zbus::fdo::Result<String>;
    #[DBusProxy(property)]
    fn set_alias(&self, value: &str) -> zbus::fdo::Result<()>;

    /// Appearance property
    #[dbus_proxy(property)]
    fn appearance(&self) -> zbus::fdo::Result<u16>;

    /// Blocked property
    #[dbus_proxy(property)]
    fn blocked(&self) -> zbus::fdo::Result<bool>;
    #[DBusProxy(property)]
    fn set_blocked(&self, value: bool) -> zbus::fdo::Result<()>;

    /// Class property
    #[dbus_proxy(property)]
    fn class(&self) -> zbus::fdo::Result<u32>;

    /// Connected property
    #[dbus_proxy(property)]
    fn connected(&self) -> zbus::fdo::Result<bool>;

    /// Icon property
    #[dbus_proxy(property)]
    fn icon(&self) -> zbus::fdo::Result<String>;

    /// LegacyPairing property
    #[dbus_proxy(property)]
    fn legacy_pairing(&self) -> zbus::fdo::Result<bool>;

    /// ManufacturerData property
    #[dbus_proxy(property)]
    fn manufacturer_data(
        &self,
    ) -> zbus::fdo::Result<ManufacturerData>;

    /// Modalias property
    #[dbus_proxy(property)]
    fn modalias(&self) -> zbus::fdo::Result<String>;

    /// Name property
    #[dbus_proxy(property)]
    fn name(&self) -> zbus::fdo::Result<String>;

    /// Paired property
    #[dbus_proxy(property)]
    fn paired(&self) -> zbus::fdo::Result<bool>;

    /// RSSI property
    #[dbus_proxy(property)]
    fn rssi(&self) -> zbus::fdo::Result<i16>;

    /// ServiceData property
    #[dbus_proxy(property)]
    fn service_data(
        &self,
    ) -> zbus::fdo::Result<ServiceData>;

    /// ServicesResolved property
    #[dbus_proxy(property)]
    fn services_resolved(&self) -> zbus::fdo::Result<bool>;

    /// Trusted property
    #[dbus_proxy(property)]
    fn trusted(&self) -> zbus::fdo::Result<bool>;
    #[DBusProxy(property)]
    fn set_trusted(&self, value: bool) -> zbus::fdo::Result<()>;

    /// TxPower property
    #[dbus_proxy(property)]
    fn tx_power(&self) -> zbus::fdo::Result<i16>;

    /// UUIDs property
    #[dbus_proxy(property)]
    fn uuids(&self) -> zbus::fdo::Result<Vec<String>>;

    /// WakeAllowed property
    #[dbus_proxy(property)]
    fn wake_allowed(&self) -> zbus::fdo::Result<bool>;
    #[DBusProxy(property)]
    fn set_wake_allowed(&self, value: bool) -> zbus::fdo::Result<()>;
}
