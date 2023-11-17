use super::container::*;
use super::INode;

pub struct Center;

impl Center {
    pub fn new(c: Box<dyn INode>) -> Container {
        Container::new(FlexDirection::Row, vec![c])
            .align_items(AlignItems::Center)
            .justify_content(JustifyContent::Center)
    }
}
