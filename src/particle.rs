/// 单个 SPH 粒子的表示。
#[derive(Clone)]
pub struct Particle {
    pub position: [f64; 3],
    pub velocity: [f64; 3],
    pub force: [f64; 3],
    pub density: f64,
    pub pressure: f64,
    pub material_id: usize,
}

/// 组成模拟区域的粒子集合。
pub struct ParticleSystem {
    pub particles: Vec<Particle>,
    /// 每个粒子的邻居索引列表
    pub neighbors: Vec<Vec<usize>>, 
}

impl ParticleSystem {
    /// 根据模拟配置在规则网格上生成粒子。
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
                        material_id: 0,
                    });
                }
            }
        }
        let neighbors = vec![Vec::new(); particles.len()];
        Self { particles, neighbors }
    }

    /// 创建两个相同半径的球体，用于碰撞模拟。
    pub fn two_spheres(
        center1: [f64; 3],
        center2: [f64; 3],
        radius: f64,
        spacing: f64,
        velocity1: [f64; 3],
        velocity2: [f64; 3],
        material_id1: usize,
        material_id2: usize,
    ) -> Self {
        fn add_sphere(
            particles: &mut Vec<Particle>,
            center: [f64; 3],
            radius: f64,
            spacing: f64,
            velocity: [f64; 3],
            material_id: usize,
        ) {
            let n = (radius / spacing).ceil() as i32;
            let r2 = radius * radius;
            for ix in -n..=n {
                for iy in -n..=n {
                    for iz in -n..=n {
                        let dx = ix as f64 * spacing;
                        let dy = iy as f64 * spacing;
                        let dz = iz as f64 * spacing;
                        if dx * dx + dy * dy + dz * dz <= r2 {
                            particles.push(Particle {
                                position: [center[0] + dx, center[1] + dy, center[2] + dz],
                                velocity,
                                force: [0.0; 3],
                                density: 1000.0,
                                pressure: 0.0,
                                material_id,
                            });
                        }
                    }
                }
            }
        }

        let mut particles = Vec::new();
        add_sphere(
            &mut particles,
            center1,
            radius,
            spacing,
            velocity1,
            material_id1,
        );
        add_sphere(
            &mut particles,
            center2,
            radius,
            spacing,
            velocity2,
            material_id2,
        );

        let neighbors = vec![Vec::new(); particles.len()];
        Self { particles, neighbors }
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
