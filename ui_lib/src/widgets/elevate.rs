use super::Widget;

pub struct Elevate {
    child: Box<dyn Widget>,
    elevation: usize,
}

impl Elevate {
    pub fn new(c: Box<dyn Widget>, e: usize) -> Self {
        Self {
            child: c,
            elevation: e,
        }
    }

    pub fn b(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Widget for Elevate {
    fn create_render_object(&self) -> Box<dyn crate::rendering::render_object::RenderObject> {
        let mut ro = self.child.create_render_object();
        ro.set_prop_usize("elevation", self.elevation);
        ro
    }
}
