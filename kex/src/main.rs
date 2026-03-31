use std::env;
use common::*;
use client::ClientPool;

mod common;
mod event;
mod client;
mod simulator;
mod network;
mod nemesis;
mod node;

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

    // Dummy testing of client training
    let mut pool = ClientPool::new();
    pool.create_clients(
        config.number_clients,
        "python", // Note windows
        "worker.py",
    ).expect("Failed to add clients");

    pool.init_all().expect("Failed to init all clients");
    pool.train_all(5,5,5.0).expect("Failed to train");
    pool.shutdown_all().expect("Failed to shutdown all clients");

    // TODO: Data loading
    // TODO: Initialize clients
    // Clients are loaded directly after configuration to avoid latency during simulation
    // TODO: Include status bar
}
