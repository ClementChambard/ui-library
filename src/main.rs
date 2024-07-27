use ui_lib::types::{Alignment, BoxConstraints, MainAxisAlignment, MainAxisSize};
use ui_lib::widgets::*;

struct CustomWidget {
    me: ConstrainedBox,
}

impl Widget for CustomWidget {
    fn create_render_object(&self) -> Box<dyn ui_lib::rendering::render_object::RenderObject> {
        self.me.create_render_object()
    }
}

impl CustomWidget {
    fn new() -> Self {
        Self {
            me: ConstrainedBox::new(
                Some(
                    Row::new(ui_lib::b_vec![
                        Blob::new(40., 40., 0xFF0000),
                        ConstrainedBox::new(
                            Some(
                                Column::new(ui_lib::b_vec![
                                    Blob::new(100., 15., 0xFF0000),
                                    Blob::new(90., 15., 0xFF0000),
                                ])
                                .justify_content(MainAxisAlignment::SpaceAround)
                                .b(),
                            ),
                            BoxConstraints::tight_for_height(40.),
                        ),
                    ])
                    .main_axis_size(MainAxisSize::Min)
                    .justify_content(MainAxisAlignment::SpaceBetween)
                    .b(),
                ),
                BoxConstraints {
                    min_width: 150.,
                    ..Default::default()
                },
            ),
        }
    }

    fn b(self) -> Box<Self> {
        Box::new(self)
    }
}

fn main() {
    App::new(
        "Rust UI Prototype",
        800.0,
        600.0,
        ui_lib::widgets![
            Align::new(
                Alignment::BottomRight,
                Column::new(ui_lib::b_vec![
                    Expanded::new(Blob::new(300., 20., 0xFF0000).b()).flex(2),
                    Expanded::new(Blob::new(140., 30., 0x00FF00).b()).flex(1),
                    Blob::new(500., 50., 0x0000FF),
                    Blob::new(400., 100., 0xFFFF00),
                    Blob::new(250., 20., 0x00FFFF),
                    //Button::new(200., 50.)
                    //    .callback(ButtonCallbackType::Pressed, || println!("pressed")),
                    Blob::new(200., 50., 0x000000),
                    CustomWidget::new(),
                ])
                .main_axis_size(MainAxisSize::Min)
                .b(),
            ),
            PositionBox::new(
                Elevate::new(Blob::new(200., 200., 0xFF88FF).b(), 10).b(),
                140.,
                40.,
            )
            .absolute(),
        ],
    )
    .run();
}
