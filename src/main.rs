use std::{
    fmt,
    thread,
    time::Duration,
    convert::TryFrom,
};
use zbus::Connection;
use serde::Serialize;
use anyhow::format_err;

mod device1;
mod adapter1;

#[derive(Debug, Clone, Copy, Serialize)]
enum PreferredTemperature {
    C(Celsius),
    F(Fahrenheit),
}
#[derive(Debug, Clone, Copy, Serialize)]
struct Celsius(f32);
#[derive(Debug, Clone, Copy, Serialize)]
struct Fahrenheit(f32);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(
            (c.0 * 9f32 / 5f32) + 32f32
        )
    }
}

impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Self {
        Celsius(
            (f.0 - 32f32) * 5f32 / 9f32
        )
    }
}

impl fmt::Display for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}°C", self.0)
    }
}

impl fmt::Display for Fahrenheit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}°F", self.0)
    }
}

impl fmt::Display for PreferredTemperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::C(value) => value.fmt(f),
            Self::F(value) => value.fmt(f),
        }
    }
}

impl fmt::Display for SwitchbotThermometer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: T={}, H={}%, B={}%",
            self.address,
            self.temperature(),
            self.humidity,
            self.battery,
        )
    }
}

#[derive(Clone, Debug, Serialize)]
struct SwitchbotThermometer {
    address: String,
    temperature: Celsius,
    fahrenheit: bool,
    humidity: u8,
    battery: u8,
}

impl SwitchbotThermometer {

    pub fn temperature(&self) -> PreferredTemperature {
        if self.fahrenheit {
            PreferredTemperature::F(self.f())
        } else {
            PreferredTemperature::C(self.c())
        }
    }

    pub fn c(&self) -> Celsius {
        self.temperature
    }

    pub fn f(&self) -> Fahrenheit {
        self.temperature.into()
    }

    fn decode_temperature(bytes: &[u8]) -> Result<Celsius, anyhow::Error> {
        let b3 = bytes[3];
        let b4 = bytes[4];
        let signum = if (b4 >> 7 & 0x1) == 1  {
            1
        } else {
            -1
        } as i32;
        let temperature_x10 = signum
            *
            ((b4 & 0b0111_1111) as i32) * 10 + (b3 as i32)
        ;
        Ok(Celsius((temperature_x10 as f32) / 10f32))
    }

    fn decode_temperature_unit(bytes: &[u8]) -> bool {
        let b5 = bytes[5];
        b5 >> 7 & 0x1 == 1
    }

    fn decode_humidity(bytes: &[u8]) -> u8 {
        let b5 = bytes[5];
        b5 & 0b0111_1111
    }

    fn decode_battery(bytes: &[u8]) -> u8 {
        let b2 = bytes[2];
        b2 & 0b0111_1111
    }

}

static SWITCHBOT_DATA: &str = "00000d00-0000-1000-8000-00805f9b34fb";
impl <'a> TryFrom<device1::Device1Proxy<'a>> for SwitchbotThermometer {
    type Error = anyhow::Error;
    fn try_from(device: device1::Device1Proxy) -> Result<Self, Self::Error> {
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

fn main() -> anyhow::Result<()> {

    let system = Connection::new_system()?;

    let adapter = adapter1::Adapter1Proxy::new(&system)?;

    loop {
        let device = device1::Device1Proxy::new_for(
            &system,
            "org.bluez",
            "/org/bluez/hci0/dev_F0_73_23_10_C7_3E",
        );

        let device1 = device1::Device1Proxy::new_for(
            &system,
            "org.bluez",
            "/org/bluez/hci0/dev_F0_14_77_A4_77_3B",
        );

        display_thermometer(device.ok());
        display_thermometer(device1.ok());
        if !adapter.discovering().unwrap_or_default() {
            adapter.start_discovery()?;
        }
        thread::sleep(Duration::from_secs(5));
    }
}

fn display_thermometer(device: Option<device1::Device1Proxy>) -> Option<()> {
    let device = device.and_then(|d| SwitchbotThermometer::try_from(d).ok());
    if let Some(thermometer) = device {
        println!("{}", thermometer);
    }
    Some(())
}
