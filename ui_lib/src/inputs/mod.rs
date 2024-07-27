use crate::types::{Position, Size};

#[derive(Clone, Copy)]
pub struct ButtonState {
    pub pressed: bool,
    pub held: bool,
    pub released: bool,
}

impl ButtonState {
    pub fn now(val: bool) -> Self {
        Self {
            pressed: false,
            held: val,
            released: false,
        }
    }

    pub fn update(&self, val: bool) -> Self {
        Self {
            pressed: !self.held && val,
            held: val,
            released: self.held && !val,
        }
    }
}

#[derive(Clone)]
pub struct MouseStatus {
    pub pos: Option<(usize, usize, usize)>,
    pub motion: Option<(usize, usize)>,
    pub lclick: ButtonState,
    pub rclick: ButtonState,
    pub mclick: ButtonState,
    pub wheel: f32,
}

impl MouseStatus {
    pub fn now(
        pos: Option<(usize, usize, usize)>,
        left: bool,
        right: bool,
        middle: bool,
        wheel: f32,
    ) -> Self {
        Self {
            pos,
            motion: None,
            lclick: ButtonState::now(left),
            rclick: ButtonState::now(right),
            mclick: ButtonState::now(middle),
            wheel,
        }
    }

    pub fn update(
        &self,
        pos: Option<(usize, usize, usize)>,
        left: bool,
        right: bool,
        middle: bool,
        wheel: f32,
    ) -> Self {
        let motion = if let Some((x, y, _)) = pos {
            if let Some((ox, oy, _)) = self.pos {
                Some((x - ox, y - oy))
            } else {
                None
            }
        } else {
            None
        };
        Self {
            pos,
            motion,
            lclick: self.lclick.update(left),
            rclick: self.rclick.update(right),
            mclick: self.mclick.update(middle),
            wheel,
        }
    }

    pub fn is_hovering(&self, pos: Position, size: Size, elevation: usize) -> bool {
        if let Some((x, y, z)) = self.pos {
            let (x, y) = (x as f32, y as f32);
            x >= pos.x && y >= pos.y && z <= elevation && x <= pos.x + size.w && y <= pos.y + size.h
        } else {
            false
        }
    }
}
