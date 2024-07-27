use std::collections::HashMap;

use crate::{
    rendering::{RenderContext, RenderContextMut},
    types::{BoxConstraints, Position, Size},
};

use super::{RenderObject, RenderObjectProp};

pub struct LimitedRenderObject {
    child: Option<Box<dyn RenderObject>>,
    max_width: f32,
    max_height: f32,

    props: HashMap<String, RenderObjectProp>,
    render_pos: Position,
}

impl LimitedRenderObject {
    pub fn new(child: Option<Box<dyn RenderObject>>, max_width: f32, max_height: f32) -> Self {
        Self {
            child,
            max_width,
            max_height,
            props: HashMap::new(),
            render_pos: Position::default(),
        }
    }

    fn limit_constraints(&self, constraints: BoxConstraints) -> BoxConstraints {
        BoxConstraints {
            min_width: constraints.min_width,
            min_height: constraints.min_height,
            max_width: if constraints.has_bounded_width() {
                constraints.max_width
            } else {
                constraints.constrain_width(self.max_width)
            },
            max_height: if constraints.has_bounded_height() {
                constraints.max_height
            } else {
                constraints.constrain_height(self.max_height)
            },
        }
    }

    fn compute_size(&mut self, constraints: BoxConstraints) -> Size {
        let cns = self.limit_constraints(constraints.clone());
        match self.child {
            Some(ref mut c) => constraints.constrain(c.calculate_layout(cns)),
            None => cns.constrain(Size::default()),
        }
    }
}

impl RenderObject for LimitedRenderObject {
    fn render(&self, context: &mut RenderContext, context_mut: RenderContextMut) {
        if let Some(ref c) = self.child {
            c.render(context, context_mut.update(self));
        }
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
        self.compute_size(constraints)
    }
}
