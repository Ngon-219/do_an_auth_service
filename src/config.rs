use once_cell::sync::Lazy;
use clap::Parser;

pub static APP_CONFIG: Lazy<Config> = Lazy::new(Config::parse);

#[derive(Debug, Parser)]
pub struct Config {
    #[clap(long, env, default_value_t = 8080)]
    pub port: u16,

    #[clap(long, env, default_value_t = true)]
    pub swagger_enabled: bool,

    #[clap(long, env)]
    pub log_level: String,
}