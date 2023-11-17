use crate::{
    rendering::AppBuffer,
    types::{Constraints, Position, Size, SizeFlex},
};

use super::INode;

pub struct AbsoluteWrapper {
    child: Box<dyn INode>,
    pub pos: Position,
    pub absolute: bool,
}

impl AbsoluteWrapper {
    pub fn new(child: Box<dyn INode>) -> Self {
        Self {
            child,
            pos: Position::default(),
            absolute: false,
        }
    }

    pub fn x(mut self, x: f32) -> Self {
        self.pos.x = x;
        self
    }

    pub fn y(mut self, y: f32) -> Self {
        self.pos.y = y;
        self
    }

    pub fn absolute(mut self) -> Self {
        self.absolute = true;
        self
    }

    pub fn zindex(mut self, i: usize) -> Self {
        self.set_zindex(i);
        self
    }

    pub fn boxx(self) -> Box<Self> {
        Box::new(self)
    }
}

impl INode for AbsoluteWrapper {
    fn give_constraints(&mut self, _: crate::types::Constraints) -> SizeFlex {
        self.child.give_constraints(Constraints::default());
        Size::default().into()
    }

    fn set_position(&mut self, _: crate::types::Position) {
        self.child.set_position(Position::default());
    }

    fn get_position(&self) -> Position {
        self.child.get_position()
    }

    fn set_size(&mut self, s: Size) {
        self.child.set_size(s);
    }

    fn get_size(&self) -> Size {
        self.child.get_size()
    }

    fn on_draw(&self, p: &Position, buffer: &mut AppBuffer) {
        let sum = p + &self.pos;
        let p = if self.absolute { &self.pos } else { &sum };
        self.child.on_draw(p, buffer);
    }

    fn on_tick(&mut self, info: super::app::TickInfo) {
        self.child.on_tick(info);
    }

    fn set_zindex(&mut self, i: usize) {
        self.child.set_zindex(i);
    }

    fn get_zindex(&self) -> usize {
        self.child.get_zindex()
    }
}
