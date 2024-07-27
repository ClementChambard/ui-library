use std::collections::HashMap;

use crate::{
    rendering::{RenderContext, RenderContextMut},
    types::{BoxConstraints, Position, Size},
};

use super::{RenderObject, RenderObjectProp};

pub struct BlobRenderObject {
    color: u32,
    size: Size,

    props: HashMap<String, RenderObjectProp>,
    render_pos: Position,
    render_size: Size,
}

impl BlobRenderObject {
    pub fn new(color: u32, size: Size) -> Self {
        Self {
            color,
            size,

            props: HashMap::new(),
            render_pos: Position::default(),
            render_size: Size::default(),
        }
    }
}

impl RenderObject for BlobRenderObject {
    fn calculate_layout(&mut self, constraints: BoxConstraints) -> Size {
        self.render_size = constraints.constrain(self.size);
        self.render_size
    }

    fn set_render_pos(&mut self, render_pos: Position) {
        self.render_pos = render_pos;
    }

    fn get_render_pos(&self) -> Position {
        self.render_pos
    }

    fn render(&self, context: &mut RenderContext, context_mut: RenderContextMut) {
        let context_mut = context_mut.update(self);
        context.appbuffer.draw_rectangle(
            context_mut.position.x as isize,
            context_mut.position.y as isize,
            self.render_size.w as usize,
            self.render_size.h as usize,
            self.color,
            context_mut.elevation,
        );
    }

    fn get_prop(&self, prop: &str) -> Option<&RenderObjectProp> {
        self.props.get(prop)
    }

    fn set_prop(&mut self, prop: &str, val: RenderObjectProp) {
        self.props.insert(prop.to_string(), val);
    }
}
