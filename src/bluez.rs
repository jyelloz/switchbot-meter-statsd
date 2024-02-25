use zbus::proxy;
#[proxy(interface = "org.bluez.Adapter1", assume_defaults = true)]
trait Adapter1 {
    /// RemoveDevice method
    fn remove_device(&self, device: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// SetDiscoveryFilter method
    fn set_discovery_filter(
        &self,
        filter: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// StartDiscovery method
    fn start_discovery(&self) -> zbus::Result<()>;

    /// StopDiscovery method
    fn stop_discovery(&self) -> zbus::Result<()>;

    /// Address property
    #[zbus(property)]
    fn address(&self) -> zbus::Result<String>;

    /// Alias property
    #[zbus(property)]
    fn alias(&self) -> zbus::Result<String>;
    #[zbus(property)]
    fn set_alias(&self, value: &str) -> zbus::Result<()>;

    /// Class property
    #[zbus(property)]
    fn class(&self) -> zbus::Result<u32>;

    /// Discoverable property
    #[zbus(property)]
    fn discoverable(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn set_discoverable(&self, value: bool) -> zbus::Result<()>;

    /// DiscoverableTimeout property
    #[zbus(property)]
    fn discoverable_timeout(&self) -> zbus::Result<u32>;
    #[zbus(property)]
    fn set_discoverable_timeout(&self, value: u32) -> zbus::Result<()>;

    /// Discovering property
    #[zbus(property)]
    fn discovering(&self) -> zbus::Result<bool>;

    /// Modalias property
    #[zbus(property)]
    fn modalias(&self) -> zbus::Result<String>;

    /// Name property
    #[zbus(property)]
    fn name(&self) -> zbus::Result<String>;

    /// Pairable property
    #[zbus(property)]
    fn pairable(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn set_pairable(&self, value: bool) -> zbus::Result<()>;

    /// PairableTimeout property
    #[zbus(property)]
    fn pairable_timeout(&self) -> zbus::Result<u32>;
    #[zbus(property)]
    fn set_pairable_timeout(&self, value: u32) -> zbus::Result<()>;

    /// Powered property
    #[zbus(property)]
    fn powered(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn set_powered(&self, value: bool) -> zbus::Result<()>;

    /// UUIDs property
    #[zbus(property, name = "UUIDs")]
    fn uuids(&self) -> zbus::Result<Vec<String>>;
}

#[proxy(interface = "org.bluez.Device1", assume_defaults = true)]
trait Device1 {
    /// CancelPairing method
    fn cancel_pairing(&self) -> zbus::Result<()>;

    /// Connect method
    fn connect(&self) -> zbus::Result<()>;

    /// ConnectProfile method
    fn connect_profile(&self, UUID: &str) -> zbus::Result<()>;

    /// Disconnect method
    fn disconnect(&self) -> zbus::Result<()>;

    /// DisconnectProfile method
    fn disconnect_profile(&self, UUID: &str) -> zbus::Result<()>;

    /// Pair method
    fn pair(&self) -> zbus::Result<()>;

    /// Adapter property
    #[zbus(property)]
    fn adapter(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Address property
    #[zbus(property)]
    fn address(&self) -> zbus::Result<String>;

    /// Alias property
    #[zbus(property)]
    fn alias(&self) -> zbus::Result<String>;
    #[zbus(property)]
    fn set_alias(&self, value: &str) -> zbus::Result<()>;

    /// Appearance property
    #[zbus(property)]
    fn appearance(&self) -> zbus::Result<u16>;

    /// Blocked property
    #[zbus(property)]
    fn blocked(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn set_blocked(&self, value: bool) -> zbus::Result<()>;

    /// Class property
    #[zbus(property)]
    fn class(&self) -> zbus::Result<u32>;

    /// Connected property
    #[zbus(property)]
    fn connected(&self) -> zbus::Result<bool>;

    /// Icon property
    #[zbus(property)]
    fn icon(&self) -> zbus::Result<String>;

    /// LegacyPairing property
    #[zbus(property)]
    fn legacy_pairing(&self) -> zbus::Result<bool>;

    /// ManufacturerData property
    #[zbus(property)]
    fn manufacturer_data(
        &self,
    ) -> zbus::Result<std::collections::HashMap<u16, zbus::zvariant::OwnedValue>>;

    /// Modalias property
    #[zbus(property)]
    fn modalias(&self) -> zbus::Result<String>;

    /// Name property
    #[zbus(property)]
    fn name(&self) -> zbus::Result<String>;

    /// Paired property
    #[zbus(property)]
    fn paired(&self) -> zbus::Result<bool>;

    /// RSSI property
    #[zbus(property, name = "RSSI")]
    fn rssi(&self) -> zbus::Result<i16>;

    /// ServiceData property
    #[zbus(property)]
    fn service_data(
        &self,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// ServicesResolved property
    #[zbus(property)]
    fn services_resolved(&self) -> zbus::Result<bool>;

    /// Trusted property
    #[zbus(property)]
    fn trusted(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn set_trusted(&self, value: bool) -> zbus::Result<()>;

    /// TxPower property
    #[zbus(property)]
    fn tx_power(&self) -> zbus::Result<i16>;

    /// UUIDs property
    #[zbus(property, name = "UUIDs")]
    fn uuids(&self) -> zbus::Result<Vec<String>>;
}

#[proxy(interface = "org.bluez.GattService1", assume_defaults = true)]
trait GattService1 {
    /// Characteristics property
    #[zbus(property)]
    fn characteristics(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// Device property
    #[zbus(property)]
    fn device(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Primary property
    #[zbus(property)]
    fn primary(&self) -> zbus::Result<bool>;

    /// UUID property
    #[zbus(property, name = "UUID")]
    fn uuid(&self) -> zbus::Result<String>;
}

#[proxy(interface = "org.bluez.GattCharacteristic1", assume_defaults = true)]
trait GattCharacteristic1 {
    /// ReadValue method
    fn read_value(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<Vec<u8>>;

    /// StartNotify method
    fn start_notify(&self) -> zbus::Result<()>;

    /// StopNotify method
    fn stop_notify(&self) -> zbus::Result<()>;

    /// WriteValue method
    fn write_value(
        &self,
        value: &[u8],
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Descriptors property
    #[zbus(property)]
    fn descriptors(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// Flags property
    #[zbus(property)]
    fn flags(&self) -> zbus::Result<Vec<String>>;

    /// Notifying property
    #[zbus(property)]
    fn notifying(&self) -> zbus::Result<bool>;

    /// Service property
    #[zbus(property)]
    fn service(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// UUID property
    #[zbus(property, name = "UUID")]
    fn uuid(&self) -> zbus::Result<String>;

    /// Value property
    #[zbus(property)]
    fn value(&self) -> zbus::Result<Vec<u8>>;
}

#[proxy(interface = "org.bluez.GattDescriptor1", assume_defaults = true)]
trait GattDescriptor1 {
    /// ReadValue method
    fn read_value(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<Vec<u8>>;

    /// WriteValue method
    fn write_value(
        &self,
        value: &[u8],
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Characteristic property
    #[zbus(property)]
    fn characteristic(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// UUID property
    #[zbus(property, name = "UUID")]
    fn uuid(&self) -> zbus::Result<String>;

    /// Value property
    #[zbus(property)]
    fn value(&self) -> zbus::Result<Vec<u8>>;
}
