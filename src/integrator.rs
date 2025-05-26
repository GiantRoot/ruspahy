//! 粒子运动的时间积分。
//!
//! 这里采用显式 Euler 积分器，根据粒子所受的力
//! 更新其速度和位置。

use crate::particle::ParticleSystem;

/// 使用显式 Euler 方法推进一个时间步。
pub fn integrate(psys: &mut ParticleSystem, dt: f64) {
    for p in &mut psys.particles {
        for i in 0..3 {
            p.velocity[i] += dt * p.force[i] / p.density;
            p.position[i] += dt * p.velocity[i];
        }
    }
}
