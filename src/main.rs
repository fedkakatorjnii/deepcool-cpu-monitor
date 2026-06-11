use clap::Parser;
use hidapi::HidApi;
use std::thread;
use std::time::Duration;

use crate::args::Args;
use crate::constants::Case;
use crate::data::Data;
use crate::path::expand_sudo_aware_path;
use crate::temperature::get_temperature;
use crate::usage::get_usage;

mod args;
mod config;
mod constants;
mod convert_num;
mod data;
mod path;
mod temperature;
mod usage;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let config_path = expand_sudo_aware_path(&args.config_path)?;
    let cfg = config::load(&config_path)?;

    let api = HidApi::new().unwrap();
    let device = api.open(cfg.vendor_id, cfg.product_id).unwrap();

    let data = Data::new(Case::Load, 0.0);

    device.set_blocking_mode(false)?;
    device.write(&data.convert())?;
    let duration = Duration::from_millis(cfg.time);

    loop {
        // temperature
        let data = Data::new(Case::Temp, get_temperature(&cfg.cpu));

        device.set_blocking_mode(false).unwrap();
        device.write(&data.convert()).unwrap();
        thread::sleep(duration);

        // usage
        let data = Data::new(Case::Usage, get_usage());

        device.set_blocking_mode(false).unwrap();
        device.write(&data.convert()).unwrap();
        thread::sleep(Duration::from_millis(cfg.time));
    }
}
