//! SPH 求解器使用的平滑核函数。

/// 预计算的常见 SPH 核函数常数。
pub struct SPHKernel {
    pub h: f64,         // 平滑长度
    pub h2: f64,        // h 的平方
    pub poly6_coef: f64,
    pub spiky_grad_coef: f64,
    pub viscosity_lap_coef: f64,
}

impl SPHKernel {
    /// 根据给定的平滑长度 `h` 构建核函数系数。
    pub fn new(h: f64) -> Self {
        let h2 = h * h;
        let poly6_coef = 315.0 / (64.0 * std::f64::consts::PI * h.powi(9));
        let spiky_grad_coef = -45.0 / (std::f64::consts::PI * h.powi(6));
        let viscosity_lap_coef = 45.0 / (std::f64::consts::PI * h.powi(6));
        Self {
            h,
            h2,
            poly6_coef,
            spiky_grad_coef,
            viscosity_lap_coef,
        }
    }

    /// Poly6 核函数用于密度估计。
    pub fn w_poly6(&self, r2: f64) -> f64 {
        if r2 < self.h2 {
            let diff = self.h2 - r2;
            self.poly6_coef * diff.powi(3)
        } else {
            0.0
        }
    }

    /// Spiky 核函数的梯度用于计算压力项。
    pub fn grad_w_spiky(&self, r: f64, dir: [f64; 3]) -> [f64; 3] {
        if r > 0.0 && r < self.h {
            let coeff = self.spiky_grad_coef * (self.h - r).powi(2) / r;
            [dir[0] * coeff, dir[1] * coeff, dir[2] * coeff]
        } else {
            [0.0; 3]
        }
    }

    /// 粘性核函数的拉普拉斯算子。
    pub fn lap_w_viscosity(&self, r: f64) -> f64 {
        if r < self.h {
            self.viscosity_lap_coef * (self.h - r)
        } else {
            0.0
        }
    }
}
