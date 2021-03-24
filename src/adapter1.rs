use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "org.bluez.Adapter1",
    default_service = "org.bluez",
    default_path = "/org/bluez/hci0",
)]
pub trait Adapter1 {
    /// GetDiscoveryFilters method
    fn get_discovery_filters(&self) -> zbus::Result<Vec<String>>;

    /// RemoveDevice method
    fn remove_device(&self, device: &zvariant::ObjectPath) -> zbus::Result<()>;

    /// SetDiscoveryFilter method
    fn set_discovery_filter(
        &self,
        properties: std::collections::HashMap<&str, zvariant::Value>,
    ) -> zbus::Result<()>;

    /// StartDiscovery method
    fn start_discovery(&self) -> zbus::Result<()>;

    /// StopDiscovery method
    fn stop_discovery(&self) -> zbus::Result<()>;

    /// Address property
    #[dbus_proxy(property)]
    fn address(&self) -> zbus::fdo::Result<String>;

    /// AddressType property
    #[dbus_proxy(property)]
    fn address_type(&self) -> zbus::fdo::Result<String>;

    /// Alias property
    #[dbus_proxy(property)]
    fn alias(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property)]
    fn set_alias(&self, value: &str) -> zbus::fdo::Result<()>;

    /// Class property
    #[dbus_proxy(property)]
    fn class(&self) -> zbus::fdo::Result<u32>;

    /// Discoverable property
    #[dbus_proxy(property)]
    fn discoverable(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn set_discoverable(&self, value: bool) -> zbus::fdo::Result<()>;

    /// DiscoverableTimeout property
    #[dbus_proxy(property)]
    fn discoverable_timeout(&self) -> zbus::fdo::Result<u32>;
    #[dbus_proxy(property)]
    fn set_discoverable_timeout(&self, value: u32) -> zbus::fdo::Result<()>;

    /// Discovering property
    #[dbus_proxy(property)]
    fn discovering(&self) -> zbus::fdo::Result<bool>;

    /// Modalias property
    #[dbus_proxy(property)]
    fn modalias(&self) -> zbus::fdo::Result<String>;

    /// Name property
    #[dbus_proxy(property)]
    fn name(&self) -> zbus::fdo::Result<String>;

    /// Pairable property
    #[dbus_proxy(property)]
    fn pairable(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn set_pairable(&self, value: bool) -> zbus::fdo::Result<()>;

    /// PairableTimeout property
    #[dbus_proxy(property)]
    fn pairable_timeout(&self) -> zbus::fdo::Result<u32>;
    #[dbus_proxy(property)]
    fn set_pairable_timeout(&self, value: u32) -> zbus::fdo::Result<()>;

    /// Powered property
    #[dbus_proxy(property)]
    fn powered(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn set_powered(&self, value: bool) -> zbus::fdo::Result<()>;

    /// Roles property
    #[dbus_proxy(property)]
    fn roles(&self) -> zbus::fdo::Result<Vec<String>>;

    /// UUIDs property
    #[dbus_proxy(property)]
    fn uuids(&self) -> zbus::fdo::Result<Vec<String>>;
}
#[dbus_proxy(interface = "org.bluez.GattManager1")]
trait GattManager1 {
    /// RegisterApplication method
    fn register_application(
        &self,
        application: &zvariant::ObjectPath,
        options: std::collections::HashMap<&str, zvariant::Value>,
    ) -> zbus::Result<()>;

    /// UnregisterApplication method
    fn unregister_application(&self, application: &zvariant::ObjectPath) -> zbus::Result<()>;
}
#[dbus_proxy(interface = "org.bluez.LEAdvertisingManager1")]
trait LEAdvertisingManager1 {
    /// RegisterAdvertisement method
    fn register_advertisement(
        &self,
        advertisement: &zvariant::ObjectPath,
        options: std::collections::HashMap<&str, zvariant::Value>,
    ) -> zbus::Result<()>;

    /// UnregisterAdvertisement method
    fn unregister_advertisement(&self, service: &zvariant::ObjectPath) -> zbus::Result<()>;

    /// ActiveInstances property
    #[dbus_proxy(property)]
    fn active_instances(&self) -> zbus::fdo::Result<u8>;

    /// SupportedIncludes property
    #[dbus_proxy(property)]
    fn supported_includes(&self) -> zbus::fdo::Result<Vec<String>>;

    /// SupportedInstances property
    #[dbus_proxy(property)]
    fn supported_instances(&self) -> zbus::fdo::Result<u8>;

    /// SupportedSecondaryChannels property
    #[dbus_proxy(property)]
    fn supported_secondary_channels(&self) -> zbus::fdo::Result<Vec<String>>;
}
#[dbus_proxy(interface = "org.bluez.Media1")]
trait Media1 {
    /// RegisterApplication method
    fn register_application(
        &self,
        application: &zvariant::ObjectPath,
        options: std::collections::HashMap<&str, zvariant::Value>,
    ) -> zbus::Result<()>;

    /// RegisterEndpoint method
    fn register_endpoint(
        &self,
        endpoint: &zvariant::ObjectPath,
        properties: std::collections::HashMap<&str, zvariant::Value>,
    ) -> zbus::Result<()>;

    /// RegisterPlayer method
    fn register_player(
        &self,
        player: &zvariant::ObjectPath,
        properties: std::collections::HashMap<&str, zvariant::Value>,
    ) -> zbus::Result<()>;

    /// UnregisterApplication method
    fn unregister_application(&self, application: &zvariant::ObjectPath) -> zbus::Result<()>;

    /// UnregisterEndpoint method
    fn unregister_endpoint(&self, endpoint: &zvariant::ObjectPath) -> zbus::Result<()>;

    /// UnregisterPlayer method
    fn unregister_player(&self, player: &zvariant::ObjectPath) -> zbus::Result<()>;
}
#[dbus_proxy(interface = "org.bluez.NetworkServer1")]
trait NetworkServer1 {
    /// Register method
    fn register(&self, uuid: &str, bridge: &str) -> zbus::Result<()>;

    /// Unregister method
    fn unregister(&self, uuid: &str) -> zbus::Result<()>;
}
