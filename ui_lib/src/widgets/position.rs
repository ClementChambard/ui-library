use crate::rendering::render_object::MoveRenderObject;

use super::Widget;

pub struct PositionBox {
    child: Box<dyn Widget>,
    pos: crate::types::Position,
    absolute: bool,
}

impl PositionBox {
    pub fn new(child: Box<dyn Widget>, x: f32, y: f32) -> Self {
        Self {
            child,
            pos: crate::types::Position { x, y },
            absolute: false,
        }
    }

    pub fn absolute(mut self) -> Self {
        self.absolute = true;
        self
    }

    pub fn b(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Widget for PositionBox {
    fn create_render_object(&self) -> Box<dyn crate::rendering::render_object::RenderObject> {
        Box::new(MoveRenderObject::new(
            self.child.create_render_object(),
            self.pos,
            self.absolute,
        ))
    }
}
