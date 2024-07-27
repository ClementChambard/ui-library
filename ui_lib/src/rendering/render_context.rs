use crate::types::Position;

use super::{render_object::RenderObject, AppBuffer};

pub struct RenderContext<'a> {
    pub appbuffer: &'a mut AppBuffer,
}

#[derive(Clone)]
pub struct RenderContextMut {
    pub position: Position,
    pub elevation: usize,
}

impl RenderContextMut {
    pub fn update(&self, render_object: &dyn RenderObject) -> Self {
        let elevation = self.elevation + render_object.get_prop_usize("elevation").unwrap_or(0);
        let position = self.position + render_object.get_render_pos();
        RenderContextMut {
            position,
            elevation,
        }
    }
}
