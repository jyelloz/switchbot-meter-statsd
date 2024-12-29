use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, thread, time::Duration};
use zbus::{
    blocking::{fdo, Connection, MessageIterator},
    fdo::PropertiesChanged,
    message,
    zvariant::{Dict, ObjectPath, Type, Value},
    MatchRule, Message,
};

#[allow(non_snake_case)]
mod bluez;
mod models;
mod statsd_output;

use bluez::adapter1;
use models::{Reporter as _, SwitchbotThermometer};
use statsd_output::StatsdReporter;

static DEVICE_1: &str = "org.bluez.Device1";
static PROPERTIES: &str = "org.freedesktop.DBus.Properties";
static PROPERTIES_CHANGED: &str = "PropertiesChanged";
static SERVICE_DATA: &str = "ServiceData";
static THERMOMETER_UUID: &str = "00000d00-0000-1000-8000-00805f9b34fb";

#[derive(Clone, Debug, Deserialize, Serialize, Type)]
struct ThermometerData {
    data: Vec<u8>,
}

impl From<Vec<u8>> for ThermometerData {
    fn from(data: Vec<u8>) -> Self {
        Self { data }
    }
}

impl TryFrom<&Value<'_>> for ThermometerData {
    type Error = zbus::zvariant::Error;

    fn try_from(value: &Value<'_>) -> Result<Self, Self::Error> {
        let bytes: Vec<u8> = value.to_owned().try_into()?;
        Ok(Self::from(bytes))
    }
}

#[derive(Clone, Debug)]
enum Event {
    Updated(String, ThermometerData),
}

impl Event {
    fn from_message(message: Message) -> Option<Self> {
        let path = message.header().path().map(ObjectPath::to_string)?;
        let message = PropertiesChanged::from_message(message)?;
        let args = message
            .args()
            .ok()
            .filter(|args| args.interface_name() == DEVICE_1)?;
        let changed_properties = args.changed_properties();
        let service_data = changed_properties
            .get(SERVICE_DATA)
            .and_then(|data| Dict::try_from(data).ok())?;
        let thermometer_data = service_data
            .get::<&str, ThermometerData>(&THERMOMETER_UUID)
            .ok()
            .flatten()?;
        Some(Self::Updated(path, thermometer_data))
    }
}

struct PropertiesChangedIterator {
    messages: zbus::blocking::MessageIterator,
}

impl PropertiesChangedIterator {
    pub fn system() -> anyhow::Result<Self> {
        let system = Connection::system()?;
        {
            let dbus_proxy = fdo::DBusProxy::new(&system)?;
            let rule = MatchRule::builder()
                .msg_type(message::Type::Signal)
                .interface(PROPERTIES)?
                .member(PROPERTIES_CHANGED)?
                .path_namespace("/org/bluez")?
                .build();
            dbus_proxy.add_match_rule(rule)?;
        }
        let messages = MessageIterator::from(system);
        Ok(Self { messages })
    }
}

impl std::iter::Iterator for PropertiesChangedIterator {
    type Item = Message;

    fn next(&mut self) -> Option<Self::Item> {
        let Self { messages } = self;
        loop {
            let msg = messages.next()?;
            if let Ok(msg) = msg {
                return Some(msg);
            }
        }
    }
}

fn mac_address_from_dbus_path(path: &str) -> String {
    path.split('/')
        .last()
        .unwrap()
        .split('_')
        .skip(1)
        .collect::<Vec<&str>>()
        .join(":")
        .to_uppercase()
}

fn main() -> anyhow::Result<()> {
    spawn_ensure_discovering()?;
    let statsd = StatsdReporter::try_default()?;
    let events = PropertiesChangedIterator::system()?.filter_map(Event::from_message);
    for Event::Updated(path, data) in events {
        let device_id = mac_address_from_dbus_path(&path);
        let device = SwitchbotThermometer::try_from((device_id.clone(), data.data.as_slice()))?;
        println!(
            "{} {} {} {}",
            &device_id,
            f32::from(device.f()),
            device.humidity,
            device.battery,
        );
        statsd.report(&device).ok();
    }
    Ok(())
}

fn spawn_ensure_discovering() -> anyhow::Result<()> {
    let connection = Connection::system()?;
    let adapter = adapter1::Adapter1ProxyBlocking::builder(&connection)
        .destination("org.bluez")?
        .path("/org/bluez/hci0")?
        .build()?;
    thread::spawn(move || ensure_discovering_task(adapter));
    Ok(())
}

fn ensure_discovering_task(adapter: adapter1::Adapter1ProxyBlocking) {
    loop {
        if let Err(e) = ensure_discovering(&adapter) {
            eprintln!("failed to ensure adapter is discovering: {:?}", e);
        }
        thread::sleep(Duration::from_secs(30));
    }
}

fn ensure_discovering(adapter: &adapter1::Adapter1ProxyBlocking) -> anyhow::Result<()> {
    if !adapter.powered()? {
        adapter.set_powered(true)?;
    }
    if !adapter.discovering()? {
        adapter.start_discovery()?;
    }
    Ok(())
}
