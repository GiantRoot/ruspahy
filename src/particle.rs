/// 单个 SPH 粒子的表示。
#[derive(Clone)]
pub struct Particle {
    pub position: [f64; 3],
    pub velocity: [f64; 3],
    pub force: [f64; 3],
    pub density: f64,
    pub pressure: f64,
    /// Von Mises 应力或其它简化表示
    pub stress: f64,
    pub material_id: usize,
}

/// 组成模拟区域的粒子集合。
pub struct ParticleSystem {
    pub particles: Vec<Particle>,
    /// 每个粒子的邻居索引列表
    pub neighbors: Vec<Vec<usize>>,
    /// 系统中可用的材料列表
    pub materials: Vec<crate::material::Material>,
    /// 材料界面属性列表
    pub interfaces: Vec<crate::material::Interface>,
}

impl ParticleSystem {
    /// 根据配置创建粒子集。若配置中提供 `spheres` 字段，则以球体布置粒子；
    /// 否则在规则网格上生成。
    pub fn new(config: &crate::config::SimConfig) -> Self {
        let mut particles = Vec::new();

        if !config.spheres.is_empty() {
            for s in &config.spheres {
                Self::add_sphere(&mut particles, s, config.spacing);
            }
        } else {
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
                            stress: 0.0,
                            material_id: 0,
                        });
                    }
                }
            }
        }

        let neighbors = vec![Vec::new(); particles.len()];
        Self {
            particles,
            neighbors,
            materials: config.materials.clone(),
            interfaces: config.interfaces.clone(),
        }
    }

    /// 在给定粒子集合中添加球体分布的粒子。
    fn add_sphere(particles: &mut Vec<Particle>, s: &crate::config::SphereConfig, spacing: f64) {
        let n = (s.radius / spacing).ceil() as i32;
        let r2 = s.radius * s.radius;
        for ix in -n..=n {
            for iy in -n..=n {
                for iz in -n..=n {
                    let dx = ix as f64 * spacing;
                    let dy = iy as f64 * spacing;
                    let dz = iz as f64 * spacing;
                    if dx * dx + dy * dy + dz * dz <= r2 {
                        particles.push(Particle {
                            position: [s.center[0] + dx, s.center[1] + dy, s.center[2] + dz],
                            velocity: s.velocity,
                            force: [0.0; 3],
                            density: 1000.0,
                            pressure: 0.0,
                            stress: 0.0,
                            material_id: s.material_id,
                        });
                    }
                }
            }
        }
    }


    /// 为每个粒子构建邻域列表。
    ///
    /// 目前还是代理实现，系统在 [`crate::force`]
    /// 中采用全对形工的算法。
    pub fn build_neighbor_list(&mut self) {
        let h = self.mean_spacing();
        self.neighbors = crate::neighbor::build_neighbor_list(&self.particles, h);
    }

    /// 计算每个粒子受到的力。
    ///
    /// 其中的压力和粘性算法都在
    /// [`crate::force`] 中实现。
    pub fn compute_forces(&mut self) {
        let kernel = crate::sph_kernel::SPHKernel::new(self.mean_spacing());
        crate::force::compute_density_pressure(self, &kernel);
        crate::force::compute_forces(self, &kernel);
        crate::force::compute_stress(self);
    }

    /// 获取两种材料间的界面定义
    pub fn find_interface(
        &self,
        mat_a: usize,
        mat_b: usize,
    ) -> Option<&crate::material::Interface> {
        self.interfaces.iter().find(|iface| {
            (iface.mat_a == mat_a && iface.mat_b == mat_b)
                || (iface.mat_a == mat_b && iface.mat_b == mat_a)
        })
    }

    /// 根据粒子间距推算平滑长度。
    fn mean_spacing(&self) -> f64 {
        if self.particles.is_empty() {
            0.1
        } else {
            let mut min_dist = f64::MAX;
            for i in 0..self.particles.len() {
                for j in i + 1..self.particles.len() {
                    let d = (
                        (self.particles[i].position[0] - self.particles[j].position[0]).powi(2)
                            + (self.particles[i].position[1] - self.particles[j].position[1]).powi(2)
                            + (self.particles[i].position[2] - self.particles[j].position[2]).powi(2)
                    )
                    .sqrt();
                    if d < min_dist {
                        min_dist = d;
                    }
                }
            }
            min_dist
        }
    }
}
