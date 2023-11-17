mod rendering;
mod types;
mod widgets;

use rendering::AppBuffer;
use types::Size;
use widgets::*;

use minifb::{Key, Scale, Window, WindowOptions};

struct Draggable {
    me: AbsoluteWrapper,
    was_pressing: bool,
    dragging: bool,
    start_drag_pos: (usize, usize),
}

impl Draggable {
    pub fn new() -> Self {
        Self {
            me: AbsoluteWrapper::new(Blob::new(200., 200.).color(0xFF00FF).boxx())
                .zindex(100)
                .absolute(),
            dragging: false,
            was_pressing: false,
            start_drag_pos: (0, 0),
        }
    }

    pub fn boxx(self) -> Box<Self> {
        Box::new(self)
    }
}

impl INode for Draggable {
    fn give_constraints(&mut self, c: types::Constraints) -> types::SizeFlex {
        self.me.give_constraints(c)
    }

    fn on_draw(&self, p: &types::Position, buffer: &mut AppBuffer) {
        self.me.on_draw(p, buffer);
    }

    fn on_tick(&mut self, info: app::TickInfo) {
        let x = if self.me.absolute {
            self.me.pos.x as usize
        } else {
            (info.parent_pos.x + self.me.pos.x) as usize
        };
        let y = if self.me.absolute {
            self.me.pos.y as usize
        } else {
            (info.parent_pos.y + self.me.pos.y) as usize
        };

        if let Some((mx, my, mz)) = info.mouse_pos {
            if info.window_ref.get_mouse_down(minifb::MouseButton::Left) {
                if self.dragging {
                    let new_x =
                        self.me.pos.x as isize + mx as isize - self.start_drag_pos.0 as isize;
                    let new_y =
                        self.me.pos.y as isize + my as isize - self.start_drag_pos.1 as isize;
                    self.me.pos.x = new_x as f32;
                    self.me.pos.y = new_y as f32;
                    self.start_drag_pos.0 = mx;
                    self.start_drag_pos.1 = my;
                } else if !self.was_pressing
                    && mx > x
                    && mx < x + 200
                    && my > y
                    && my < y + 200
                    && mz <= self.me.get_zindex()
                {
                    self.dragging = true;
                    self.start_drag_pos.0 = mx;
                    self.start_drag_pos.1 = my;
                }
                self.was_pressing = true;
            } else {
                self.was_pressing = false;
                self.dragging = false;
            }
        } else {
            self.was_pressing = false;
            self.dragging = false;
        }
        self.me.on_tick(info);
    }

    fn set_position(&mut self, p: types::Position) {
        self.me.set_position(p);
    }

    fn get_position(&self) -> types::Position {
        self.me.get_position()
    }

    fn set_size(&mut self, s: Size) {
        self.me.set_size(s);
    }

    fn get_size(&self) -> Size {
        self.me.get_size()
    }

    fn set_zindex(&mut self, i: usize) {
        self.me.set_zindex(i);
    }

    fn get_zindex(&self) -> usize {
        self.me.get_zindex()
    }
}

struct CustomWidget {
    me: ConstrainedBox,
    // other fields
    example_other_field: i32,
}

impl INode for CustomWidget {
    fn give_constraints(&mut self, c: types::Constraints) -> types::SizeFlex {
        self.me.give_constraints(c)
    }
    fn on_draw(&self, p: &types::Position, buffer: &mut AppBuffer) {
        self.me.on_draw(p, buffer);
    }
    fn on_tick(&mut self, _: app::TickInfo) {}
    fn set_position(&mut self, p: types::Position) {
        self.me.set_position(p);
    }
    fn get_position(&self) -> types::Position {
        self.me.get_position()
    }
    fn set_size(&mut self, s: Size) {
        self.me.set_size(s);
    }
    fn get_size(&self) -> Size {
        self.me.get_size()
    }
    fn set_zindex(&mut self, i: usize) {
        self.me.set_zindex(i);
    }
    fn get_zindex(&self) -> usize {
        self.me.get_zindex()
    }
}

impl CustomWidget {
    fn new(example_other_field: i32) -> Self {
        Self {
            me: ConstrainedBox::new(
                Container::new(
                    FlexDirection::Row,
                    vec![
                        Blob::new(40., 40.).color(0xFF0000).boxx(),
                        ConstrainedBox::new(
                            Container::new(
                                FlexDirection::Column,
                                vec![
                                    Blob::new(100., 15.).color(0xFF0000).boxx(),
                                    Blob::new(90., 15.).color(0xFF0000).boxx(),
                                ],
                            )
                            .justify_content(JustifyContent::SpaceAround)
                            .align_items(AlignItems::Center)
                            .boxx(),
                        )
                        .height(40.)
                        .boxx(),
                    ],
                )
                .align_items(AlignItems::Center)
                .justify_content(JustifyContent::SpaceBetween)
                .boxx(),
            )
            .min_width(150.),
            example_other_field,
        }
    }

    fn boxx(self) -> Box<Self> {
        Box::new(self)
    }
}

fn main() {
    let width = 800;
    let height = 600;
    let mut window = Window::new(
        "Rust UI Prototype",
        width,
        height,
        WindowOptions {
            resize: true,
            scale: Scale::X1, // Set the initial scale factor
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create window");
    let mut app = App::new(
        width as f32,
        height as f32,
        Align::new(
            Alignment::BottomRight,
            Container::new(
                FlexDirection::Column,
                vec![
                    Draggable::new().boxx(),
                    AbsoluteWrapper::new(Blob::new(200., 200.).color(0xFF88FF).boxx())
                        .x(140.)
                        .y(40.)
                        .absolute()
                        .zindex(10)
                        .boxx(),
                    ConstrainedBox::new(Blob::new(300., 20.).color(0xFF0000).boxx())
                        .flex_y(2)
                        .boxx(),
                    ConstrainedBox::new(Blob::new(140., 30.).color(0x00FF00).boxx())
                        .flex_y(1)
                        .boxx(),
                    Blob::new(500., 50.).color(0x0000FF).boxx(),
                    Blob::new(400., 100.).color(0xFFFF00).boxx(),
                    Blob::new(250., 20.).color(0x00FFFF).boxx(),
                    Button::new(200., 50.)
                        .callback(ButtonCallbackType::Pressed, || println!("pressed"))
                        .boxx(),
                    CustomWidget::new(123).boxx(),
                ],
            )
            .color(0xCCCCCC)
            .align_items(AlignItems::Center)
            .padding_vertical(5.)
            .padding_left(20.)
            .boxx(),
        )
        .boxx(),
    );

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        app.on_tick(&window);
        app.on_draw();

        // Update the window with the buffer data
        window
            .update_with_buffer(&app.buffer.color, app.size.w as usize, app.size.h as usize)
            .expect("Unable to update window");
    }
}
