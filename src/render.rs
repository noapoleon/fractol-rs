use std::ops::Range;

use crate::complex::Complex;
use crate::fractals;
use crate::fractals::Fractal;
use crate::utils;
use crate::view::Viewpoint;

pub struct RenderParams {
    pub fractal: Fractal,
    max_iter: u32,
    color_factor: f64,
    pub complex_offset: Complex,
    pub complex_follow: bool,
    // color_ramp: Vec<(f64, u32)>;
}

impl RenderParams {
    pub fn new(fractal: Fractal, max_iter: u32) -> Self {
        Self {
            fractal,
            max_iter,
            color_factor: 1.0 / max_iter as f64,
            complex_offset: Complex { re: 0.0, im: 0.0 },
            complex_follow: true,
        }
    }

    // getters
    pub fn max_iter(&self) -> u32 {
        self.max_iter
    }
    pub fn color_factor(&self) -> f64 {
        self.color_factor
    }

    // setters
    pub fn set_max_iter(&mut self, n: u32) -> Redraw {
        if self.max_iter == n {
            return Redraw::None;
        }
        self.max_iter = n;
        self.color_factor = 1.0 / n as f64;
        Redraw::Full
    }
}

pub enum IterChange {
    Mul(u32),
    Div(u32),
}

pub enum Redraw {
    None,
    Partial(isize, isize),
    Full,
}

impl Redraw {
    pub fn merge(self, other: Redraw) -> Self {
        match (self, other) {
            (Redraw::Full, _) | (_, Redraw::Full) => Redraw::Full,
            (Redraw::None, r) | (r, Redraw::None) => r,
            (Redraw::Partial(a, b), Redraw::Partial(c, d)) => Redraw::Partial(a + c, b + d),
        }
    }
}

pub struct Renderer {
    pub buf: Vec<u32>,
    pub params: RenderParams,
    pub redraw: Redraw,
}

impl Renderer {
    pub fn new(buf: Vec<u32>, params: RenderParams) -> Self {
        Self {
            buf,
            params,
            redraw: Redraw::Full,
        }
    }

    /// computes pixel color based on selected fractal
    pub fn pixel_color(&self, pos: (usize, usize), vp: &Viewpoint) -> u32 {
        let point = vp.pixel_to_point(pos);
        let res = match self.params.fractal {
            Fractal::Mandelbrot => fractals::mandelbrot(point, self.params.max_iter()) as u8,
            Fractal::Julia => fractals::julia(point, self.params.complex_offset, self.params.max_iter()) as u8,
        };

        if res == self.params.max_iter() as u8 {
            0x0
        } else {
            let col = (res as f64 * self.params.color_factor() * 255.0) as u8;

            utils::from_u8_rgb(col, 0, 0)
        }
    }

    pub fn render(&mut self, vp: &Viewpoint) -> Redraw {
        let r = match self.redraw {
            Redraw::Full => self.render_full(vp),
            Redraw::Partial(dx, dy) => self.render_partial(vp, (dx, dy)) ,
            Redraw::None => Redraw::None,
        };
        self.redraw = Redraw::None;
        r
    }

    fn render_rect(&mut self, vp: &Viewpoint, (rx, ry): (Range<usize>, Range<usize>)) {
        for y in ry {
            for x in rx.clone() {
                self.buf[y * vp.width + x] = self.pixel_color((x, y), vp);
            }
        }
    }

    fn render_full(&mut self, vp: &Viewpoint) -> Redraw {
        self.render_rect(vp, (0..vp.width, 0..vp.height));
        Redraw::Full
    }

    fn render_partial(&mut self, vp: &Viewpoint, (dx, dy): (isize, isize)) -> Redraw {
        // delta is fully out of viewpoint
        if dx.unsigned_abs() > vp.width || dy.unsigned_abs() > vp.height {
            return self.render_full(vp);
        }

        // define dest ranges
        let rx = if dx > 0 { dx.unsigned_abs()..vp.width } else { 0..vp.width - dx.unsigned_abs() };
        let ry = if dy > 0 { dy.unsigned_abs()..vp.height } else { 0..vp.height - dy.unsigned_abs() };

        let mut old_x: usize;
        let mut old_y: usize;
        let buf_copy = self.buf.clone();
        // shift reusable buf slice
        for y in ry {
            for x in rx.clone() {
                old_x = (x as isize - dx).unsigned_abs();
                old_y = (y as isize - dy).unsigned_abs();
                // self.buf[y * vp.width + x] = self.buf[old_y * vp.height + old_x];
                self.buf[y * vp.width + x] = buf_copy[old_y * vp.width + old_x];
            }
        }

        // TODO: render 1 to 2 extra rectangles
        let rx = if dx > 0 { 0..dx.unsigned_abs() } else { vp.width - dx.unsigned_abs()..vp.width };
        let ry = if dy > 0 { 0..dy.unsigned_abs() } else { vp.height - dy.unsigned_abs()..vp.height };
        self.render_rect(vp, (
            rx,
            0..vp.height,
        ));
        self.render_rect(vp, (
            0..vp.width,
            ry,
        ));

        Redraw::Partial(dx, dy)
    }

    pub fn change_max_iter(&mut self, ic: IterChange) -> Redraw {
        let mut n = match ic {
            IterChange::Mul(f) => self.params.max_iter * f,
            IterChange::Div(f) => self.params.max_iter / f,
        };
        if n == 0 {
            n = 1;
        }
        self.params.set_max_iter(n)
    }

    pub fn cycle_fractal(&mut self) -> Redraw {
        self.params.fractal = self.params.fractal.next();
        Redraw::Full
    }

    pub fn toggle_complex_offset(&mut self) -> Redraw {
        self.params.complex_follow = !self.params.complex_follow;
        Redraw::Full
    }

    pub fn update_complex_offset(&mut self, vp: &Viewpoint, pos: (f32, f32)) -> Redraw {
        self.params.complex_offset = vp.pixel_to_point((pos.0.trunc() as usize , pos.1.trunc() as usize));
        Redraw::Full
    }

    pub fn mouse_moved(&mut self, vp: &Viewpoint, x: f32, y: f32) -> Redraw {
        if self.params.complex_follow {
            self.update_complex_offset(vp, (x, y));
            Redraw::Full
        } else {
            Redraw::None
        }
    }
}
