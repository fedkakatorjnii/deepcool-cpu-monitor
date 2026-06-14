use clap::Parser;
use hidapi::HidApi;
use std::thread;
use std::time::Duration;

use crate::args::Args;
use crate::constants::Case;
use crate::cpu::show_components;
use crate::data::Data;
use crate::log::init_log;
use crate::temperature::get_temperature;
use crate::usage::get_usage;

mod args;
mod config;
mod constants;
mod convert_num;
mod cpu;
mod data;
mod log;
mod temperature;
mod usage;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let config_path = shellexpand::tilde(&args.config_path).to_string();
    let cfg = config::load(&config_path)?;

    init_log(&cfg.log_level);

    if args.show_components {
        show_components();
        return Ok(());
    }

    tracing::info!("Starting deepcool-cpu-monitor...");
    tracing::info!("Configuration path: {config_path}");

    let api = HidApi::new()?;
    let device = api.open(cfg.vendor_id, cfg.product_id)?;

    let data = Data::new(Case::Load, 0.0);

    device.set_blocking_mode(false)?;
    device.write(&data.convert())?;
    let duration = Duration::from_millis(cfg.time);

    loop {
        // temperature
        let data = Data::new(Case::Temp, get_temperature(&cfg.cpu));
        tracing::info!("temperature: {}", data.data);

        device.set_blocking_mode(false)?;
        device.write(&data.convert())?;
        thread::sleep(duration);

        // usage
        let data = Data::new(Case::Usage, get_usage());
        tracing::info!("usage: {}", data.data);

        device.set_blocking_mode(false)?;
        device.write(&data.convert())?;
        thread::sleep(Duration::from_millis(cfg.time));
    }
}
