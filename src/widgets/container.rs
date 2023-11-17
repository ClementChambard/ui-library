use super::INode;
use crate::{
    rendering::AppBuffer,
    types::{Constraints, Insets, Position, Size, SizeFlex},
};

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum JustifyContent {
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceEvenly,
    SpaceAround,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum AlignItems {
    Start,
    End,
    Center,
}

impl AlignItems {
    fn get_v(&self, space: f32, v: f32) -> f32 {
        match self {
            Self::Start => 0.,
            Self::End => space - v,
            Self::Center => (space - v) / 2.,
        }
    }
}

impl JustifyContent {
    fn free_space(&self, space: f32, n_child: usize) -> (f32, f32) {
        match self {
            Self::Start => (0., 0.),
            Self::End => (space, 0.),
            Self::Center => (space / 2., 0.),
            Self::SpaceBetween => (0., space / (n_child - 1) as f32),
            Self::SpaceEvenly => (space / (n_child + 1) as f32, space / (n_child + 1) as f32),
            Self::SpaceAround => {
                let around = space / (2. * n_child as f32);
                (around, 2. * around)
            }
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum FlexDirection {
    Row,
    Column,
}

impl FlexDirection {
    fn axis_from_pos(&self, x: f32, y: f32) -> (f32, f32) {
        // main axis, cross axis
        match self {
            Self::Row => (x, y),
            Self::Column => (y, x),
        }
    }
    fn pos_from_axis(&self, main: f32, cross: f32) -> (f32, f32) {
        // x, y
        match self {
            Self::Row => (main, cross),
            Self::Column => (cross, main),
        }
    }
    fn constraint(&self, avail: f32, c: &Constraints) -> Constraints {
        match self {
            Self::Row => Constraints {
                w_min: 0.,
                w_max: avail,
                h_min: 0.,
                h_max: c.h_max,
            },
            Self::Column => Constraints {
                w_min: 0.,
                w_max: c.w_max,
                h_min: 0.,
                h_max: avail,
            },
        }
    }
}

pub struct Container {
    children: Vec<Box<dyn INode>>,
    pos: Position,
    padding: Insets,
    size: Size,
    justify_content: JustifyContent,
    align_items: AlignItems,
    direction: FlexDirection,
    color: u32,
    zindex: usize,
}

impl Container {
    pub fn new(direction: FlexDirection, children: Vec<Box<dyn INode>>) -> Self {
        Self {
            children,
            pos: Position::default(),
            padding: Insets::default(),
            size: Size::default(),
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Start,
            color: 0xFFFFFF,
            direction,
            zindex: 0,
        }
    }

    pub fn boxx(self) -> Box<Self> {
        Box::new(self)
    }

    pub fn color(mut self, c: u32) -> Self {
        self.color = c;
        self
    }

    pub fn padding(mut self, p: Insets) -> Self {
        self.padding = p;
        self
    }

    pub fn padding_all(mut self, p: f32) -> Self {
        self.padding.right = p;
        self.padding.left = p;
        self.padding.top = p;
        self.padding.bottom = p;
        self
    }

    pub fn padding_horizontal(mut self, p: f32) -> Self {
        self.padding.right = p;
        self.padding.left = p;
        self
    }

    pub fn padding_vertical(mut self, p: f32) -> Self {
        self.padding.top = p;
        self.padding.bottom = p;
        self
    }
    pub fn padding_left(mut self, p: f32) -> Self {
        self.padding.left = p;
        self
    }

    pub fn padding_top(mut self, p: f32) -> Self {
        self.padding.top = p;
        self
    }

    pub fn padding_right(mut self, p: f32) -> Self {
        self.padding.right = p;
        self
    }

    pub fn padding_bottom(mut self, p: f32) -> Self {
        self.padding.bottom = p;
        self
    }

    pub fn align_items(mut self, a: AlignItems) -> Self {
        self.align_items = a;
        self
    }

    pub fn justify_content(mut self, j: JustifyContent) -> Self {
        self.justify_content = j;
        self
    }

    pub fn zindex(mut self, i: usize) -> Self {
        self.set_zindex(i);
        self
    }
}

impl INode for Container {
    fn give_constraints(&mut self, c: Constraints) -> SizeFlex {
        let cns = c.reduce(
            self.padding.left + self.padding.right,
            self.padding.top + self.padding.bottom,
        );
        let (max_main, max_cross) = self.direction.axis_from_pos(cns.w_max, cns.h_max);
        let mut avail_main = max_main;
        let mut used_main = 0.;
        let mut sized_children = Vec::new();
        let mut row_cross = 0.;
        let mut total_flex = 0.;
        let n_children = self.children.len();
        for c in &mut self.children {
            let SizeFlex {
                w,
                h,
                flex_x,
                flex_y,
            } = c.give_constraints(self.direction.constraint(avail_main, &cns));
            let (mut main, mut cross) = self.direction.axis_from_pos(w, h);
            let (flex_main, flex_cross) =
                self.direction.axis_from_pos(flex_x as f32, flex_y as f32);
            if flex_cross > 0. {
                cross = max_cross;
            }
            if flex_main > 0. {
                total_flex += flex_main;
                main = 0.;
            }
            sized_children.push((c, main, cross, flex_main));
            row_cross = if row_cross < cross { cross } else { row_cross };
            avail_main -= main;
            used_main += main;
        }
        let (min_main, min_cross) = self.direction.axis_from_pos(cns.w_min, cns.h_min);
        if row_cross < min_cross {
            row_cross = min_cross;
        }

        let row_main = if total_flex > 0. {
            max_main
        } else if used_main < min_main {
            min_main
        } else {
            used_main
        };
        let (start, between) = self
            .justify_content
            .free_space(row_main - used_main, n_children);

        let (start, between, flexible) = if total_flex > 0. {
            (0., 0., max_main - used_main)
        } else {
            (start, between, 0.)
        };
        let (pad_main, pad_cross) = self
            .direction
            .axis_from_pos(self.padding.left, self.padding.top);
        let mut cur_main = start + pad_main;
        for (c, main, cr, flex_main) in sized_children {
            let cross = pad_cross + self.align_items.get_v(row_cross, cr);
            let main = if flex_main > 0. {
                (flex_main / total_flex) * flexible
            } else {
                main
            };
            if flex_main > 0. {
                let s = c.get_size();
                let (_, cross_) = self.direction.axis_from_pos(s.w, s.h);
                let (w, h) = self.direction.pos_from_axis(main, cross_);
                c.set_size(Size { w, h });
            }
            let (x, y) = self.direction.pos_from_axis(cur_main, cross);
            c.set_position(Position { x, y });
            cur_main += between + main;
        }
        let (row_w, row_h) = self.direction.pos_from_axis(row_main, row_cross);
        self.size = Size {
            w: row_w + self.padding.left + self.padding.right,
            h: row_h + self.padding.top + self.padding.bottom,
        };
        self.size.into()
    }
    fn on_draw(&self, p: &Position, buffer: &mut AppBuffer) {
        let p = Position {
            x: p.x + self.pos.x,
            y: p.y + self.pos.y,
        };
        buffer.draw_rectangle(
            p.x as isize,
            p.y as isize,
            self.size.w as usize,
            self.size.h as usize,
            self.color,
            self.zindex,
        );
        for c in &self.children {
            c.on_draw(&p, buffer);
        }
    }
    fn on_tick(&mut self, info: super::app::TickInfo) {
        let info = super::app::TickInfo {
            parent_pos: Position {
                x: info.parent_pos.x + self.pos.x,
                y: info.parent_pos.y + self.pos.y,
            },
            ..info
        };

        for c in &mut self.children {
            c.on_tick(info.clone());
        }
    }

    fn set_position(&mut self, p: Position) {
        self.pos = p;
    }

    fn get_position(&self) -> Position {
        self.pos
    }

    fn set_size(&mut self, s: Size) {
        self.size = s;
        self.give_constraints(s.constrtaints());
    }

    fn get_size(&self) -> Size {
        self.size
    }

    fn set_zindex(&mut self, i: usize) {
        self.zindex = i;
        for c in &mut self.children {
            c.set_zindex(i);
        }
    }
    fn get_zindex(&self) -> usize {
        self.zindex
    }
}
