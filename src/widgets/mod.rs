pub mod absolute_wrapper;
pub mod align;
pub mod app;
pub mod blob;
pub mod button;
pub mod center;
pub mod constrained_box;
pub mod container;

use crate::{
    rendering::AppBuffer,
    types::{Constraints, Position, Size, SizeFlex},
};

pub use absolute_wrapper::AbsoluteWrapper;
pub use align::{Align, Alignment};
pub use app::App;
pub use blob::Blob;
pub use button::{Button, ButtonCallbackType};
pub use center::Center;
pub use constrained_box::ConstrainedBox;
pub use container::{AlignItems, Container, FlexDirection, JustifyContent};

pub trait INode {
    fn give_constraints(&mut self, c: Constraints) -> SizeFlex;
    fn on_draw(&self, p: &Position, buffer: &mut AppBuffer);
    fn on_tick(&mut self, info: app::TickInfo);
    fn set_zindex(&mut self, i: usize);
    fn get_zindex(&self) -> usize;
    fn set_position(&mut self, p: Position);
    fn get_position(&self) -> Position;
    fn set_size(&mut self, s: Size);
    fn get_size(&self) -> Size;
}
