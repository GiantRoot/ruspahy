//! 配置加载工具。
//!
//! `SimConfig` 结构保存从 TOML 文件读取的模拟参数，
//! [`load_config`] 函数负责读取并通过 `serde` 反序列化。

use std::fs;
use std::path::Path;
use serde::Deserialize;
use crate::material::{Interface, Material};

/// 控制 SPH 模拟的参数集合。
#[derive(Deserialize)]
#[allow(dead_code)]
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

/// 从 TOML 文件加载 [`SimConfig`]。
pub fn load_config<P: AsRef<Path>>(path: P) -> SimConfig {
    let content = fs::read_to_string(path).expect("Failed to read config file");
    toml::from_str(&content).expect("Invalid TOML format")
}
