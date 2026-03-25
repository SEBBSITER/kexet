use std::env;
use common::*;
mod common;

enum Flag {
    Clients(i32),
    Servers(i32),
}

fn setup_configuration() -> Result<Config, ConfigError> {
    // TODO: Replace with better solution
    let config_path = std::env::current_dir()
        .unwrap()
        .join("configuration.toml");

    let mut config = Config::default();
    config.apply_file("configuration.conf")?;
    config.apply_args(env::args().skip(1))?;
    Ok(config)
}

fn main() {
    // Set up configuration
    let config = match setup_configuration() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };
    config.get_status();
}
