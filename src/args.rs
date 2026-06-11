use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to config file
    #[arg(short, long, default_value = "~/.config/deepcool/config.toml")]
    pub config_path: String,
}
