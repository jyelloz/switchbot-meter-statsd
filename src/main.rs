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
    MatchRule,
    Message,
    MessageHeader,
    MessageType,
    blocking::{
        MessageIterator,
        Connection,
        fdo,
    },
    zvariant::{
        Dict,
        OwnedValue,
        Type,
        ObjectPath,
    },
};
use serde::{
    Serialize,
    Deserialize,
};
use thiserror::Error;

#[allow(non_snake_case)]
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

#[derive(Error, Debug, Clone)]
enum EventConvertError {
    #[error("failed to parse DBus message: {0:?}")]
    MessageParsing(&'static str),
    #[error("property changed was on wrong interface {0:?}")]
    InterfaceMismatch(String),
    #[error("thermometer data missing")]
    ThermometerDataMissing,
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

impl TryFrom<&Message> for Event {
    type Error = EventConvertError;
    fn try_from(msg: &Message) -> Result<Self, Self::Error> {
        let path = msg.path()
            .ok_or(EventConvertError::MessageParsing("path"))?;
        let body: PropertiesChanged = msg.body()
            .map_err(|_| EventConvertError::MessageParsing("body"))?;
        (path, body).try_into()
    }
}

impl TryFrom<(ObjectPath<'_>, PropertiesChanged)> for Event {
    type Error = EventConvertError;
    fn try_from(item: (ObjectPath<'_>, PropertiesChanged)) -> Result<Self, Self::Error> {
        let (path, mut body) = item;
        let path = path.to_string();
        if body.interface != DEVICE_1 {
            Err(EventConvertError::InterfaceMismatch(body.interface.clone()))?;
        }
        let dict: Dict = body.changed_properties.remove(SERVICE_DATA)
            .ok_or(EventConvertError::MessageParsing(SERVICE_DATA))?
            .try_into()
            .map_err(|_| EventConvertError::MessageParsing(SERVICE_DATA))?;
        let mut dict: HashMap<String, Vec<u8>> = dict.try_into()
            .map_err(|_| EventConvertError::MessageParsing("HashMap"))?;
        dict.remove(THERMOMETER_UUID)
            .map(ThermometerData::from)
            .map(|data| Event::Updated(path, data))
            .ok_or(EventConvertError::ThermometerDataMissing)
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
                .msg_type(MessageType::Signal)
                .interface("org.freedesktop.DBus.Properties")?
                .member("PropertiesChanged")?
                .path_namespace("/org/bluez")?
                .build();
            dbus_proxy.add_match_rule(rule)?;
        }
        let messages = MessageIterator::from(system);
        Ok(Self { messages })
    }
}

static DEVICE_1: &str = "org.bluez.Device1";
static PROPERTIES: &str = "org.freedesktop.DBus.Properties";
static PROPERTIES_CHANGED: &str = "PropertiesChanged";
static SERVICE_DATA: &str = "ServiceData";
static THERMOMETER_UUID: &str = "00000d00-0000-1000-8000-00805f9b34fb";

impl std::iter::Iterator for PropertiesChangedIterator {
    type Item = std::sync::Arc<Message>;

    fn next(&mut self) -> Option<Self::Item> {
        let Self { messages } = self;
        loop {
            let msg = messages.next()?;
            let msg = msg.ok().and_then(Self::filter);
            if msg.is_some() {
                return msg;
            }
        }
    }

}

impl PropertiesChangedIterator {
    fn filter<M: std::ops::Deref<Target=Message>>(msg: M) -> Option<M> {
        msg.header()
            .ok()
            .and_then(Self::is_signal)
            .and_then(Self::is_dbus_properties)
            .and_then(Self::is_dbus_properties_changed)?;
        Some(msg)
    }

    fn is_signal(header: MessageHeader<'_>) -> Option<MessageHeader<'_>> {
        header.message_type()
            .ok()
            .filter(|t| *t == MessageType::Signal)?;
        Some(header)
    }

    fn is_dbus_properties(header: MessageHeader<'_>) -> Option<MessageHeader<'_>> {
        header.interface()
            .ok()
            .flatten()
            .filter(|i| *i == PROPERTIES)?;
        Some(header)
    }

    fn is_dbus_properties_changed(header: MessageHeader<'_>) -> Option<MessageHeader<'_>> {
        header.member()
            .ok()
            .flatten()
            .filter(|m| *m == PROPERTIES_CHANGED)?;
        Some(header)
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
    let events = PropertiesChangedIterator::system()?
        .map(|m| Event::try_from(&*m))
        .filter(|e| e.is_ok());
    for Event::Updated(path, data) in events.flatten() {
        let device_id = mac_address_from_dbus_path(&path);
        let device = SwitchbotThermometer::try_from(
            (device_id.clone(), data.data.as_slice())
        )?;
        println!(
            "{} {} {} {}",
            &device_id,
            device.c().0,
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
