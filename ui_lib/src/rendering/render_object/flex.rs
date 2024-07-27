use std::collections::HashMap;

use crate::types::{
    Axis, BoxConstraints, CrossAxisAlignment, FlexFit, MainAxisAlignment, MainAxisSize, Position,
    Size, TextDirection, VerticalDirection,
};

use super::{RenderObject, RenderObjectProp};

pub struct RenderFlex {
    children: Vec<Box<dyn RenderObject>>,
    direction: Axis,
    main_axis_size: MainAxisSize,
    main_axis_alignment: MainAxisAlignment,
    cross_axis_alignment: CrossAxisAlignment,
    text_direction: TextDirection,
    vertical_direction: VerticalDirection,
    text_baseline: Option<()>,
    clip_behavior: Option<()>,

    props: HashMap<String, RenderObjectProp>,
    render_pos: Position,
}

struct LayoutSizes {
    main_size: f32,
    cross_size: f32,
}

fn spread_positions(
    size: f32,
    sizes: &[f32],
    before: f32,
    between: f32,
    backwards: bool,
) -> Vec<f32> {
    let mut positions = Vec::new();
    if !backwards {
        let mut pos_here = before;
        for (i, s) in sizes.iter().enumerate() {
            if i != 0 {
                pos_here += between;
            }
            positions.push(pos_here);
            pos_here += s;
        }
    } else {
        let mut pos_here = size - before;
        for (i, s) in sizes.iter().enumerate() {
            pos_here -= s;
            if i != 0 {
                pos_here -= between;
            }
            positions.push(pos_here);
        }
    }

    positions
}

fn get_vs(
    size: f32,
    sizes: &[f32],
    cross_axis_alignment: CrossAxisAlignment,
    backwards: bool,
) -> Vec<f32> {
    sizes
        .iter()
        .map(|s| {
            let s = cross_axis_alignment.get_cross_offset(size, *s);
            if backwards {
                size - s
            } else {
                s
            }
        })
        .collect()
}

impl RenderFlex {
    pub fn new(
        children: Vec<Box<dyn RenderObject>>,
        direction: Axis,
        main_axis_size: MainAxisSize,
        main_axis_alignment: MainAxisAlignment,
        cross_axis_alignment: CrossAxisAlignment,
        text_direction: TextDirection,
        vertical_direction: VerticalDirection,
        text_baseline: Option<()>,
        clip_behavior: Option<()>,
    ) -> Self {
        Self {
            children,
            direction,
            main_axis_size,
            main_axis_alignment,
            cross_axis_alignment,
            text_direction,
            vertical_direction,
            text_baseline,
            clip_behavior,
            props: HashMap::new(),
            render_pos: Position::default(),
        }
    }

    fn get_flex(&self, child: &dyn RenderObject) -> i32 {
        child.get_prop_i32("flex").unwrap_or(0)
    }

    fn get_fit(&self, child: &dyn RenderObject) -> FlexFit {
        child.get_prop_i32("fit").unwrap_or(0).into()
    }

    fn get_cross_size(&self, size: Size) -> f32 {
        match self.direction {
            Axis::Horizontal => size.h,
            Axis::Vertical => size.w,
        }
    }

    fn get_main_size(&self, size: Size) -> f32 {
        match self.direction {
            Axis::Horizontal => size.w,
            Axis::Vertical => size.h,
        }
    }

    fn can_compute_intrinsics(&self) -> bool {
        self.cross_axis_alignment != CrossAxisAlignment::Baseline
    }

    fn position_children(&mut self, sizes: &[Size], main_size: f32, cross_size: f32) {
        let mut free_space = 0f32;
        for s in sizes {
            free_space += match self.direction {
                Axis::Horizontal => s.w,
                Axis::Vertical => s.h,
            }
        }
        let (before, between, _) = self
            .main_axis_alignment
            .distribute_free_space(main_size - free_space, sizes.len());
        let backwards = (self.direction == Axis::Horizontal
            && self.text_direction == TextDirection::Rtl)
            || (self.direction == Axis::Vertical
                && self.vertical_direction == VerticalDirection::Up);
        let cross_backwards = (self.direction == Axis::Horizontal
            && self.vertical_direction == VerticalDirection::Up)
            || (self.direction == Axis::Vertical && self.text_direction == TextDirection::Rtl);

        let main_sizes: Vec<f32> = sizes
            .iter()
            .map(|s| match self.direction {
                Axis::Horizontal => s.w,
                Axis::Vertical => s.h,
            })
            .collect();
        let cross_sizes: Vec<f32> = sizes
            .iter()
            .map(|s| match self.direction {
                Axis::Horizontal => s.h,
                Axis::Vertical => s.w,
            })
            .collect();
        let pos_mains = spread_positions(main_size, &main_sizes, before, between, backwards);
        let pos_cross = get_vs(
            cross_size,
            &cross_sizes,
            self.cross_axis_alignment,
            cross_backwards,
        );
        let positions: Vec<(f32, f32)> = pos_mains
            .into_iter()
            .zip(pos_cross.into_iter())
            .map(|(m, c)| match self.direction {
                Axis::Horizontal => (m, c),
                Axis::Vertical => (c, m),
            })
            .collect();
        for (i, c) in self.children.iter_mut().enumerate() {
            let p = Position {
                x: positions[i].0,
                y: positions[i].1,
            };
            c.set_render_pos(p);
        }
    }

