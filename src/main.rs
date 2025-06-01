use nalgebra::{Vector3};

/// Simple SPH particle structure.
struct Particle {
    position: Vector3<f64>,
    velocity: Vector3<f64>,
    density: f64,
    pressure: f64,
}

const SMOOTHING_LENGTH: f64 = 2.0;      // kernel radius
const REST_DENSITY: f64 = 1.0;          // reference density
const PARTICLE_MASS: f64 = 1.0;         // uniform mass
const STIFFNESS: f64 = 1000.0;          // equation of state coefficient

/// Cubic spline kernel in 3D
fn kernel(r: f64) -> f64 {
    let q = r / SMOOTHING_LENGTH;
    let sigma = 1.0 / (std::f64::consts::PI * SMOOTHING_LENGTH.powi(3));
    if q <= 1.0 {
        sigma * (1.0 - 1.5*q.powi(2) + 0.75*q.powi(3))
    } else if q <= 2.0 {
        sigma * 0.25 * (2.0 - q).powi(3)
    } else {
        0.0
    }
}

/// Gradient of the cubic spline kernel
fn grad_kernel(r_vec: Vector3<f64>, r: f64) -> Vector3<f64> {
    let q = r / SMOOTHING_LENGTH;
    let sigma = 1.0 / (std::f64::consts::PI * SMOOTHING_LENGTH.powi(4));
    if r == 0.0 || q > 2.0 {
        return Vector3::zeros();
    }
    if q <= 1.0 {
        sigma * (-3.0*q + 2.25*q.powi(2)) * r_vec / r
    } else {
        sigma * (-0.75*(2.0 - q).powi(2)) * r_vec / r
    }
}

/// Initialize particles inside a sphere
fn init_sphere(center: Vector3<f64>, radius: f64, spacing: f64) -> Vec<Particle> {
    let mut particles = Vec::new();
    let mut z = -radius;
    while z <= radius {
        let mut y = -radius;
        while y <= radius {
            let mut x = -radius;
            while x <= radius {
                let pos = Vector3::new(x, y, z) + center;
                if (pos - center).norm() <= radius {
                    particles.push(Particle {
                        position: pos,
                        velocity: Vector3::zeros(),
                        density: REST_DENSITY,
                        pressure: 0.0,
                    });
                }
                x += spacing;
            }
            y += spacing;
        }
        z += spacing;
    }
    particles
}

fn apply_boundaries(p: &mut Particle, radius: f64, bottom: f64, lid: f64) {
    // Cylinder walls
    let r_xy = (p.position.x.powi(2) + p.position.y.powi(2)).sqrt();
    if r_xy > radius {
        let scale = radius / r_xy;
        p.position.x *= scale;
        p.position.y *= scale;
        p.velocity.x = 0.0;
        p.velocity.y = 0.0;
    }

    // Bottom
    if p.position.z < bottom {
        p.position.z = bottom;
        if p.velocity.z < 0.0 {
            p.velocity.z = 0.0;
        }
    }

    // Lid
    if p.position.z > lid {
        p.position.z = lid;
        if p.velocity.z > 0.0 {
            p.velocity.z = 0.0;
        }
    }
}

fn main() {
    // Simulation parameters (micrometer units)
    let bucket_radius = 15.0;
    let bucket_height = 50.0;
    let mut lid_height = bucket_height;
    let lid_speed = 0.1; // µm per step

    // Initialize two spheres
    let mut particles = Vec::new();
    particles.extend(init_sphere(Vector3::new(0.0, 0.0, 6.5), 6.5, 2.0));
    particles.extend(init_sphere(Vector3::new(0.0, 0.0, 16.5), 10.0, 2.0));

    let dt = 0.005;
    let steps = 200;

    for _step in 0..steps {
        lid_height -= lid_speed * dt;
        if lid_height < 0.0 { lid_height = 0.0; }

        // Density and pressure
        for i in 0..particles.len() {
            let mut density = 0.0;
            for j in 0..particles.len() {
                let r = (particles[i].position - particles[j].position).norm();
                density += PARTICLE_MASS * kernel(r);
            }
            particles[i].density = density;
            particles[i].pressure = STIFFNESS * (density - REST_DENSITY);
        }

        // Forces and integration
        for i in 0..particles.len() {
            let mut accel = Vector3::zeros();
            for j in 0..particles.len() {
                if i == j { continue; }
                let r_vec = particles[i].position - particles[j].position;
                let r = r_vec.norm();
                if r < SMOOTHING_LENGTH && r > 1e-12 {
                    let grad = grad_kernel(r_vec, r);
                    let pressure_term = (particles[i].pressure + particles[j].pressure)
                        / (2.0 * particles[j].density);
                    accel -= PARTICLE_MASS * pressure_term * grad;
                }
            }
            particles[i].velocity += accel * dt;
        }

        for p in &mut particles {
            p.position += p.velocity * dt;
            apply_boundaries(p, bucket_radius, 0.0, lid_height);
        }
    }

    println!("Simulation finished with {} particles", particles.len());
}

