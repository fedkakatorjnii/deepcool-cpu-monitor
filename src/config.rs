use anyhow::Result;
use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub cpu: String,
    pub vendor_id: u16,
    pub product_id: u16,
    pub time: u64,
}

pub fn load(config_path: &str) -> Result<Settings> {
    let cfg = Config::builder()
        .add_source(File::with_name(config_path).required(true))
        .build()?;

    Ok(cfg.try_deserialize()?)
}
