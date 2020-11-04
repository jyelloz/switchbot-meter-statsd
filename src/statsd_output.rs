use std::net::UdpSocket;
use cadence::{
    prelude::*,
    StatsdClient,
    UdpMetricSink,
    DEFAULT_PORT,
};

pub fn statsd_output(
    prefix: &str,
    device_id: &str,
    temperature: u64,
    humidity: u64,
    battery: u64,
) -> anyhow::Result<()> {

    let host = ("127.0.0.1", DEFAULT_PORT);
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let sink = UdpMetricSink::from(host, socket)?;
    let metrics = StatsdClient::from_sink(prefix, sink);

    output_temperature(&metrics, device_id, temperature)?;
    output_humidity(&metrics, device_id, humidity)?;
    output_battery(&metrics, device_id, battery)?;

    Ok(())

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
