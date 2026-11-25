use crate::complex::Complex;
use crate::render::Redraw;

pub struct Viewpoint {
    pub center: Complex,
    pub scale: f64,
    pub width: usize,
    pub height: usize,
    // pub max_iter: u32,
    // zoom: f64,
}

impl Viewpoint {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            center: Complex { re: -0.5, im: 0.0 },
            scale: 3.0 / width as f64,
            width,
            height,
            // max_iter: 32,
        }
    }

    pub fn pan_pixels(&mut self, dx: isize, dy: isize) -> Redraw {
        self.center.re -= dx as f64 * self.scale;
        self.center.im += dy as f64 * self.scale;
        Redraw::Partial(dx, dy)
    }

    pub fn zoom(&mut self, fac: f64) -> Redraw {
        self.scale *= fac;
        Redraw::Full
    }

    pub fn pixel_to_point(&self, (x, y): (usize, usize)) -> Complex {
        let (x, y): (f64, f64) = (x as f64 + 0.5, y as f64 + 0.5);
        Complex {
            re: self.center.re + self.scale * (x - self.width as f64 * 0.5),
            im: self.center.im + self.scale * (self.height as f64 * 0.5 - y),
        }
    }
}
