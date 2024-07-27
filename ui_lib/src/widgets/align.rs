use crate::{rendering::render_object::PositionRenderObject, types::Alignment};

use super::Widget;

pub struct Center;
impl Center {
    pub fn new(c: Box<dyn Widget>) -> Align {
        Align::new(Alignment::Center, c)
    }
}

pub struct Align {
    child: Box<dyn Widget>,
    aligment: Alignment,
    width_factor: f32,
    height_factor: f32,
}

impl Align {
    pub fn new(a: Alignment, c: Box<dyn Widget>) -> Self {
        Self {
            child: c,
            aligment: a,
            width_factor: 1.0,
            height_factor: 1.0,
        }
    }

    pub fn width_factor(mut self, width_factor: f32) -> Self {
        self.width_factor = width_factor;
        self
    }

    pub fn height_factor(mut self, height_factor: f32) -> Self {
        self.height_factor = height_factor;
        self
    }

    pub fn b(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Widget for Align {
    fn create_render_object(&self) -> Box<dyn crate::rendering::render_object::RenderObject> {
        Box::new(PositionRenderObject::new(
            self.child.create_render_object(),
            self.aligment,
            self.width_factor,
            self.height_factor,
        ))
    }
}
