use crate::{
    rendering::AppBuffer,
    types::{Constraints, Position, Size, SizeFlex},
};

use super::INode;

pub struct ConstrainedBox {
    constraints: Constraints,
    pos: Position,
    child: Box<dyn INode>,
    flex_x: usize,
    flex_y: usize,
}

impl INode for ConstrainedBox {
    fn give_constraints(&mut self, c: Constraints) -> SizeFlex {
        let c = Constraints::most_restraining(&self.constraints, &c);
        let mut s = self.child.give_constraints(c);
        s.flex_x = self.flex_x;
        s.flex_y = self.flex_y;
        s
    }
    fn on_draw(&self, p: &Position, buffer: &mut AppBuffer) {
        self.child.on_draw(&(p + &self.pos), buffer);
    }
    fn on_tick(&mut self, info: super::app::TickInfo) {
        let info = super::app::TickInfo {
            parent_pos: Position {
                x: info.parent_pos.x + self.pos.x,
                y: info.parent_pos.y + self.pos.y,
            },
            ..info
        };
        self.child.on_tick(info);
    }
    fn set_position(&mut self, p: Position) {
        self.pos = p;
    }
    fn get_position(&self) -> Position {
        self.pos
    }
    fn set_size(&mut self, s: Size) {
        self.child.set_size(s);
    }
    fn get_size(&self) -> Size {
        self.child.get_size()
    }
    fn set_zindex(&mut self, i: usize) {
        self.child.set_zindex(i);
    }
    fn get_zindex(&self) -> usize {
        self.child.get_zindex()
    }
}

impl ConstrainedBox {
    pub fn new(c: Box<dyn INode>) -> Self {
        Self {
            constraints: Constraints::default(),
            pos: Position::default(),
            child: c,
            flex_x: 0,
            flex_y: 0,
        }
    }

    pub fn boxx(self) -> Box<Self> {
        Box::new(self)
    }

    pub fn zindex(mut self, i: usize) -> Self {
        self.set_zindex(i);
        self
    }

    pub fn constrain(mut self, c: Constraints) -> Self {
        self.constraints = c;
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        self.constraints.w_max = w;
        self.constraints.w_min = w;
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        self.constraints.h_max = h;
        self.constraints.h_min = h;
        self
    }

    pub fn min_width(mut self, w: f32) -> Self {
        self.constraints.w_min = w;
        self
    }

    pub fn min_height(mut self, h: f32) -> Self {
        self.constraints.h_min = h;
        self
    }

    pub fn max_width(mut self, w: f32) -> Self {
        self.constraints.w_max = w;
        self
    }

    pub fn max_height(mut self, h: f32) -> Self {
        self.constraints.h_max = h;
        self
    }

    pub fn fill_width(self) -> Self {
        self.width(f32::INFINITY)
    }

    pub fn flex_x(mut self, flex: usize) -> Self {
        self.flex_x = flex;
        self
    }

    pub fn flex_y(mut self, flex: usize) -> Self {
        self.flex_y = flex;
        self
    }

    pub fn fill_height(self) -> Self {
        self.height(f32::INFINITY)
    }
}
