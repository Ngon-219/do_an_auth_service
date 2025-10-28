use clap::Parser;
use once_cell::sync::Lazy;

pub static APP_CONFIG: Lazy<Config> = Lazy::new(Config::parse);

#[derive(Debug, Parser, Clone)]
pub struct Config {
    #[clap(long, env, default_value_t = 8080)]
    pub port: u16,

    #[clap(long, env, default_value_t = true)]
    pub swagger_enabled: bool,

    #[clap(long, env)]
    pub log_level: String,

    #[clap(long, env)]
    pub database_url: String,

    #[clap(long, env)]
    pub blockchain_rpc_url: String,

    #[clap(long, env)]
    pub data_storage_contract_address: String,

    #[clap(long, env)]
    pub admin_private_key: String,
}
