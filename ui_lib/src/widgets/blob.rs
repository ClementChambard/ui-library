use crate::{rendering::render_object::BlobRenderObject, types::Size};

pub struct Blob {
    pub size: Size,
    pub color: u32,
}

impl super::Widget for Blob {
    fn create_render_object(&self) -> Box<dyn crate::rendering::render_object::RenderObject> {
        Box::new(BlobRenderObject::new(self.color, self.size))
    }
}

impl Blob {
    pub fn new(width: f32, height: f32, color: u32) -> Self {
        Self {
            size: Size {
                w: width,
                h: height,
            },
            color,
        }
    }

    pub fn b(self) -> Box<Self> {
        Box::new(self)
    }
}
