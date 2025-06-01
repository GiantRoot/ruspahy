//! 粒子运动的时间积分。
//!
//! 这里采用显式 Euler 积分器，根据粒子所受的力
//! 更新其速度和位置。

use crate::particle::ParticleSystem;
use crate::config::SimConfig;

/// 根据桶约束调整粒子位置。
fn apply_bucket_constraints(psys: &mut ParticleSystem, config: &SimConfig, step: usize, dt: f64) {
    if let Some(bucket) = &config.bucket {
        let mut lid = bucket.height - (step as f64 + 1.0) * bucket.lid_speed * dt;
        if lid < bucket.min_height {
            lid = bucket.min_height;
        }
        for p in &mut psys.particles {
            // 底部
            if p.position[2] < 0.0 {
                p.position[2] = 0.0;
                p.velocity[2] = 0.0;
            }
            // 侧壁
            let r = (p.position[0].powi(2) + p.position[1].powi(2)).sqrt();
            if r > bucket.radius {
                let scale = bucket.radius / r;
                p.position[0] *= scale;
                p.position[1] *= scale;
                p.velocity[0] = 0.0;
                p.velocity[1] = 0.0;
            }
            // 盖子
            if p.position[2] > lid {
                p.position[2] = lid;
                if p.velocity[2] > 0.0 {
                    p.velocity[2] = 0.0;
                }
            }
        }
    }
}

/// 使用显式 Euler 方法推进一个时间步。
pub fn integrate(psys: &mut ParticleSystem, dt: f64, config: &SimConfig, step: usize) {
    for p in &mut psys.particles {
        for i in 0..3 {
            p.velocity[i] += dt * p.force[i] / p.density;
            p.position[i] += dt * p.velocity[i];
        }
    }
    apply_bucket_constraints(psys, config, step, dt);
}
