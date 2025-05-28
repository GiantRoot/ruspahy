//! 力计算相关函数。
//!
//! 本模块实现了粒子密度、压力和粘性等计算，
//! 这些计算均依据 SPH 核函数完成。
//! 当前实现采用全对遍历，在粒子数较多时效率不高。

use crate::particle::ParticleSystem;
use crate::sph_kernel::SPHKernel;
use rayon::prelude::*;

/// 计算每个粒子的密度和压力。
pub fn compute_density_pressure(psys: &mut ParticleSystem, kernel: &SPHKernel) {
    let all_particles = psys.particles.clone();
    let mass = 1.0;

    psys
        .particles
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, pi)| {
            let mut density = mass * kernel.w_poly6(0.0);
            for &j in &psys.neighbors[i] {
                let pj = &all_particles[j];
                let r2 = squared_distance(pi.position, pj.position);
                density += mass * kernel.w_poly6(r2);
            }
            pi.density = density;
            pi.pressure = 1000.0 * (density - 1000.0);
        });
}

/// 计算压力与粘性力。
pub fn compute_forces(psys: &mut ParticleSystem, kernel: &SPHKernel) {
    let mass = 1.0;
    let viscosity = 0.1;

    for p in &mut psys.particles {
        p.force = [0.0; 3];
    }

    for i in 0..psys.particles.len() {
        for &j in &psys.neighbors[i] {
            if j <= i {
                continue;
            }

            let mat_a = psys.particles[i].material_id;
            let mat_b = psys.particles[j].material_id;
            let interface = psys.find_interface(mat_a, mat_b).cloned();

            // 安全地同时获取两个粒子的可变引用
            let (pi, pj) = {
                let (left, right) = psys.particles.split_at_mut(j);
                (&mut left[i], &mut right[0])
            };

            let r_vec = vector_sub(pj.position, pi.position);
            let r2 = dot(r_vec, r_vec);
            let r = r2.sqrt();

            // 压力项（对称形式以满足动量守恒）
            let grad_w = kernel.grad_w_spiky(r, r_vec);
            let pressure_term =
                mass * (pi.pressure / (pi.density * pi.density) + pj.pressure / (pj.density * pj.density));
            let mut pair_force = [0.0; 3];
            for k in 0..3 {
                pair_force[k] -= pressure_term * grad_w[k];
            }

            // 粘性项
            let vel_diff = vector_sub(pj.velocity, pi.velocity);
            let lap_w = kernel.lap_w_viscosity(r);
            for k in 0..3 {
                pair_force[k] += viscosity * mass * vel_diff[k] / pj.density * lap_w;
            }

            // 界面粘结力
            if let Some(interface) = interface {
                if r < kernel.h {
                    let dir = if r > 0.0 {
                        [r_vec[0] / r, r_vec[1] / r, r_vec[2] / r]
                    } else {
                        [0.0; 3]
                    };
                    let coeff = interface.bond_strength * (kernel.h - r) / kernel.h;
                    for k in 0..3 {
                        pair_force[k] += coeff * dir[k];
                    }
                }
            }

            for k in 0..3 {
                pi.force[k] += pair_force[k];
                pj.force[k] -= pair_force[k];
            }
        }
    }
}

/// 根据压力值计算简化的等效应力，并考虑材料屈服
pub fn compute_stress(psys: &mut ParticleSystem) {
    for p in &mut psys.particles {
        let yield_strength = psys
            .materials
            .get(p.material_id)
            .and_then(|m| m.yield_strength)
            .unwrap_or(f64::INFINITY);
        let mut sigma = p.pressure.abs();
        if sigma > yield_strength {
            sigma = yield_strength;
        }
        p.stress = sigma;
    }
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
