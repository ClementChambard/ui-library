pub struct AppBuffer {
    pub width: usize,
    pub height: usize,
    pub color: Vec<u32>,
    pub depth: Vec<usize>,
}

impl AppBuffer {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            width: w,
            height: h,
            color: vec![0; w * h],
            depth: vec![0; w * h],
        }
    }

    // Will clear the buffer too
    pub fn resize(&mut self, w: usize, h: usize) {
        self.width = w;
        self.height = h;
        self.color.resize(w * h, 0);
        self.depth.resize(w * h, 0);
    }

    // Function to draw a colored rectangle in the buffer
    pub fn draw_rectangle(
        &mut self,
        x: isize,
        y: isize,
        width: usize,
        height: usize,
        color: u32,
        depth: usize,
    ) {
        let (x, y, w, h) = clip_rectangle(x, y, width, height, self.width, self.height);
        for i in x..x + w {
            for j in y..y + h {
                let pos = self.width * j + i;
                if self.depth[pos] > depth {
                    continue;
                }
                self.color[pos] = color;
                self.depth[pos] = depth;
            }
        }
    }

    pub fn clear(&mut self, color: u32, depth: usize) {
        self.color.iter_mut().for_each(|pixel| *pixel = color);
        self.depth.iter_mut().for_each(|pixel| *pixel = depth);
    }
}

fn clip_rectangle(
    x: isize,
    y: isize,
    w: usize,
    h: usize,
    mx: usize,
    my: usize,
) -> (usize, usize, usize, usize) {
    if (x + (w as isize) < 0)
        || (y + (h as isize) < 0)
        || (x > (mx as isize))
        || (y > (my as isize))
    {
        return (0, 0, 0, 0);
    }
    let mut w = w;
    let mut h = h;
    let x = if x < 0 {
        w = (w as isize + x) as usize;
        0usize
    } else {
        x as usize
    };
    let y = if y < 0 {
        h = (h as isize + y) as usize;
        0usize
    } else {
        y as usize
    };
    if x + w > mx {
        w = mx - x;
    }
    if y + h > my {
        h = my - y;
    }
    (x, y, w, h)
}
