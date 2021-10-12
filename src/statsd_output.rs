use std::net::UdpSocket;
use cadence::{
    prelude::*,
    StatsdClient,
    UdpMetricSink,
    DEFAULT_PORT,
};
use crate::models::{
    Reporter,
    ReportResult,
    SwitchbotThermometer,
};

pub type InitResult = anyhow::Result<StatsdReporter>;

pub struct StatsdReporter {
    metrics: StatsdClient,
}

impl StatsdReporter {
    pub fn try_default() -> InitResult {
        Self::try_default_prefixed("switchbot")
    }
    pub fn try_default_prefixed(prefix: &str) -> InitResult {
        let host = ("127.0.0.1", DEFAULT_PORT);
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        let sink = UdpMetricSink::from(host, socket)?;
        let metrics = StatsdClient::from_sink(prefix, sink);
        Ok(Self { metrics })
    }
}

impl Reporter for StatsdReporter {
    fn report(&self, device: &SwitchbotThermometer) -> ReportResult {
        let Self { metrics } = self;

        let SwitchbotThermometer {
            address,
            humidity,
            battery,
            ..
        } = device;

        let device_id = address.replace(":", "")
            .to_ascii_lowercase();

        let temperature = (device.c().0 * 100f32) as u64;
        let humidity = *humidity as u64;
        let battery = *battery as u64;

        output_temperature(metrics, &device_id, temperature)?;
        output_humidity(metrics, &device_id, humidity)?;
        output_battery(metrics, &device_id, battery)?;

        Ok(())
    }
}

fn output_temperature(
    metrics: &StatsdClient,
    device_id: &str,
    value: u64,
) -> anyhow::Result<()> {
    let key = format!("temperature.{}", device_id);
    metrics.gauge(&key, value)?;
    Ok(())
}

fn output_humidity(
    metrics: &StatsdClient,
    device_id: &str,
    value: u64,
) -> anyhow::Result<()> {
    let key = format!("humidity.{}", device_id);
    metrics.gauge(&key, value)?;
    Ok(())
}

fn output_battery(
    metrics: &StatsdClient,
    device_id: &str,
    value: u64,
) -> anyhow::Result<()> {
    let key = format!("battery.{}", device_id);
    metrics.gauge(&key, value)?;
    Ok(())
}
