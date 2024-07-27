use crate::{
    rendering::render_object::RenderFlex,
    types::{
        Axis, CrossAxisAlignment, MainAxisAlignment, MainAxisSize, TextDirection, VerticalDirection,
    },
};

use super::Widget;

pub struct Flex {
    direction: Axis,
    main_axis_alignment: MainAxisAlignment,
    main_axis_size: MainAxisSize,
    cross_axis_alignment: CrossAxisAlignment,
    text_direction: TextDirection,
    vertical_direction: VerticalDirection,
    text_baseline: Option<()>,
    clip_behavior: Option<()>,
    children: Vec<Box<dyn Widget>>,
}

impl Flex {
    pub fn new(children: Vec<Box<dyn Widget>>, direction: Axis) -> Self {
        Self {
            direction,
            children,
            main_axis_alignment: MainAxisAlignment::Start,
            main_axis_size: MainAxisSize::Max,
            cross_axis_alignment: CrossAxisAlignment::Center,
            text_direction: TextDirection::Ltr,
            vertical_direction: VerticalDirection::Down,
            text_baseline: None,
            clip_behavior: None,
        }
    }

    pub fn justify_content(mut self, a: MainAxisAlignment) -> Self {
        self.main_axis_alignment = a;
        self
    }

    pub fn align_items(mut self, a: CrossAxisAlignment) -> Self {
        self.cross_axis_alignment = a;
        self
    }

    pub fn main_axis_size(mut self, s: MainAxisSize) -> Self {
        self.main_axis_size = s;
        self
    }

    pub fn text_direction(mut self, d: TextDirection) -> Self {
        self.text_direction = d;
        self
    }

    pub fn vertical_direction(mut self, d: VerticalDirection) -> Self {
        self.vertical_direction = d;
        self
    }

    pub fn b(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Widget for Flex {
    fn create_render_object(&self) -> Box<dyn crate::rendering::render_object::RenderObject> {
        Box::new(RenderFlex::new(
            self.children
                .iter()
                .map(|c| c.create_render_object())
                .collect(),
            self.direction,
            self.main_axis_size,
            self.main_axis_alignment,
            self.cross_axis_alignment,
            self.text_direction,
            self.vertical_direction,
            self.text_baseline,
            self.clip_behavior,
        ))
    }
}

pub struct Column;
impl Column {
    pub fn new(c: Vec<Box<dyn Widget>>) -> Flex {
        Flex::new(c, Axis::Vertical)
    }
}

pub struct Row;
impl Row {
    pub fn new(c: Vec<Box<dyn Widget>>) -> Flex {
        Flex::new(c, Axis::Horizontal)
    }
}
