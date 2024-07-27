use crate::rendering::render_object::ListRenderObject;

use super::Widget;

pub struct WidgetList {
    children: Vec<Box<dyn Widget>>,
}

impl WidgetList {
    pub fn new(children: Vec<Box<dyn Widget>>) -> Self {
        Self { children }
    }
}

impl Widget for WidgetList {
    fn create_render_object(&self) -> Box<dyn crate::rendering::render_object::RenderObject> {
        Box::new(ListRenderObject::new(
            self.children
                .iter()
                .map(|c| c.create_render_object())
                .collect(),
        ))
    }
}

#[macro_export]
macro_rules! widgets {
    ($fst:expr $(,$widgets:expr)* $(,)?) => {
        Box::new(WidgetList::new(
            vec![$fst.b() $(,$widgets.b())*]
        ))
    };
    () => {
        Box::new(WidgetList::new(vec![]))
    };
}
