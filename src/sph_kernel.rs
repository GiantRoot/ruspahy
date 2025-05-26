pub struct SPHKernel {
    pub h: f64,         // smoothing length
    pub h2: f64,        // h^2
    pub poly6_coef: f64,
    pub spiky_grad_coef: f64,
    pub viscosity_lap_coef: f64,
}

impl SPHKernel {
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

    pub fn w_poly6(&self, r2: f64) -> f64 {
        if r2 < self.h2 {
            let diff = self.h2 - r2;
            self.poly6_coef * diff.powi(3)
        } else {
            0.0
        }
    }

    pub fn grad_w_spiky(&self, r: f64, dir: [f64; 3]) -> [f64; 3] {
        if r > 0.0 && r < self.h {
            let coeff = self.spiky_grad_coef * (self.h - r).powi(2) / r;
            [dir[0] * coeff, dir[1] * coeff, dir[2] * coeff]
        } else {
            [0.0; 3]
        }
    }

    pub fn lap_w_viscosity(&self, r: f64) -> f64 {
        if r < self.h {
            self.viscosity_lap_coef * (self.h - r)
        } else {
            0.0
        }
    }
}
