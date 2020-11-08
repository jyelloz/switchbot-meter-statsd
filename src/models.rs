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
impl Into<f32> for Fahrenheit {
    fn into(self) -> f32 {
        self.0
    }
}

impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Self {
        Celsius(
            (f.0 - 32f32) * 5f32 / 9f32
        )
    }
}
impl Into<f32> for Celsius {
    fn into(self) -> f32 {
        self.0
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
        } as i32;
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
