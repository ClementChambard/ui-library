#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Alignment {
    TopLeft,
    TopMiddle,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomMiddle,
    BottomRight,
    Value(f32, f32),
}

impl Alignment {
    pub fn loc(&self) -> (f32, f32) {
        match self {
            Self::Center => (0.0, 0.0),
            Self::TopLeft => (-1.0, -1.0),
            Self::TopMiddle => (0.0, -1.0),
            Self::TopRight => (1.0, -1.0),
            Self::CenterLeft => (-1.0, 0.0),
            Self::CenterRight => (1.0, 0.0),
            Self::BottomLeft => (-1.0, 1.0),
            Self::BottomMiddle => (0.0, 1.0),
            Self::BottomRight => (1.0, 1.0),
            Self::Value(x, y) => (*x, *y),
        }
    }
}
