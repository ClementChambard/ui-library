#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct Size {
    pub w: f32,
    pub h: f32,
}

impl Size {
    pub fn constrtaints(&self) -> super::Constraints {
        super::Constraints {
            w_min: self.w,
            w_max: self.w,
            h_min: self.h,
            h_max: self.h,
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct SizeFlex {
    pub w: f32,
    pub h: f32,
    pub flex_x: usize,
    pub flex_y: usize,
}

impl From<Size> for SizeFlex {
    fn from(value: Size) -> Self {
        Self {
            w: value.w,
            h: value.h,
            flex_x: 0,
            flex_y: 0,
        }
    }
}
