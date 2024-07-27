use std::collections::HashMap;

use crate::{
    rendering::{RenderContext, RenderContextMut},
    types::{BoxConstraints, Position, Size},
};

use super::{RenderObject, RenderObjectProp};

pub struct ConstrainedRenderObject {
    child: Option<Box<dyn RenderObject>>,
    additionnal_constraints: BoxConstraints,

    props: HashMap<String, RenderObjectProp>,
    render_pos: Position,
}

impl ConstrainedRenderObject {
    pub fn new(
        child: Option<Box<dyn RenderObject>>,
        additionnal_constraints: BoxConstraints,
    ) -> Self {
        Self {
            child,
            additionnal_constraints,
            props: HashMap::new(),
            render_pos: Position::default(),
        }
    }
}

impl RenderObject for ConstrainedRenderObject {
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
        match self.child {
            Some(ref mut c) => {
                c.calculate_layout(self.additionnal_constraints.enforce(&constraints))
            }
            None => self
                .additionnal_constraints
                .enforce(&constraints)
                .constrain(Size::default()),
        }
    }
}
