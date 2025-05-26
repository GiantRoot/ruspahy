use ruspahy::{Particle, SPH, Vec2};

fn main() {
    let mut particles = Vec::new();
    particles.push(Particle {
        pos: Vec2::new(0.0, 0.5),
        vel: Vec2::zero(),
        force: Vec2::zero(),
        mass: 1.0,
        density: 0.0,
        pressure: 0.0,
    });

    particles.push(Particle {
        pos: Vec2::new(0.1, 0.5),
        vel: Vec2::zero(),
        force: Vec2::zero(),
        mass: 1.0,
        density: 0.0,
        pressure: 0.0,
    });

    let mut sph = SPH::new(particles, 0.1, 1000.0, 2000.0, 0.1, 0.001);

    for step in 0..10 {
        sph.step();
        println!("step {}: pos0=({:.3}, {:.3}), pos1=({:.3}, {:.3})",
            step,
            sph.particles[0].pos.x, sph.particles[0].pos.y,
            sph.particles[1].pos.x, sph.particles[1].pos.y);
    }
}
