//! 配置加载工具。
//!
//! `SimConfig` 结构保存从 TOML 文件读取的模拟参数，
//! [`load_config`] 函数负责读取并通过 `serde` 反序列化。

use std::fs;
use std::path::Path;
use serde::Deserialize;
use crate::material::{Interface, Material};

/// 圆柱形桶及其移动盖子的设置。
#[derive(Deserialize, Clone)]
#[allow(dead_code)]
pub struct BucketConfig {
    pub radius: f64,
    pub height: f64,
    /// 盖子向下移动的速度（每个时间单位下降量）
    pub lid_speed: f64,
    /// 盖子可移动到的最小高度
    pub min_height: f64,
}

/// 初始球体设置，可用于模拟碰撞等场景。
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct SphereConfig {
    pub center: [f64; 3],
    pub radius: f64,
    pub velocity: [f64; 3],
    #[serde(default)]
    pub material_id: usize,
}

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
    #[serde(default)]
    pub spheres: Vec<SphereConfig>,
    #[serde(default)]
    pub bucket: Option<BucketConfig>,
}

/// 从 TOML 文件加载 [`SimConfig`]。
pub fn load_config<P: AsRef<Path>>(path: P) -> SimConfig {
    let content = fs::read_to_string(path).expect("Failed to read config file");
    toml::from_str(&content).expect("Invalid TOML format")
}
