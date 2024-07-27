mod blob;
mod constrained;
mod flex;
mod limited;
mod list;
mod position;
mod props;
pub use blob::BlobRenderObject;
pub use constrained::ConstrainedRenderObject;
pub use flex::RenderFlex;
pub use limited::LimitedRenderObject;
pub use list::ListRenderObject;
pub use position::{MoveRenderObject, PositionRenderObject};
pub use props::RenderObjectProp;

use super::{render_context::RenderContextMut, RenderContext};
use crate::types::{BoxConstraints, Position, Size};

pub trait RenderObject {
    fn render(&self, _context: &mut RenderContext, _context_mut: RenderContextMut) {}
    fn calculate_layout(&mut self, _constraints: BoxConstraints) -> Size {
        Size::default()
    }
    fn set_render_pos(&mut self, _render_pos: Position) {}
    fn get_render_pos(&self) -> Position {
        Position::default()
    }
    fn get_prop(&self, _prop: &str) -> Option<&RenderObjectProp> {
        None
    }
    fn get_prop_i32(&self, prop: &str) -> Option<i32> {
        match self.get_prop(prop)? {
            RenderObjectProp::I32(i) => Some(*i),
            _ => None,
        }
    }
    fn get_prop_u32(&self, prop: &str) -> Option<u32> {
        match self.get_prop(prop)? {
            RenderObjectProp::U32(i) => Some(*i),
            _ => None,
        }
    }
    fn get_prop_usize(&self, prop: &str) -> Option<usize> {
        match self.get_prop(prop)? {
            RenderObjectProp::Usize(i) => Some(*i),
            _ => None,
        }
    }
    fn get_prop_f32(&self, prop: &str) -> Option<f32> {
        match self.get_prop(prop)? {
            RenderObjectProp::F32(f) => Some(*f),
            _ => None,
        }
    }

    fn set_prop(&mut self, _prop: &str, _val: RenderObjectProp) {}
    fn set_prop_i32(&mut self, prop: &str, val: i32) {
        self.set_prop(prop, RenderObjectProp::I32(val));
    }
    fn set_prop_u32(&mut self, prop: &str, val: u32) {
        self.set_prop(prop, RenderObjectProp::U32(val));
    }
    fn set_prop_usize(&mut self, prop: &str, val: usize) {
        self.set_prop(prop, RenderObjectProp::Usize(val));
    }
    fn set_prop_f32(&mut self, prop: &str, val: f32) {
        self.set_prop(prop, RenderObjectProp::F32(val));
    }
}
