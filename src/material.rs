//! 材料定义及界面类型。
//!
//! 项目支持多种固体材料，每个粒子带有 `material_id` 指向其材质。
//! 接口用于描述不同材料之间的相互作用。

use serde::Deserialize;

/// 固体的不同本构类型。
#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaterialType {
    Elastic,
    Elastoplastic,
    /// 同时考虑损伤的弹塑性材料
    ElastoplasticDamage,
    Brittle,
}

/// 简单的材料属性。
#[derive(Clone, Deserialize)]
pub struct Material {
    pub id: usize,
    pub name: String,
    pub material_type: MaterialType,
    pub density: f64,
    pub youngs_modulus: f64,
    /// 屈服应力，仅当材料类型为弹塑性或弹塑性损伤时有效
    #[serde(default)]
    pub yield_strength: Option<f64>,
    /// 强化模量，用于简单的线性硬化模型
    #[serde(default)]
    pub hardening_modulus: Option<f64>,
    /// 损伤阈值，当累积塑性应变超过该值时开始退化
    #[serde(default)]
    pub damage_threshold: Option<f64>,
}

/// 两种材料之间的连接方式。
#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InterfaceType {
    Strong,
    Weak,
    Variable,
}

/// 材料界面配置。
#[derive(Clone, Deserialize)]
pub struct Interface {
    pub mat_a: usize,
    pub mat_b: usize,
    pub interface_type: InterfaceType,
    /// 界面结合强度，可用数值描述弱到强的连续变化
    #[serde(default = "default_bond_strength")]
    pub bond_strength: f64,
}

/// 默认的界面结合强度。
fn default_bond_strength() -> f64 {
    1.0
}
