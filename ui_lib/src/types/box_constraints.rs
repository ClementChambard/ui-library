use std::fmt::Display;

use super::{Insets, Size};

#[derive(Debug, Clone, PartialEq)]
pub struct BoxConstraints {
    pub min_width: f32,
    pub max_width: f32,
    pub min_height: f32,
    pub max_height: f32,
}

impl Default for BoxConstraints {
    fn default() -> Self {
        Self {
            min_width: 0.0,
            max_width: f32::INFINITY,
            min_height: 0.0,
            max_height: f32::INFINITY,
        }
    }
}

impl BoxConstraints {
    pub fn tight(size: Size) -> Self {
        Self {
            min_width: size.w,
            max_width: size.w,
            min_height: size.h,
            max_height: size.h,
        }
    }

    pub fn tight_for(width: f32, height: f32) -> Self {
        // original: optional params
        Self {
            min_width: width,
            max_width: width,
            min_height: height,
            max_height: height,
        }
    }

    pub fn tight_for_width(width: f32) -> Self {
        // original: in tight_for
        Self {
            min_width: width,
            max_width: width,
            min_height: 0.0,
            max_height: f32::INFINITY,
        }
    }

    pub fn tight_for_height(height: f32) -> Self {
        // original: in tight_for
        Self {
            min_width: 0.0,
            max_width: f32::INFINITY,
            min_height: height,
            max_height: height,
        }
    }

    pub fn loose(size: Size) -> Self {
        Self {
            min_width: 0.0,
            max_width: size.w,
            min_height: 0.0,
            max_height: size.h,
        }
    }

    // expand: w or inf, h or inf

    pub fn deflate(&self, edges: Insets) -> Self {
        // TODO: edge-insets
        let horizontal = edges.left + edges.right;
        let vertical = edges.top + edges.bottom;
        let deflated_min_width = 0f32.max(self.min_width - horizontal);
        let deflated_min_height = 0f32.max(self.min_height - vertical);
        Self {
            min_width: deflated_min_width,
            max_width: deflated_min_width.max(self.max_width - horizontal),
            min_height: deflated_min_height,
            max_height: deflated_min_height.max(self.max_height - vertical),
        }
    }

    pub fn loosen(&self) -> Self {
        Self {
            min_width: 0.0,
            max_width: self.max_width,
            min_height: 0.0,
            max_height: self.max_height,
        }
    }

    pub fn enforce(&self, other: &BoxConstraints) -> Self {
        Self {
            min_width: self.min_width.clamp(other.min_width, other.max_width),
            max_width: self.max_width.clamp(other.min_width, other.max_width),
            min_height: self.min_height.clamp(other.min_height, other.max_height),
            max_height: self.max_height.clamp(other.min_height, other.max_height),
        }
    }

    pub fn tighten(&self, width: f32, height: f32) -> Self {
        Self {
            min_width: width.clamp(self.min_width, self.max_width),
            max_width: width.clamp(self.min_width, self.max_width),
            min_height: height.clamp(self.min_height, self.max_height),
            max_height: height.clamp(self.min_height, self.max_height),
        }
    }

    pub fn tighten_width(&self, width: f32) -> Self {
        Self {
            min_width: width.clamp(self.min_width, self.max_width),
            max_width: width.clamp(self.min_width, self.max_width),
            ..*self
        }
    }

    pub fn tighten_height(&self, height: f32) -> Self {
        Self {
            min_height: height.clamp(self.min_height, self.max_height),
            max_height: height.clamp(self.min_height, self.max_height),
            ..*self
        }
    }

    pub fn get_flipped(&self) -> Self {
        Self {
            min_width: self.min_height,
            max_width: self.max_height,
            min_height: self.min_width,
            max_height: self.max_width,
        }
    }

    pub fn width_constraints(&self) -> Self {
        Self {
            min_height: 0.0,
            max_height: f32::INFINITY,
            ..*self
        }
    }

    pub fn height_constraints(&self) -> Self {
        Self {
            min_width: 0.0,
            max_width: f32::INFINITY,
            ..*self
        }
    }

    pub fn constrain_width(&self, width: f32) -> f32 {
        width.clamp(self.min_width, self.max_width)
    }

    pub fn constrain_height(&self, height: f32) -> f32 {
        height.clamp(self.min_height, self.max_height)
    }

    pub fn constrain(&self, size: Size) -> Size {
        return Size {
            w: self.constrain_width(size.w),
            h: self.constrain_height(size.h),
        };
    }

    pub fn constrain_dimensions(&self, width: f32, height: f32) -> Size {
        return Size {
            w: self.constrain_width(width),
            h: self.constrain_height(height),
        };
    }

    pub fn constrain_size_and_attempt_to_preserve_aspect_ratio(&self, size: Size) -> Size {
        if self.is_tight() {
            return self.smallest();
        }

        let mut width = size.w;
        let mut height = size.h;

        assert!(width > 0.0);
        assert!(height > 0.0);

        let aspect_ratio = width / height;

        if width > self.max_width {
            width = self.max_width;
            height = width / aspect_ratio;
        }

        if height > self.max_height {
            height = self.max_height;
            width = height * aspect_ratio;
        }

        if width < self.min_width {
            width = self.min_width;
            height = width / aspect_ratio;
        }

        if height < self.min_height {
            height = self.min_height;
            width = height * aspect_ratio;
        }

        self.constrain_dimensions(width, height)
    }

