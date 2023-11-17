use super::container::*;
use super::INode;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Alignment {
    TopLeft,
    TopMiddle,
    TopRight,
    CenterLeft,
    CenterRight,
    BottomLeft,
    BottomMiddle,
    BottomRight,
}

pub struct Align;

impl Align {
    pub fn new(a: Alignment, c: Box<dyn INode>) -> Container {
        let (main, cross) = match a {
            Alignment::TopLeft => (JustifyContent::Start, AlignItems::Start),
            Alignment::TopMiddle => (JustifyContent::Center, AlignItems::Start),
            Alignment::TopRight => (JustifyContent::End, AlignItems::Start),
            Alignment::CenterLeft => (JustifyContent::Start, AlignItems::Center),
            Alignment::CenterRight => (JustifyContent::End, AlignItems::Center),
            Alignment::BottomLeft => (JustifyContent::Start, AlignItems::End),
            Alignment::BottomMiddle => (JustifyContent::Center, AlignItems::End),
            Alignment::BottomRight => (JustifyContent::End, AlignItems::End),
        };
        Container::new(FlexDirection::Row, vec![c])
            .align_items(cross)
            .justify_content(main)
    }
}
