//! Configuration loading utilities.
//!
//! The `SimConfig` structure stores simulation parameters that are loaded
//! from a TOML file. The [`load_config`] function reads the file and
//! deserializes it using `serde`.

use std::fs;
use std::path::Path;
use serde::Deserialize;
use crate::material::{Interface, Material};

/// Parameters that control the SPH simulation.
#[derive(Deserialize)]
pub struct SimConfig {
    pub grid: [usize; 3],
    pub spacing: f64,
    pub time_step: f64,
    pub num_steps: usize,
    pub output_interval: usize,
    #[serde(default)]
    pub materials: Vec<Material>,
    #[serde(default)]
    pub interfaces: Vec<Interface>,
}

/// Load a [`SimConfig`] from a TOML file.
pub fn load_config<P: AsRef<Path>>(path: P) -> SimConfig {
    let content = fs::read_to_string(path).expect("Failed to read config file");
    toml::from_str(&content).expect("Invalid TOML format")
}
