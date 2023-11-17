#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Constraints {
    pub w_min: f32,
    pub w_max: f32,
    pub h_min: f32,
    pub h_max: f32,
}

impl Default for Constraints {
    fn default() -> Self {
        Self {
            w_min: 0.0,
            w_max: f32::INFINITY,
            h_min: 0.0,
            h_max: f32::INFINITY,
        }
    }
}

impl Constraints {
    pub fn reduce(self, w: f32, h: f32) -> Self {
        let w_max = self.w_max - w;
        let h_max = self.h_max - h;
        let w_min = if w_max < self.w_min {
            w_max
        } else {
            self.w_min
        };
        let h_min = if h_max < self.h_min {
            h_max
        } else {
            self.h_min
        };
        Self {
            w_max,
            w_min,
            h_max,
            h_min,
        }
    }
    pub fn most_restraining(c1: &Constraints, c2: &Constraints) -> Self {
        let w_max = if c1.w_max < c2.w_max {
            c1.w_max
        } else {
            c2.w_max
        };
        let h_max = if c1.h_max < c2.h_max {
            c1.h_max
        } else {
            c2.h_max
        };
        let mut w_min = if c1.w_min > c2.w_min {
            c1.w_min
        } else {
            c2.w_min
        };
        let mut h_min = if c1.h_min > c2.h_min {
            c1.h_min
        } else {
            c2.h_min
        };
        if w_min > w_max {
            w_min = w_max
        }
        if h_min > h_max {
            h_min = h_max
        }
        Self {
            w_max,
            w_min,
            h_max,
            h_min,
        }
    }
}
