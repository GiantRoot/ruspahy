pub mod vector;

pub use vector::Vec2;

#[derive(Clone)]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub force: Vec2,
    pub mass: f64,
    pub density: f64,
    pub pressure: f64,
}

pub struct SPH {
    pub particles: Vec<Particle>,
    pub smoothing_length: f64,
    pub rest_density: f64,
    pub gas_constant: f64,
    pub viscosity: f64,
    pub time_step: f64,
}

impl SPH {
    pub fn new(
        particles: Vec<Particle>,
        smoothing_length: f64,
        rest_density: f64,
        gas_constant: f64,
        viscosity: f64,
        time_step: f64,
    ) -> Self {
        Self {
            particles,
            smoothing_length,
            rest_density,
            gas_constant,
            viscosity,
            time_step,
        }
    }

    pub fn step(&mut self) {
        self.compute_density_pressure();
        self.compute_forces();
        self.integrate();
    }

    fn compute_density_pressure(&mut self) {
        let h = self.smoothing_length;
        let poly6 = 315.0 / (64.0 * std::f64::consts::PI * h.powi(9));
        let h2 = h * h;
        let gas_constant = self.gas_constant;
        let rest_density = self.rest_density;
        for i in 0..self.particles.len() {
            let mut density = 0.0;
            for j in 0..self.particles.len() {
                let r = self.particles[i].pos - self.particles[j].pos;
                let r2 = r.norm2();
                if r2 < h2 {
                    density += self.particles[j].mass * poly6 * (h2 - r2).powi(3);
                }
            }
            self.particles[i].density = density;
            self.particles[i].pressure = gas_constant * (density - rest_density);
        }
    }

    fn compute_forces(&mut self) {
        let h = self.smoothing_length;
        let spiky = -45.0 / (std::f64::consts::PI * h.powi(6));
        let viscosity_laplacian = 45.0 / (std::f64::consts::PI * h.powi(6));
        let viscosity = self.viscosity;

        for i in 0..self.particles.len() {
            let mut force = Vec2::zero();
            for j in 0..self.particles.len() {
                if i == j {
                    continue;
                }
                let rij = self.particles[i].pos - self.particles[j].pos;
                let r = rij.norm();
                if r < h {
                    // pressure force
                    let grad = rij.normalized() * spiky * (h - r).powi(2);
                    force -= grad
                        * self.particles[j].mass
                        * (self.particles[i].pressure + self.particles[j].pressure)
                        / (2.0 * self.particles[j].density);

                    // viscosity force
                    let lap = viscosity_laplacian * (h - r);
                    force += (self.particles[j].vel - self.particles[i].vel)
                        * viscosity
                        * self.particles[j].mass
                        * lap
                        / self.particles[j].density;
                }
            }
            // gravity
            force += Vec2 { x: 0.0, y: -9.81 } * self.particles[i].mass;
            self.particles[i].force = force;
        }
    }

    fn integrate(&mut self) {
        let dt = self.time_step;
        for p in &mut self.particles {
            p.vel += p.force * (dt / p.density);
            p.pos += p.vel * dt;
        }
    }
}
