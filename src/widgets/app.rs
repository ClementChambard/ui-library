use super::INode;
use crate::{
    rendering::AppBuffer,
    types::{Constraints, Position, Size},
};

pub struct App {
    pub size: Size,
    root: Box<dyn INode>,
    pub buffer: AppBuffer,
}

#[derive(Clone)]
pub struct TickInfo<'a> {
    pub parent_pos: Position,
    pub mouse_pos: Option<(usize, usize, usize)>,
    pub window_ref: &'a minifb::Window, // TODO: pass other fields here instead
}

impl App {
    pub fn new(w: f32, h: f32, root: Box<dyn INode>) -> Self {
        let mut root = root;
        root.give_constraints(Constraints {
            w_min: w,
            w_max: w,
            h_min: h,
            h_max: h,
        });
        // Create a buffer to store pixel data (RGB)
        Self {
            size: Size { w, h },
            root,
            buffer: AppBuffer::new(w as usize, h as usize),
        }
    }
    pub fn on_draw(&mut self) {
        self.root.on_draw(&Position::default(), &mut self.buffer);
    }
    pub fn on_tick(&mut self, w: &minifb::Window) {
        let (new_width, new_height) = w.get_size();
        if new_width != self.buffer.width || new_height != self.buffer.height {
            // Resize the buffer accordingly
            self.buffer.resize(new_width, new_height);
            self.size.w = new_width as f32;
            self.size.h = new_height as f32;

            self.root.give_constraints(Constraints {
                w_min: self.size.w,
                w_max: self.size.w,
                h_min: self.size.h,
                h_max: self.size.h,
            });
        }

        let mouse_pos = if let Some((mouse_x, mouse_y)) = w.get_mouse_pos(minifb::MouseMode::Clamp)
        {
            let (mouse_x, mouse_y) = (mouse_x as usize, mouse_y as usize);
            let mouse_z = self.buffer.depth[mouse_x + mouse_y * self.buffer.width];
            Some((mouse_x, mouse_y, mouse_z))
        } else {
            None
        };

        self.buffer.clear(0xFFFFFFFF, 0);

        let tick_info = TickInfo {
            parent_pos: Position::default(),
            mouse_pos,
            window_ref: w,
        };

        self.root.on_tick(tick_info);
    }
}
