use crate::particle::ParticleSystem;

pub fn integrate(psys: &mut ParticleSystem, dt: f64) {
    for p in &mut psys.particles {
        for i in 0..3 {
            p.velocity[i] += dt * p.force[i] / p.density;
            p.position[i] += dt * p.velocity[i];
        }
    }
}