use crate::{
    rendering::render_object::{ConstrainedRenderObject, LimitedRenderObject},
    types::{BoxConstraints, Size},
};

use super::Widget;

pub struct ConstrainedBox {
    child: Option<Box<dyn Widget>>,
    constraints: BoxConstraints,
}

impl ConstrainedBox {
    pub fn new(child: Option<Box<dyn Widget>>, constraints: BoxConstraints) -> Self {
        Self { child, constraints }
    }

    pub fn b(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Widget for ConstrainedBox {
    fn create_render_object(&self) -> Box<dyn crate::rendering::render_object::RenderObject> {
        Box::new(ConstrainedRenderObject::new(
            match self.child {
                Some(ref c) => Some(c.create_render_object()),
                None => None,
            },
            self.constraints.clone(),
        ))
    }
}

pub struct LimitedBox {
    child: Option<Box<dyn Widget>>,
    max_width: f32,
    max_height: f32,
}

impl LimitedBox {
    pub fn new(child: Option<Box<dyn Widget>>, max_width: f32, max_height: f32) -> Self {
        Self {
            child,
            max_width,
            max_height,
        }
    }

    pub fn b(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Widget for LimitedBox {
    fn create_render_object(&self) -> Box<dyn crate::rendering::render_object::RenderObject> {
        Box::new(LimitedRenderObject::new(
            match self.child {
                Some(ref c) => Some(c.create_render_object()),
                None => None,
            },
            self.max_width,
            self.max_height,
        ))
    }
}

pub struct SizedBox {
    child: Option<Box<dyn Widget>>,
    width: f32,
    height: f32,
}

impl SizedBox {
    pub fn new(child: Option<Box<dyn Widget>>, width: f32, height: f32) -> Self {
        Self {
            child,
            width,
            height,
        }
    }

    pub fn expand(child: Option<Box<dyn Widget>>) -> Self {
        Self {
            child,
            width: f32::INFINITY,
            height: f32::INFINITY,
        }
    }

    pub fn shrink(child: Option<Box<dyn Widget>>) -> Self {
        Self {
            child,
            width: 0.0,
            height: 0.0,
        }
    }

    pub fn from_size(child: Option<Box<dyn Widget>>, size: Size) -> Self {
        Self {
            child,
            width: size.w,
            height: size.h,
        }
    }

    pub fn square(child: Option<Box<dyn Widget>>, dim: f32) -> Self {
        Self {
            child,
            width: dim,
            height: dim,
        }
    }

    pub fn b(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Widget for SizedBox {
    fn create_render_object(&self) -> Box<dyn crate::rendering::render_object::RenderObject> {
        Box::new(ConstrainedRenderObject::new(
            match self.child {
                Some(ref c) => Some(c.create_render_object()),
                None => None,
            },
            BoxConstraints::tight_for(self.width, self.height),
        ))
    }
}
