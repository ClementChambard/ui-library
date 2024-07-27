use super::{Expanded, SizedBox, Widget};

pub struct Spacer {
    flex: i32,
}

impl Spacer {
    pub fn new() -> Self {
        Self { flex: 1 }
    }

    pub fn flex(mut self, flex: i32) -> Self {
        self.flex = flex;
        self
    }

    pub fn b(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Widget for Spacer {
    fn create_render_object(&self) -> Box<dyn crate::rendering::render_object::RenderObject> {
        Expanded::new(SizedBox::shrink(None).b())
            .flex(self.flex)
            .create_render_object()
    }
}
