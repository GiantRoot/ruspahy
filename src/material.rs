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
}
