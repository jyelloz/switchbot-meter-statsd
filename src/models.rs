use std::{
   fmt,
   convert::TryFrom,
};
use serde::{
    Serialize,
    Deserialize,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Celsius(pub f32);
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Fahrenheit(pub f32);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(
            (c.0 * 9f32 / 5f32) + 32f32
        )
    }
}
impl From<Fahrenheit> for f32 {
    fn from(val: Fahrenheit) -> Self {
        val.0
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

impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Self {
        Celsius(
            (f.0 - 32f32) * 5f32 / 9f32
        )
    }
}
impl From<Celsius> for f32 {
    fn from(val: Celsius) -> Self {
        val.0
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PreferredTemperature {
    C(Celsius),
    F(Fahrenheit),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SwitchbotThermometer {
    pub address: String,
    pub temperature: Celsius,
    pub fahrenheit: bool,
    pub humidity: u8,
    pub battery: u8,
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

    pub fn decode_temperature(bytes: &[u8]) -> Result<Celsius, anyhow::Error> {
        let b3 = bytes[3];
        let b4 = bytes[4];
        let signum = if (b4 >> 7 & 0x1) == 1  {
            1
        } else {
            -1
        };
        let temperature_x10 = signum
            *
            ((b4 & 0b0111_1111) as i32) * 10 + (b3 as i32)
        ;
        Ok(Celsius((temperature_x10 as f32) / 10f32))
    }

    pub fn decode_temperature_unit(bytes: &[u8]) -> bool {
        let b5 = bytes[5];
        b5 >> 7 & 0x1 == 1
    }

    pub fn decode_humidity(bytes: &[u8]) -> u8 {
        let b5 = bytes[5];
        b5 & 0b0111_1111
    }

    pub fn decode_battery(bytes: &[u8]) -> u8 {
        let b2 = bytes[2];
        b2 & 0b0111_1111
    }

}

impl TryFrom<(String, &[u8])> for SwitchbotThermometer {
    type Error = anyhow::Error;
    fn try_from(data: (String, &[u8])) -> anyhow::Result<Self> {
        let (address, bytes) = data;
        let temperature = Self::decode_temperature(bytes)?;
        let fahrenheit = Self::decode_temperature_unit(bytes);
        let humidity = Self::decode_humidity(bytes);
        let battery = Self::decode_battery(bytes);
        Ok(
            SwitchbotThermometer {
                address,
                temperature,
                fahrenheit,
                humidity,
                battery,
            }
        )
    }
}

pub type ReportResult = anyhow::Result<()>;
pub trait Reporter {
    fn report(&self, device: &SwitchbotThermometer) -> ReportResult;
}
