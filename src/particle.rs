pub struct Particle {
    pub position: [f64; 3],
    pub velocity: [f64; 3],
    pub force: [f64; 3],
    pub density: f64,
    pub pressure: f64,
}

pub struct ParticleSystem {
    pub particles: Vec<Particle>,
}

impl ParticleSystem {
    pub fn new(config: &crate::config::SimConfig) -> Self {
        let mut particles = Vec::new();
        for z in 0..config.grid[2] {
            for y in 0..config.grid[1] {
                for x in 0..config.grid[0] {
                    let pos = [
                        x as f64 * config.spacing,
                        y as f64 * config.spacing,
                        z as f64 * config.spacing,
                    ];
                    particles.push(Particle {
                        position: pos,
                        velocity: [0.0; 3],
                        force: [0.0; 3],
                        density: 1000.0,
                        pressure: 0.0,
                    });
                }
            }
        }
        Self { particles }
    }

    pub fn build_neighbor_list(&mut self) {
        // TODO: implement neighbor search
    }

    pub fn compute_forces(&mut self) {
        // TODO: implement pressure and viscosity forces
    }
}