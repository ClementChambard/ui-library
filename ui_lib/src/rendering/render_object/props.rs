#[derive(Clone, Copy, PartialEq)]
pub enum RenderObjectProp {
    I32(i32),
    U32(u32),
    Usize(usize),
    F32(f32),
}

impl From<usize> for RenderObjectProp {
    fn from(value: usize) -> Self {
        Self::Usize(value)
    }
}

impl From<u32> for RenderObjectProp {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}

impl From<i32> for RenderObjectProp {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<f32> for RenderObjectProp {
    fn from(value: f32) -> Self {
        Self::F32(value)
    }
}
