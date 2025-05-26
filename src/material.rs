//! Material definitions and interface types.
//!
//! The project supports multiple solid materials. Each particle carries a
//! `material_id` referencing one of these materials. Interfaces describe how
//! different materials interact with each other.

use serde::Deserialize;

/// Different constitutive behaviors for solids.
#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaterialType {
    Elastic,
    Elastoplastic,
    Brittle,
}

/// Simple material properties.
#[derive(Clone, Deserialize)]
pub struct Material {
    pub id: usize,
    pub name: String,
    pub material_type: MaterialType,
    pub density: f64,
    pub youngs_modulus: f64,
}

/// Connection behavior between two materials.
#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InterfaceType {
    Strong,
    Weak,
    Variable,
}

/// Interface configuration.
#[derive(Clone, Deserialize)]
pub struct Interface {
    pub mat_a: usize,
    pub mat_b: usize,
    pub interface_type: InterfaceType,
}
