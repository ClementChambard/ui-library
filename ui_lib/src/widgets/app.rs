use minifb::{Scale, Window, WindowOptions};

use crate::{
    inputs::MouseStatus,
    rendering::{render_object::RenderObject, AppBuffer, RenderContext, RenderContextMut},
    types::{BoxConstraints, Position, Size},
};

use super::Widget;

pub struct App {
    pub size: Size,
    root: Box<dyn Widget>,
    render_root: Box<dyn RenderObject>,
    pub buffer: AppBuffer,
    mouse: MouseStatus,
    window: Window,
}

#[derive(Clone)]
pub struct UpdateContext<'a> {
    mouse_status: &'a MouseStatus,
}

impl App {
    pub fn new(name: &str, w: f32, h: f32, root: Box<dyn Widget>) -> Self {
        let root = root;
        let mut render_root = root.create_render_object();
        render_root.calculate_layout(BoxConstraints::tight_for(w, h));
        // Create a buffer to store pixel data (RGB)

        let window = Window::new(
            name,
            w as usize,
            h as usize,
            WindowOptions {
                resize: true,
                scale: Scale::X1, // Set the initial scale factor
                ..WindowOptions::default()
            },
        )
        .expect("Unable to create window");

        Self {
            size: Size { w, h },
            root,
            window,
            render_root,
            buffer: AppBuffer::new(w as usize, h as usize),
            mouse: MouseStatus::now(None, false, false, false, 0.0),
        }
    }
    fn on_draw(&mut self) {
        self.render_root.render(
            &mut RenderContext {
                appbuffer: &mut self.buffer,
            },
            RenderContextMut {
                position: Position::default(),
                elevation: 0,
            },
        );
    }

    pub fn run(&mut self) {
        while self.window.is_open() {
            self.on_tick();
            self.on_draw();

            // Update the window with the buffer data
            self.window
                .update_with_buffer(
                    &self.buffer.color,
                    self.size.w as usize,
                    self.size.h as usize,
                )
                .expect("Unable to update window");
        }
    }

    fn on_tick(&mut self) {
        let w = &mut self.window;
        let (new_width, new_height) = w.get_size();
        if new_width != self.buffer.width || new_height != self.buffer.height {
            // Resize the buffer accordingly
            self.buffer.resize(new_width, new_height);
            self.size.w = new_width as f32;
            self.size.h = new_height as f32;

            self.render_root
                .calculate_layout(BoxConstraints::tight_for(self.size.w, self.size.h));
        }

        let mpos = if let Some((mx, my)) = w.get_mouse_pos(minifb::MouseMode::Clamp) {
            let (mx, my) = (mx as usize, my as usize);
            let mz = self.buffer.depth[mx + my * self.buffer.width];
            Some((mx, my, mz))
        } else {
            None
        };
        let mleft = w.get_mouse_down(minifb::MouseButton::Left);
        let mright = w.get_mouse_down(minifb::MouseButton::Right);
        let mmiddle = w.get_mouse_down(minifb::MouseButton::Middle);
        let mscroll = w.get_scroll_wheel().unwrap_or((0.0, 0.0)).1;
        self.mouse.update(mpos, mleft, mright, mmiddle, mscroll);

        self.buffer.clear(0xFFFFFFFF, 0);
    }
}
