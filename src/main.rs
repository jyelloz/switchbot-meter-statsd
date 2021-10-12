use std::{
    convert::{
        TryInto,
        TryFrom,
    },
    collections::HashMap,
    thread,
    time::Duration,
};
use zbus::{
    Connection,
    MessageHeader,
    MessageType,
    fdo,
};
use serde::{
    Serialize,
    Deserialize,
};
use zvariant::OwnedValue;
use zvariant_derive::Type;

mod adapter1;
mod models;
mod statsd_output;

use models::{
    SwitchbotThermometer,
    Reporter as _,
};
use statsd_output::StatsdReporter;

#[derive(Clone, Debug, Deserialize, Serialize, Type)]
struct ThermometerData {
    data: Vec<u8>,
}

impl std::convert::From<Vec<u8>> for ThermometerData {
    fn from(data: Vec<u8>) -> Self {
        Self { data }
    }
}

#[derive(Clone, Debug)]
enum Event {
    Updated(String, ThermometerData),
}

#[derive(Clone, Debug, Deserialize, Serialize, Type)]
struct PropertiesChanged {
    interface: String,
    changed_properties: HashMap<String, OwnedValue>,
    invalidated_properties: Vec<String>,
}

struct Proxy<'a> {
    connection: &'a Connection,
}

static DEVICE_1: &str = "org.bluez.Device1";
static PROPERTIES: &str = "org.freedesktop.DBus.Properties";
static PROPERTIES_CHANGED: &str = "PropertiesChanged";
static SERVICE_DATA: &str = "ServiceData";
static THERMOMETER_UUID: &str = "00000d00-0000-1000-8000-00805f9b34fb";

impl <'a> Proxy<'a> {

    fn is_signal(header: &MessageHeader) -> bool {
        header.message_type()
            .ok()
            .filter(|t| *t == MessageType::Signal)
            .is_some()
    }

    fn is_dbus_properties(header: &MessageHeader) -> bool {
        header.interface()
            .ok()
            .and_then(|option| option)
            .filter(|interface| *interface == PROPERTIES)
            .is_some()
    }

    fn is_dbus_properties_changed(header: &MessageHeader) -> bool {
        header.member()
            .ok()
            .and_then(|option| option)
            .filter(|member| *member == PROPERTIES_CHANGED)
            .is_some()
    }

    fn poll<F>(&self, callback: F) -> anyhow::Result<()>
    where F: FnOnce(Event)
    {
        loop {
            let msg = self.connection.receive_message()?;
            let header = msg.header()?;
            if !Self::is_signal(&header) {
                continue;
            }
            if !Self::is_dbus_properties(&header) {
                continue;
            }
            if !Self::is_dbus_properties_changed(&header) {
                continue;
            }
            let mut body: PropertiesChanged = msg.body()?;
            if DEVICE_1 != body.interface {
                continue;
            }
            let path = header.path()?.unwrap();
            let service_data = body.changed_properties.remove(SERVICE_DATA);
            if let Some(service_data) = service_data {
                let dict: zvariant::Dict = service_data.try_into()?;
                let mut dict: HashMap<String, Vec<u8>> = dict.try_into()?;
                let data = dict.remove(THERMOMETER_UUID)
                    .map(ThermometerData::from);
                if let Some(data) = data {
                    callback(Event::Updated(path.as_str().into(), data));
                }
            }
            break;
        }
        Ok(())
    }
}

fn mac_address_from_dbus_path(path: &str) -> String {
    path.split("/")
        .last()
        .unwrap()
        .split("_")
        .skip(1)
        .collect::<Vec<&str>>()
        .join(":")
        .to_uppercase()
}

fn main() -> anyhow::Result<()> {
    let system = Connection::new_system()?;
    thread::spawn(ensure_discovering_task);
    {
        let dbus_proxy = fdo::DBusProxy::new(&system)?;
        dbus_proxy.add_match(
            "type='signal',interface='org.freedesktop.DBus.Properties',member='PropertiesChanged',path_namespace='/org/bluez'"
        )?;
    }
    let proxy = Proxy { connection: &system };
    let statsd = StatsdReporter::try_default()?;
    loop {
        proxy.poll(
            |event|
            match event {
                Event::Updated(path, data) => {
                    let device_id = mac_address_from_dbus_path(&path);
                    let device = SwitchbotThermometer::try_from(
                        (device_id.clone(), data.data.as_slice())
                    ).unwrap();
                    println!(
                        "{} {} {} {}",
                        &device_id,
                        device.c().0,
                        device.humidity,
                        device.battery,
                    );
                    statsd.report(&device).ok();
                }
            }
        )?;
    }
}

fn ensure_discovering_task() {
    let system = Connection::new_system()
        .expect("failed to get system connection");
    let adapter = adapter1::Adapter1Proxy::new(&system)
        .expect("failed to get Bluetooth Adapter proxy");
    loop {
        if let Err(e) = ensure_discovering(&adapter) {
            eprintln!("failed to ensure adapter is discovering: {:?}", e);
        }
        thread::sleep(Duration::from_secs(30));
    }
}

fn ensure_discovering(adapter: &adapter1::Adapter1Proxy) -> anyhow::Result<()> {
    if !adapter.powered()? {
        adapter.set_powered(true)?;
    }
    if !adapter.discovering()? {
        adapter.start_discovery()?;
    }
    Ok(())
}
