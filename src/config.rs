use std::fs;
use std::path::Path;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SimConfig {
    pub grid: [usize; 3],
    pub spacing: f64,
    pub time_step: f64,
    pub num_steps: usize,
    pub output_interval: usize,
}

pub fn load_config<P: AsRef<Path>>(path: P) -> SimConfig {
    let content = fs::read_to_string(path).expect("Failed to read config file");
    toml::from_str(&content).expect("Invalid TOML format")
}