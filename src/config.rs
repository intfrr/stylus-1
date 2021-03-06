use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use serde::{Deserialize, Serialize};

fn default_server_port() -> u16 {
    80
}

fn default_server_static() -> PathBuf {
    PathBuf::from("static")
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub version: u32,
    pub server: ServerConfig,
    pub monitor: MonitorConfig,
    pub css: CssConfig,
    #[serde(default)]
    pub base_path: PathBuf,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_server_port")]
    pub port: u16,
    #[serde(default = "default_server_static")]
    pub r#static: PathBuf,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MonitorConfig {
    pub dir: PathBuf,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CssConfig {
    pub metadata: CssMetadataConfig,
    pub rules: Vec<CssRule>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CssRule {
    pub selectors: String,
    pub declarations: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CssMetadataConfig {
    #[serde(default)]
    pub blank: Arc<HashMap<String, String>>,
    #[serde(default)]
    pub red: Arc<HashMap<String, String>>,
    #[serde(default)]
    pub yellow: Arc<HashMap<String, String>>,
    #[serde(default)]
    pub green: Arc<HashMap<String, String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MonitorDirConfig {
    pub test: MonitorDirTestConfig,
    #[serde(default)]
    pub base_path: PathBuf,
    #[serde(default)]
    pub id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MonitorDirTestConfig {
    #[serde(with = "humantime_serde")]
    pub interval: Duration,
    #[serde(with = "humantime_serde")]
    pub timeout: Duration,
    pub command: PathBuf,
}

pub fn parse_config(file: String) -> Result<Config, Box<dyn Error>> {
    let mut config: Config = serde_yaml::from_str(&std::fs::read_to_string(&file)?)?;
    if Iterator::count(config.base_path.components()) == 0 {
        config.base_path = Path::parent(Path::new(&file))
            .ok_or("Failed to get base path")?
            .into();
    }

    // Canonical paths
    config.base_path = Path::canonicalize(&config.base_path)?;
    config.server.r#static = Path::canonicalize(&config.base_path.join(&config.server.r#static))?;
    config.monitor.dir = Path::canonicalize(&config.base_path.join(&config.monitor.dir))?;

    // Basic checks before we return the config
    if !config.server.r#static.exists() {
        Err("Static directory does not exist".into())
    } else if !config.monitor.dir.exists() {
        Err("Monitor directory does not exist".into())
    } else {
        Ok(config)
    }
}

pub fn parse_monitor_config(file: &Path) -> Result<MonitorDirConfig, Box<dyn Error>> {
    let mut config: MonitorDirConfig = serde_yaml::from_str(&std::fs::read_to_string(&file)?)?;
    if Iterator::count(config.base_path.components()) == 0 {
        config.base_path = Path::parent(Path::new(&file))
            .ok_or("Failed to get base path")?
            .into();
    }

    // Canonical paths
    config.base_path = Path::canonicalize(&config.base_path)?;
    config.test.command = Path::canonicalize(&config.base_path.join(&config.test.command))?;

    if config.id.is_empty() {
        config.id = file
            .parent()
            .ok_or("Invalid parent")?
            .file_name()
            .ok_or("Invalid file name")?
            .to_string_lossy()
            .to_string();
    }

    // Basic checks before we return the config
    if !config.test.command.exists() {
        Err("Test command does not exist".into())
    } else {
        Ok(config)
    }
}
