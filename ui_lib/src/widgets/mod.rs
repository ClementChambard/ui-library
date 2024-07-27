use crate::rendering::render_object::RenderObject;

pub mod align;
pub mod app;
pub mod blob;
pub mod constrained;
pub mod elevate;
pub mod flex;
pub mod flexible;
pub mod list;
pub mod position;
pub mod spacer;

pub use align::{Align, Center};
pub use app::App;
pub use blob::Blob;
pub use constrained::{ConstrainedBox, LimitedBox, SizedBox};
pub use elevate::Elevate;
pub use flex::{Column, Flex, Row};
pub use flexible::{Expanded, Flexible};
pub use list::WidgetList;
pub use position::PositionBox;

pub trait Widget {
    fn create_render_object(&self) -> Box<dyn RenderObject>;
}

#[macro_export]
macro_rules! b_vec {
    ($fst:expr $(,$next:expr)* $(,)?) => {
        vec![$fst.b() $(, $next.b())*]
    };
    () => {
        vec![]
    }
}
