use crate::{
    rendering::AppBuffer,
    types::{Position, Size},
};

use super::INode;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ButtonCallbackType {
    Pressed,
    Released,
    Hovered,
    UnHovered,
}

type Callback = Box<dyn Fn()>;

pub struct ButtonCallback {
    typ: ButtonCallbackType,
    callback: Callback,
}

pub struct Button {
    held: bool,
    hovered: bool,
    callbacks: Vec<ButtonCallback>,
    pos: Position,
    size: Size,
    ideal_size: Size,
    zindex: usize,
}

impl Button {
    pub fn new(w: f32, h: f32) -> Self {
        Self {
            held: false,
            hovered: false,
            callbacks: Vec::new(),
            pos: Position::default(),
            size: Size::default(),
            ideal_size: Size { w, h },
            zindex: 0,
        }
    }

    pub fn callback<T: Fn() + 'static>(mut self, t: ButtonCallbackType, f: T) -> Self {
        self.callbacks.push(ButtonCallback {
            callback: Box::new(f),
            typ: t,
        });
        self
    }

    pub fn boxx(self) -> Box<Self> {
        Box::new(self)
    }

    fn call(&self, t: ButtonCallbackType) {
        for bt in &self.callbacks {
            if bt.typ == t {
                let f = &bt.callback;
                f();
            }
        }
    }

    pub fn zindex(mut self, i: usize) -> Self {
        self.set_zindex(i);
        self
    }
}

impl INode for Button {
    fn give_constraints(&mut self, c: crate::types::Constraints) -> crate::types::SizeFlex {
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
    fn on_draw(&self, p: &crate::types::Position, buffer: &mut AppBuffer) {
        let p = Position {
            x: p.x + self.pos.x,
            y: p.y + self.pos.y,
        };
        let mut color = 0x000000;
        if self.hovered {
            color = 0x333333;
        }
        if self.held {
            color = 0x666666;
        }
        buffer.draw_rectangle(
            p.x as isize,
            p.y as isize,
            self.size.w as usize,
            self.size.h as usize,
            color,
            self.zindex,
        );
    }
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
    fn on_tick(&mut self, info: super::app::TickInfo) {
        let p = Position {
            x: info.parent_pos.x + self.pos.x,
            y: info.parent_pos.y + self.pos.y,
        };
        if let Some((mouse_x, mouse_y, mouse_z)) = info.mouse_pos {
            let old_hovered = self.hovered;
            self.hovered = mouse_x >= (p.x as usize)
                && mouse_x <= ((p.x + self.size.w) as usize)
                && mouse_y >= (p.y as usize)
                && mouse_y <= ((p.y + self.size.h) as usize)
                && mouse_z <= self.zindex;
            if !self.hovered {
                if old_hovered {
                    self.call(ButtonCallbackType::UnHovered);
                }
                if self.held {
                    self.call(ButtonCallbackType::Released);
                }
                self.held = false;
                return;
            }
            if self.hovered && !old_hovered {
                self.call(ButtonCallbackType::Hovered);
            }
        } else {
            if self.hovered {
                self.call(ButtonCallbackType::UnHovered);
            }
            self.hovered = false;
            if self.held {
                self.call(ButtonCallbackType::Released);
            }
            self.held = false;
            return;
        }
        if info.window_ref.get_mouse_down(minifb::MouseButton::Left) {
            if !self.held {
                self.call(ButtonCallbackType::Pressed);
            }
            self.held = true;
        } else {
            if self.held {
                self.call(ButtonCallbackType::Released);
            }
            self.held = false;
        }
    }
}
