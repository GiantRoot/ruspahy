//! Force computation routines.
//!
//! This module implements density, pressure and viscosity calculations for
//! the SPH particles. The current implementation performs an all-pairs
//! search which is not optimal for large particle counts.

use crate::particle::{Particle, ParticleSystem};
use crate::sph_kernel::SPHKernel;

/// Compute density and pressure for every particle.
pub fn compute_density_pressure(psys: &mut ParticleSystem, kernel: &SPHKernel) {
    let particles = &mut psys.particles;
    let mass = 1.0; // assume unit mass

    for i in 0..particles.len() {
        let mut density = 0.0;
        let pi = &particles[i];
        for j in 0..particles.len() {
            let pj = &particles[j];
            let r2 = squared_distance(pi.position, pj.position);
            density += mass * kernel.w_poly6(r2);
        }
        particles[i].density = density;
        particles[i].pressure = 1000.0 * (density - 1000.0); // EOS: p = k(rho - rho0)
    }
}

/// Compute pressure and viscosity forces.
pub fn compute_forces(psys: &mut ParticleSystem, kernel: &SPHKernel) {
    let particles = &mut psys.particles;
    let mass = 1.0;
    let viscosity = 0.1;

    for i in 0..particles.len() {
        let mut force = [0.0; 3];
        let pi = &particles[i];
        for j in 0..particles.len() {
            if i == j { continue; }
            let pj = &particles[j];

            let r_vec = vector_sub(pj.position, pi.position);
            let r2 = dot(r_vec, r_vec);
            let r = r2.sqrt();

            // pressure force
            let grad_w = kernel.grad_w_spiky(r, r_vec);
            let pressure_term = (pi.pressure + pj.pressure) / (2.0 * pj.density);
            for k in 0..3 {
                force[k] -= mass * pressure_term * grad_w[k];
            }

            // viscosity force
            let vel_diff = vector_sub(pj.velocity, pi.velocity);
            let lap_w = kernel.lap_w_viscosity(r);
            for k in 0..3 {
                force[k] += viscosity * mass * vel_diff[k] / pj.density * lap_w;
            }
        }
        particles[i].force = force;
    }
}

/// Utility: squared distance between two 3D points.
fn squared_distance(a: [f64; 3], b: [f64; 3]) -> f64 {
    (a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)
}

/// Subtract two 3D vectors.
fn vector_sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// Dot product of two 3D vectors.
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0]*b[0] + a[1]*b[1] + a[2]*b[2]
}
