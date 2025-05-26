//! Time integration of particle motion.
//!
//! A simple explicit Euler integrator is provided which updates velocities
//! and positions using the forces computed for each particle.

use crate::particle::ParticleSystem;

/// Advance all particles by one time step using explicit Euler.
pub fn integrate(psys: &mut ParticleSystem, dt: f64) {
    for p in &mut psys.particles {
        for i in 0..3 {
            p.velocity[i] += dt * p.force[i] / p.density;
            p.position[i] += dt * p.velocity[i];
        }
    }
}