    pub fn biggest(&self) -> Size {
        Size {
            w: self.max_width,
            h: self.max_height,
        }
    }

    pub fn smallest(&self) -> Size {
        Size {
            w: self.min_width,
            h: self.min_height,
        }
    }

    pub fn has_tight_width(&self) -> bool {
        self.max_width == self.min_width
    }

    pub fn has_tight_height(&self) -> bool {
        self.max_height == self.min_height
    }

    pub fn is_tight(&self) -> bool {
        self.has_tight_width() && self.has_tight_height()
    }

    pub fn has_bounded_width(&self) -> bool {
        self.max_width < f32::INFINITY
    }

    pub fn has_bounded_height(&self) -> bool {
        self.max_height < f32::INFINITY
    }

    pub fn has_infinite_width(&self) -> bool {
        self.max_width >= f32::INFINITY
    }

    pub fn has_infinite_height(&self) -> bool {
        self.max_height >= f32::INFINITY
    }

    pub fn is_normailzed(&self) -> bool {
        self.min_width >= 0.0
            && self.min_width <= self.max_width
            && self.min_height >= 0.0
            && self.min_height <= self.max_height
    }

    pub fn is_satisfied_by(&self, size: Size) -> bool {
        self.min_width <= size.w
            && size.w <= self.max_width
            && self.min_height <= size.h
            && size.h <= self.max_height
    }

    pub fn normailze(&self) -> Self {
        if self.is_normailzed() {
            return self.clone();
        }
        let min_width = self.min_width.max(0.0);
        let min_height = self.min_height.max(0.0);
        Self {
            min_width,
            max_width: self.min_width.max(self.max_width),
            min_height,
            max_height: self.min_height.max(self.max_height),
        }
    }

    pub fn mul(&self, factor: f32) -> Self {
        Self {
            min_width: self.min_width * factor,
            max_width: self.max_width * factor,
            min_height: self.min_height * factor,
            max_height: self.max_height * factor,
        }
    }

    pub fn div(&self, factor: f32) -> Self {
        Self {
            min_width: self.min_width / factor,
            max_width: self.max_width / factor,
            min_height: self.min_height / factor,
            max_height: self.max_height / factor,
        }
    }

    pub fn idiv(&self, factor: f32) -> Self {
        Self {
            min_width: (self.min_width / factor).floor(),
            max_width: (self.max_width / factor).floor(),
            min_height: (self.min_height / factor).floor(),
            max_height: (self.max_height / factor).floor(),
        }
    }

    pub fn modulo(&self, factor: f32) -> Self {
        Self {
            min_width: self.min_width % factor,
            max_width: self.max_width % factor,
            min_height: self.min_height % factor,
            max_height: self.max_height % factor,
        }
    }

    // pub fn lerp(a: &BoxConstraints, b: &BoxConstraints, t: f32) -> Self {
    //     if a == b { return a.clone(); }
    // return BoxConstraints(
    //   minWidth: a.minWidth.isFinite ? ui.lerpDouble(a.minWidth, b.minWidth, t)! : double.infinity,
    //   maxWidth: a.maxWidth.isFinite ? ui.lerpDouble(a.maxWidth, b.maxWidth, t)! : double.infinity,
    //   minHeight: a.minHeight.isFinite ? ui.lerpDouble(a.minHeight, b.minHeight, t)! : double.infinity,
    //   maxHeight: a.maxHeight.isFinite ? ui.lerpDouble(a.maxHeight, b.maxHeight, t)! : double.infinity,
    // );
    // }

    // pub fn lerp_to_unconstrained(a: &BoxConstraints, t: f32) -> Self {
    //     a.mul(1.0 - t)
    // }

    // pub fn lerp_from_unconstrained(a: &BoxConstraints, t: f32) -> Self {
    //     a.mul(t)
    // }
}

impl Display for BoxConstraints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let annotation = if self.is_normailzed() {
            ""
        } else {
            "; NOT NORMALIZED"
        };
        if self.min_width == f32::INFINITY && self.min_height == f32::INFINITY {
            return write!(f, "BoxConstraints(biggest{annotation})");
        }
        if self.min_width == 0.0
            && self.max_width == f32::INFINITY
            && self.min_height == 0.0
            && self.max_height == f32::INFINITY
        {
            return write!(f, "BoxConstraints(unconstrained{annotation})");
        }
        fn describe(min: f32, max: f32, dim: &str) -> String {
            if min == max {
                return format!("{dim}={min:.1}");
            }
            return format!("{min:.1}<={dim}<={max:.1}");
        }
        let width = describe(self.min_width, self.max_width, "w");
        let height = describe(self.min_height, self.max_height, "h");
        write!(f, "BoxConstraints({width}, {height}{annotation})")
    }
}
