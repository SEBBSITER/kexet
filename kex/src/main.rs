use std::env;
use common::*;
use client::pool::ClientPool;
use std::path::{Path, PathBuf};
use std::process::Command;

mod common;
mod client;

// Windows / Mac support
#[cfg(target_os="windows")]
const PYTHON_COMMAND: &str = "python";

#[cfg(target_os="macos")]
const PYTHON_COMMAND: &str = "python3";

#[cfg(target_os="windows")]
fn venv_python_path(venv_dir: &Path) -> PathBuf {
    venv_dir.join("Scripts").join("python.exe")
}

#[cfg(target_os="macos")]
fn venv_python_path(venv_dir: &Path) -> PathBuf {
    venv_dir.join("bin").join("python")
}

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

// Check for existing python virtual environment, create one if missing
fn ensure_venv() -> PathBuf {
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let venv_dir = project_root.join("venv");
    let python = venv_python_path(&venv_dir);

    if !python.exists() {
        println!("Virtual environment doesn't exist. Creating it now...");
        let status = Command::new(PYTHON_COMMAND)
            .args(["-m", "venv"])
            .arg(&venv_dir)
            .status()
            .expect("Failed to create venv");

        assert!(status.success(), "venv creation failed");

        println!("Installing dependencies...");
        let requirements = project_root.join("requirements.txt");

        let status = Command::new(&python)
            .args(["-m", "pip", "install", "-r"])
            .arg(&requirements)
            .status()
            .expect("Failed to run pip install");

        assert!(status.success(), "pip install fail");
    }

    python
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

    // Set up virtual environment
    let python = ensure_venv();

    // Dummy testing of client training
    let mut pool = ClientPool::new();
    pool.create_clients(
        config.number_clients,
        &python,
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
