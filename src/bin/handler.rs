use std::{
    convert::TryInto,
    convert::TryFrom,
    collections::HashMap,
};
use zbus::{
    Connection,
    MessageHeader,
    MessageType,
    fdo::DBusProxy,
};
use serde::{
    Serialize,
    Deserialize,
};
use zvariant::OwnedValue;
use zvariant_derive::Type;

use bluezadvertising::{
    models::SwitchbotThermometer,
    statsd_output::statsd_output,
    adapter1::Adapter1Proxy,
};

#[derive(Clone, Debug, Deserialize, Serialize, Type)]
struct ThermometerData {
    data: Vec<u8>,
}

#[derive(Clone, Debug)]
enum Event {
    Updated(String, ThermometerData),
}

struct Proxy<'a> {
    connection: &'a Connection,
}

static DEVICE_1: &str = "org.bluez.Device1";
static PROPERTIES: &str = "org.freedesktop.DBus.Properties";
static PROPERTIES_CHANGED: &str = "PropertiesChanged";
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
            let service_data = body.changed_properties.remove("ServiceData");
            if let Some(service_data) = service_data {
                let dict: zvariant::Dict = service_data.try_into()?;
                let mut dict: HashMap<String, Vec<u8>> = dict.try_into()?;
                let data = dict.remove(THERMOMETER_UUID);
                if let Some(data) = data {
                    callback(
                        Event::Updated(
                            path.as_str().into(),
                            ThermometerData { data },
                        )
                    );
                }
            }
            break;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Type)]
struct PropertiesChanged {
    interface: String,
    changed_properties: HashMap<String, OwnedValue>,
    invalidated_properties: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let system = Connection::new_system()?;
    let adapter = Adapter1Proxy::new(&system)?;
    {
        let dbus_proxy = DBusProxy::new(&system)?;
        dbus_proxy.add_match(
            "type='signal',interface='org.freedesktop.DBus.Properties',member='PropertiesChanged',path_namespace='/org/bluez'"
        )?;
    }
    let proxy = Proxy { connection: &system };
    loop {
        ensure_discovering(&adapter)?;
        proxy.poll(
            |event|
            match event {
                Event::Updated(path, data) => {
                    let device_id = path.split("/")
                        .last()
                        .unwrap()
                        .split("_")
                        .skip(1)
                        .collect::<Vec<&str>>()
                        .join(":")
                        .to_uppercase();
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
                    let device_id = device_id.replace(":", "")
                        .to_ascii_lowercase();
                    statsd_output(
                        "switchbot",
                        &device_id,
                        (device.c().0 * 100f32) as u64,
                        device.humidity as u64,
                        device.battery as u64,
                    ).ok();
                }
            }
        )?;
    }
}

fn ensure_discovering(adapter: &Adapter1Proxy) -> anyhow::Result<()> {
    if !adapter.discovering()? {
        adapter.start_discovery()?;
    }
    Ok(())
}
