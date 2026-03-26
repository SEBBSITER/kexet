use std::fs;
use std::fmt;

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Parse { line_no: usize, reason: String },
    InvalidValue { key: String, reason: String },
    UnknownKey(String),
    MissingArgValue(String),
    UnexpectedArg(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "Failed to read config file: {e}"),
            Self::Parse { line_no, reason } => write!(f, "Failed to parse line: {line_no}, reason: {reason}"),
            Self::InvalidValue { key, reason } => write!(f, "Invalid value for key: {key}, reason: {reason}"),
            Self::UnknownKey(key) => write!(f, "Unknown key: {key}"),
            Self::MissingArgValue(key) => write!(f, "Missing value for argument: {key}"),
            Self::UnexpectedArg(key) => write!(f, "Unexpected argument: {key}"),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<std::io::Error> for ConfigError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

pub struct Config {
    pub(crate) number_clients: usize,
    number_servers: usize,
    topology: Topology,
}

pub enum Topology {
    Straight,
}

impl Config {
    /// Custom constructor
    pub fn new(number_clients: usize, number_servers: usize, topology: Topology) -> Self {
        Self { number_clients, number_servers, topology }
    }

    /// Default configuration
    pub fn default() -> Self {
        Self {
            number_clients: 10,
            number_servers: 4,
            topology: Topology::Straight,
        }
    }

    pub fn get_status(self) {
        println!("Number of clients: {} \n Number of servers: {}", self.number_clients, self.number_servers);
    }

    fn set(&mut self, key: &str, value: &str) -> Result<(), ConfigError> {
        match key {
            "number_clients" => {
                self.number_clients = value.parse::<usize>()
                    .map_err(|e| ConfigError::InvalidValue {
                        key: key.into(),
                        reason: e.to_string()
                    })?;
            }
            "number_servers" => {
                self.number_servers = value.parse::<usize>()
                    .map_err(|e| ConfigError::InvalidValue {
                        key: key.into(),
                        reason: e.to_string(),
                    })?;
            }
            "topology" => {
                self.topology = match value.to_lowercase().as_str() {
                    "straight" => Topology::Straight,
                    _ => return Err(ConfigError::UnexpectedArg(value.to_string())),
                }
            }
            _ => return Err(ConfigError::UnknownKey(key.into())),
        }
        Ok(())
    }

    pub fn apply_file(&mut self, path: &str) -> Result<(), ConfigError> {
        let content = fs::read_to_string(path)?;

        for (line_no, line) in content.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let (key, value) = line
                .split_once('=')
                .ok_or_else(|| ConfigError::Parse { line_no, reason: line.to_string() })?;
            self.set(key.trim(), value.trim())?;
        }

        Ok(())
    }

    pub fn apply_args<I>(&mut self, mut args: I) -> Result<(), ConfigError>
    where I: Iterator<Item = String> {
        while let Some(arg) = args.next() {
            if let Some(key) = arg.strip_prefix("--") {

                // TODO: Add support for boolean flags, like --nobuild which doesn't have a value

                let value = args
                    .next()
                    .ok_or_else(|| ConfigError::MissingArgValue(key.into()))?;
                self.set(key, &value)?;
            } else {
                return Err(ConfigError::UnexpectedArg(arg));
            }
        }

        Ok(())
    }
}