    fn compute_sizes(&mut self, constraints: BoxConstraints) -> (LayoutSizes, Vec<Size>) {
        let mut total_flex = 0;
        let max_main_size = match self.direction {
            Axis::Horizontal => constraints.max_width,
            Axis::Vertical => constraints.max_height,
        };
        let can_flex = max_main_size < f32::INFINITY;
        let mut cross_size = 0f32;
        let mut allocated_size = 0f32;
        let mut sizes = Vec::new();
        let mut last_flex_child_id = -1;
        for (i, c) in self.children.iter_mut().enumerate() {
            let flex = c.get_prop_i32("flex").unwrap_or(0);
            if flex > 0 {
                sizes.push(Size { w: 0.0, h: 0.0 });
                total_flex += flex;
                last_flex_child_id = i as isize;
            } else {
                let inner_constraints = match self.cross_axis_alignment {
                    CrossAxisAlignment::Stretch => match self.direction {
                        Axis::Horizontal => {
                            BoxConstraints::tight_for_height(constraints.max_height)
                        }
                        Axis::Vertical => BoxConstraints::tight_for_width(constraints.max_width),
                    },
                    _ => match self.direction {
                        Axis::Horizontal => BoxConstraints {
                            max_height: constraints.max_height,
                            min_height: 0.0,
                            min_width: 0.0,
                            max_width: f32::INFINITY,
                        },
                        Axis::Vertical => BoxConstraints {
                            max_width: constraints.max_width,
                            min_height: 0.0,
                            min_width: 0.0,
                            max_height: f32::INFINITY,
                        },
                    },
                };
                let child_size = c.calculate_layout(inner_constraints);
                sizes.push(child_size);
                let (m_size, c_size) = match self.direction {
                    Axis::Horizontal => (child_size.w, child_size.h),
                    Axis::Vertical => (child_size.h, child_size.w),
                };
                allocated_size += m_size;
                cross_size = cross_size.max(c_size);
            }
        }

        let free_space = 0f32.max(if can_flex { max_main_size } else { 0.0 } - allocated_size);
        let mut allocated_flex_space = 0f32;

        let space_per_flex = if can_flex && total_flex > 0 {
            free_space / (total_flex as f32)
        } else {
            f32::NAN
        };
        for (i, c) in self.children.iter_mut().enumerate() {
            let flex = c.get_prop_i32("flex").unwrap_or(0);
            if flex > 0 {
                let max_child_extent = if can_flex {
                    if i as isize == last_flex_child_id {
                        free_space - allocated_flex_space
                    } else {
                        space_per_flex * flex as f32
                    }
                } else {
                    f32::INFINITY
                };
                let min_child_extent = match c.get_prop_i32("fit").unwrap_or(0).into() {
                    FlexFit::Tight => {
                        assert!(max_child_extent < f32::INFINITY);
                        max_child_extent
                    }
                    FlexFit::Loose => 0.0,
                };
                let inner_constraints = match self.cross_axis_alignment {
                    CrossAxisAlignment::Stretch => match self.direction {
                        Axis::Horizontal => BoxConstraints {
                            min_width: min_child_extent,
                            max_width: max_child_extent,
                            min_height: constraints.max_height,
                            max_height: constraints.max_height,
                        },
                        Axis::Vertical => BoxConstraints {
                            min_height: min_child_extent,
                            max_height: max_child_extent,
                            min_width: constraints.max_width,
                            max_width: constraints.max_width,
                        },
                    },
                    _ => match self.direction {
                        Axis::Horizontal => BoxConstraints {
                            min_width: min_child_extent,
                            max_width: max_child_extent,
                            min_height: 0.0,
                            max_height: constraints.max_height,
                        },
                        Axis::Vertical => BoxConstraints {
                            min_height: min_child_extent,
                            max_height: max_child_extent,
                            min_width: 0.0,
                            max_width: constraints.max_width,
                        },
                    },
                };
                let child_size = c.calculate_layout(inner_constraints);
                sizes[i] = child_size;
                let (m_size, c_size) = match self.direction {
                    Axis::Horizontal => (child_size.w, child_size.h),
                    Axis::Vertical => (child_size.h, child_size.w),
                };
                assert!(m_size <= max_child_extent);
                allocated_size += m_size;
                allocated_flex_space += max_child_extent;
                cross_size = cross_size.max(c_size);
            }
        }

        let ideal_size = if can_flex && self.main_axis_size == MainAxisSize::Max {
            max_main_size
        } else {
            allocated_size
        };

        return (
            LayoutSizes {
                main_size: ideal_size,
                cross_size,
            },
            sizes,
        );
    }
}

impl RenderObject for RenderFlex {
    fn set_prop(&mut self, prop: &str, val: RenderObjectProp) {
        self.props.insert(prop.to_string(), val);
    }

    fn get_prop(&self, prop: &str) -> Option<&RenderObjectProp> {
        self.props.get(prop)
    }

    fn set_render_pos(&mut self, render_pos: Position) {
        self.render_pos = render_pos;
    }

    fn get_render_pos(&self) -> Position {
        self.render_pos
    }

    fn render(
        &self,
        context: &mut crate::rendering::RenderContext,
        context_mut: crate::rendering::RenderContextMut,
    ) {
        for c in &self.children {
            c.render(context, context_mut.update(self));
        }
    }

    fn calculate_layout(
        &mut self,
        constraints: crate::types::BoxConstraints,
    ) -> crate::types::Size {
        if !self.can_compute_intrinsics() {
            return Size::default();
        }

        let (size, sizes) = self.compute_sizes(constraints.clone());
        let size = match self.direction {
            Axis::Horizontal => constraints.constrain(Size {
                w: size.main_size,
                h: size.cross_size,
            }),
            Axis::Vertical => constraints.constrain(Size {
                w: size.cross_size,
                h: size.main_size,
            }),
        };

        let (main, cross) = match self.direction {
            Axis::Horizontal => (size.w, size.h),
            Axis::Vertical => (size.h, size.w),
        };

        self.position_children(&sizes, main, cross);

        size
    }
}
