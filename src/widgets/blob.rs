use super::INode;
use crate::types::{Constraints, Position, Size, SizeFlex};

use crate::rendering::AppBuffer;

pub struct Blob {
    ideal_size: Size,
    size: Size,
    pos: Position,
    color: u32,
    zindex: usize,
}

impl INode for Blob {
    fn give_constraints(&mut self, c: Constraints) -> SizeFlex {
        self.size.w = if self.ideal_size.w < c.w_min {
            c.w_min
        } else if self.ideal_size.w > c.w_max {
            c.w_max
        } else {
            self.ideal_size.w
        };
        self.size.h = if self.ideal_size.h < c.h_min {
            c.h_min
        } else if self.ideal_size.h > c.h_max {
            c.h_max
        } else {
            self.ideal_size.h
        };
        self.size.into()
    }
    fn on_draw(&self, p: &Position, buffer: &mut AppBuffer) {
        let p = Position {
            x: p.x + self.pos.x,
            y: p.y + self.pos.y,
        };
        buffer.draw_rectangle(
            p.x as isize,
            p.y as isize,
            self.size.w as usize,
            self.size.h as usize,
            self.color,
            self.zindex,
        );
    }
    fn on_tick(&mut self, _: super::app::TickInfo) {}

    fn set_position(&mut self, p: Position) {
        self.pos = p;
    }

    fn get_position(&self) -> Position {
        self.pos
    }

    fn set_size(&mut self, s: Size) {
        self.size = s;
    }

    fn get_size(&self) -> Size {
        self.size
    }

    fn set_zindex(&mut self, i: usize) {
        self.zindex = i;
    }

    fn get_zindex(&self) -> usize {
        self.zindex
    }
}

impl Blob {
    pub fn new(w: f32, h: f32) -> Self {
        Self {
            ideal_size: Size { w, h },
            size: Size::default(),
            pos: Position::default(),
            color: 0xFFFFFF,
            zindex: 0,
        }
    }

    pub fn color(mut self, color: u32) -> Self {
        self.color = color;
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
