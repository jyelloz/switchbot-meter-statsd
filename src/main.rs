use std::{
    thread,
    time::Duration,
    convert::TryFrom,
};
use zbus::Connection;
use anyhow::format_err;

mod device1;
mod adapter1;
mod statsd_output;
mod models;

use models::SwitchbotThermometer;

use device1::Device1Proxy;

static SWITCHBOT_DATA: &str = "00000d00-0000-1000-8000-00805f9b34fb";
impl <'a> TryFrom<Device1Proxy<'a>> for SwitchbotThermometer {
    type Error = anyhow::Error;
    fn try_from(device: Device1Proxy) -> Result<Self, Self::Error> {
        let service_data = device.service_data()?;
        let address = device.address()?;
        let bytes = service_data.get(SWITCHBOT_DATA)
            .ok_or(format_err!("no data at key {:?}", SWITCHBOT_DATA))?;
        let temperature = Self::decode_temperature(bytes)?;
        let humidity = Self::decode_humidity(bytes);
        let fahrenheit = Self::decode_temperature_unit(bytes);
        let battery = Self::decode_battery(bytes);
        let thermometer = SwitchbotThermometer {
            address,
            temperature,
            fahrenheit,
            humidity,
            battery,
        };
        Ok(thermometer)
    }
}

fn get_update_frequency() -> Duration {

    let seconds = std::env::args()
        .skip(1)
        .next()
        .or(std::env::var("NETDATA_UPDATE_EVERY").ok())
        .and_then(|value| u64::from_str_radix(&value, 10).ok())
        .unwrap_or(1);

    Duration::from_secs(seconds)
}

fn main() -> anyhow::Result<()> {

    let update_frequency = get_update_frequency();

    let system = Connection::new_system()?;

    let adapter = adapter1::Adapter1Proxy::new(&system)?;

    loop {
        let device = Device1Proxy::new_for(
            &system,
            "org.bluez",
            "/org/bluez/hci0/dev_F0_73_23_10_C7_3E",
        );

        let device1 = Device1Proxy::new_for(
            &system,
            "org.bluez",
            "/org/bluez/hci0/dev_F0_14_77_A4_77_3B",
        );

        if let Some(err) = display_thermometer(device).err() {
            eprintln!("error: {:?}", err);
        }
        if let Some(err) = display_thermometer(device1).err() {
            eprintln!("error: {:?}", err);
        }
        if !adapter.discovering().unwrap_or_default() {
            adapter.start_discovery()?;
        }
        thread::sleep(update_frequency);
    }
}

fn display_thermometer(device: Result<Device1Proxy, zbus::Error>) -> anyhow::Result<()> {

    let device = SwitchbotThermometer::try_from(device?)?;
    let device_id = device.address
        .replace(":", "")
        .to_ascii_lowercase();
    statsd_output::statsd_output(
        "switchbot",
        &device_id,
        (device.c().0 * 100f32) as u64,
        device.humidity as u64,
        device.battery as u64,
    )?;
    Ok(())
}
