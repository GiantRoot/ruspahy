//! 力计算相关函数。
//!
//! 本模块实现了粒子密度、压力和粘性等计算，
//! 这些计算均依据 SPH 核函数完成。
//! 当前实现采用全对遍历，在粒子数较多时效率不高。

use crate::particle::{Particle, ParticleSystem};
use crate::sph_kernel::SPHKernel;
use rayon::prelude::*;

/// 计算每个粒子的密度和压力。
pub fn compute_density_pressure(psys: &mut ParticleSystem, kernel: &SPHKernel) {
    let all_particles = psys.particles.clone();
    let mass = 1.0;

    psys
        .particles
        .par_iter_mut()
        .for_each(|pi| {
            let mut density = 0.0;
            for pj in &all_particles {
                let r2 = squared_distance(pi.position, pj.position);
                density += mass * kernel.w_poly6(r2);
            }
            pi.density = density;
            pi.pressure = 1000.0 * (density - 1000.0);
        });
}

/// 计算压力与粘性力。
pub fn compute_forces(psys: &mut ParticleSystem, kernel: &SPHKernel) {
    let all_particles = psys.particles.clone();
    let mass = 1.0;
    let viscosity = 0.1;

    psys
        .particles
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, pi)| {
            let mut force = [0.0; 3];
            for (j, pj) in all_particles.iter().enumerate() {
                if i == j {
                    continue;
                }

                let r_vec = vector_sub(pj.position, pi.position);
                let r2 = dot(r_vec, r_vec);
                let r = r2.sqrt();

                // 压力项
                let grad_w = kernel.grad_w_spiky(r, r_vec);
                let pressure_term = (pi.pressure + pj.pressure) / (2.0 * pj.density);
                for k in 0..3 {
                    force[k] -= mass * pressure_term * grad_w[k];
                }

                // 粘性项
                let vel_diff = vector_sub(pj.velocity, pi.velocity);
                let lap_w = kernel.lap_w_viscosity(r);
                for k in 0..3 {
                    force[k] += viscosity * mass * vel_diff[k] / pj.density * lap_w;
                }
            }
            pi.force = force;
        });
}

/// 工具函数：求两三维点的平方距离。
fn squared_distance(a: [f64; 3], b: [f64; 3]) -> f64 {
    (a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)
}

/// 三维向量相减。
fn vector_sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// 三维向量点积。
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0]*b[0] + a[1]*b[1] + a[2]*b[2]
}
