use super::Widget;
use crate::types::FlexFit;

pub struct Flexible {
    child: Box<dyn Widget>,
    flex: i32,
    fit: FlexFit,
}

impl Flexible {
    pub fn new(child: Box<dyn Widget>, fit: FlexFit) -> Self {
        Self {
            child,
            flex: 1,
            fit,
        }
    }

    pub fn flex(mut self, flex: i32) -> Self {
        self.flex = flex;
        self
    }

    pub fn b(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Widget for Flexible {
    fn create_render_object(&self) -> Box<dyn crate::rendering::render_object::RenderObject> {
        let mut ro = self.child.create_render_object();
        ro.set_prop_i32("flex", self.flex);
        ro.set_prop_i32("fit", self.fit.into());
        ro
    }
}

pub struct Expanded;

impl Expanded {
    pub fn new(child: Box<dyn Widget>) -> Flexible {
        Flexible::new(child, FlexFit::Tight)
    }
}
