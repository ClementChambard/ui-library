use std::collections::HashMap;

use crate::{
    rendering::{RenderContext, RenderContextMut},
    types::{Alignment, BoxConstraints, Position, Size},
};

use super::{RenderObject, RenderObjectProp};

pub struct PositionRenderObject {
    child: Box<dyn RenderObject>,
    alignment: Alignment,
    width_factor: f32,
    height_factor: f32,
    // text_direction
    props: HashMap<String, RenderObjectProp>,
    render_pos: Position,
}

impl PositionRenderObject {
    pub fn new(
        child: Box<dyn RenderObject>,
        alignment: Alignment,
        width_factor: f32,
        height_factor: f32,
    ) -> Self {
        Self {
            child,
            alignment,
            width_factor,
            height_factor,
            props: HashMap::new(),
            render_pos: Position::default(),
        }
    }
}

impl RenderObject for PositionRenderObject {
    fn render(&self, context: &mut RenderContext, context_mut: RenderContextMut) {
        self.child.render(context, context_mut.update(self));
    }

    fn set_render_pos(&mut self, render_pos: Position) {
        self.render_pos = render_pos;
    }

    fn get_render_pos(&self) -> Position {
        self.render_pos
    }

    fn get_prop(&self, prop: &str) -> Option<&RenderObjectProp> {
        self.props.get(prop)
    }

    fn set_prop(&mut self, prop: &str, val: RenderObjectProp) {
        self.props.insert(prop.to_string(), val);
    }

    fn calculate_layout(&mut self, constraints: BoxConstraints) -> Size {
        let (px, py) = self.alignment.loc();
        let wanted_size = self.child.calculate_layout(constraints.loosen());
        let container_width = constraints.constrain_width(wanted_size.w * self.width_factor);
        let container_height = constraints.constrain_height(wanted_size.h * self.height_factor);
        let x = (container_width - wanted_size.w) * (px + 1.0) / 2.0;
        let y = (container_height - wanted_size.h) * (py + 1.0) / 2.0;
        self.child.set_render_pos(Position { x, y });
        Size {
            w: container_width,
            h: container_height,
        }
    }
}

pub struct MoveRenderObject {
    child: Box<dyn RenderObject>,
    pos: Position,
    absolute: bool,

    props: HashMap<String, RenderObjectProp>,
    render_pos: Position,
}

impl MoveRenderObject {
    pub fn new(child: Box<dyn RenderObject>, pos: Position, absolute: bool) -> Self {
        Self {
            child,
            pos,
            absolute,
            props: HashMap::new(),
            render_pos: Position::default(),
        }
    }
}

impl RenderObject for MoveRenderObject {
    fn render(&self, context: &mut RenderContext, context_mut: RenderContextMut) {
        let this_pos = if self.absolute {
            Position::default()
        } else {
            self.render_pos
        };
        let context_mut = RenderContextMut {
            position: this_pos,
            elevation: self.get_prop_usize("elevation").unwrap_or(0),
            ..context_mut
        };
        self.child.render(context, context_mut);
    }

    fn set_render_pos(&mut self, render_pos: Position) {
        self.render_pos = render_pos;
    }

    fn get_render_pos(&self) -> Position {
        self.render_pos
    }

    fn get_prop(&self, prop: &str) -> Option<&RenderObjectProp> {
        self.props.get(prop)
    }

    fn set_prop(&mut self, prop: &str, val: RenderObjectProp) {
        self.props.insert(prop.to_string(), val);
    }

    fn calculate_layout(&mut self, _constraints: BoxConstraints) -> Size {
        self.child.calculate_layout(BoxConstraints::default());
        self.child.set_render_pos(self.pos);
        Size { w: 0.0, h: 0.0 }
    }
}
