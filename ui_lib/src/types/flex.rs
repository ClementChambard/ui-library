#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexFit {
    Tight,
    Loose,
}

impl From<i32> for FlexFit {
    fn from(value: i32) -> Self {
        match value {
            0 => FlexFit::Tight,
            1 => FlexFit::Loose,
            _ => panic!("unknown FlexFit value {value}"),
        }
    }
}

impl From<FlexFit> for i32 {
    fn from(value: FlexFit) -> Self {
        match value {
            FlexFit::Tight => 0,
            FlexFit::Loose => 1,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MainAxisSize {
    Min,
    Max,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MainAxisAlignment {
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl MainAxisAlignment {
    pub fn distribute_free_space(&self, free: f32, n: usize) -> (f32, f32, f32) {
        match self {
            Self::Start => (0.0, 0.0, free),
            Self::End => (free, 0.0, 0.0),
            Self::Center => (free / 2.0, 0.0, free / 2.0),
            Self::SpaceBetween => (0.0, free / (n - 1) as f32, 0.0),
            Self::SpaceAround => (
                free / (2.0 * n as f32),
                free / n as f32,
                free / (2.0 * n as f32),
            ),
            Self::SpaceEvenly => (
                free / (n + 1) as f32,
                free / (n + 1) as f32,
                free / (n + 1) as f32,
            ),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CrossAxisAlignment {
    Start,
    End,
    Center,
    Stretch,
    Baseline,
}

impl CrossAxisAlignment {
    pub fn get_cross_offset(&self, s_max: f32, s: f32) -> f32 {
        match self {
            Self::Stretch | Self::Start => 0.0,
            Self::End => s_max - s,
            Self::Center => (s_max - s) / 2.0,
            Self::Baseline => unimplemented!(),
        }
    }
}
