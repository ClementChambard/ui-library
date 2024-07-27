#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TextDirection {
    Ltr,
    Rtl,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VerticalDirection {
    Up,
    Down,
}
