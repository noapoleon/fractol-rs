use crate::complex::Complex;

#[derive(Copy, Clone)]
pub enum Fractal {
    Mandelbrot,
    Julia,
}

impl Fractal {
    pub fn next(&self) -> Self {
        match self {
            Fractal::Mandelbrot => Fractal::Julia,
            Fractal::Julia => Fractal::Mandelbrot,
        }
    }
}

/// Takes real and imaginary part of a coordinate and returns.
/// Returns the number of iterations before escaping, otherwise max_iter
pub fn mandelbrot(c: Complex, max_iter: u32) -> u32 {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..max_iter {
        if z.re * z.re + z.im * z.im > 4.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iter
}
pub fn julia(point: Complex, c: Complex, max_iter: u32) -> u32 {
    let mut z = point;
    // c.re = -0.8;
    // c.im = 0.156;
    for i in 0..max_iter {
        if z.re * z.re + z.im * z.im > 4.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iter
}
