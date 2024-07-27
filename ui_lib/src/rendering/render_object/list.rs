use crate::{
    rendering::{RenderContext, RenderContextMut},
    types::{BoxConstraints, Position, Size},
};

use super::RenderObject;

pub struct ListRenderObject {
    children: Vec<Box<dyn RenderObject>>,
}

impl ListRenderObject {
    pub fn new(children: Vec<Box<dyn RenderObject>>) -> Self {
        Self { children }
    }
}

impl RenderObject for ListRenderObject {
    fn calculate_layout(&mut self, constraints: BoxConstraints) -> Size {
        for c in &mut self.children {
            c.calculate_layout(constraints.clone());
        }
        return constraints.smallest();
    }

    fn render(&self, context: &mut RenderContext, context_mut: RenderContextMut) {
        for c in &self.children {
            c.render(context, context_mut.clone());
        }
    }
}
