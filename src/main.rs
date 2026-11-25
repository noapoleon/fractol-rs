mod complex;
mod fractals;
mod input;
mod render;
mod utils;
mod view;

use std::time::{Duration, Instant};

use minifb::{Window, WindowOptions};

use crate::fractals::Fractal;
use crate::input::{Action, Input};
use crate::render::{IterChange, Redraw, RenderParams, Renderer};
use crate::view::Viewpoint;

fn main() {
    let mut vp = Viewpoint::new(1200, 1200);
    let mut renderer = Renderer::new(
        vec![0u32; vp.width * vp.height],
        RenderParams::new(Fractal::Mandelbrot, 32),
    );

    let mut window = Window::new("fractol", vp.width, vp.height, WindowOptions::default()).unwrap();
    window.set_target_fps(144);

    // Input manager
    let mut input = Input::new();

    // fps
    let mut start = Instant::now();
    let mut frames = 0;

    // TODO: only update if viewpoint changed
    // TODO: only recompute necessary (new parts, reuse previous buffer for still visible parts)
    while window.is_open() {
        // inputs
        for action in input.poll(&window) {
            let r = match action {
                Action::Quit => return,
                Action::Pan(dx, dy) => vp.pan_pixels(dx, dy),
                Action::Zoom(f) => vp.zoom(f),
                Action::IterMul(f) => renderer.change_max_iter(IterChange::Mul(f)),
                Action::IterDiv(f) => renderer.change_max_iter(IterChange::Div(f)),
                Action::CycleFractal => renderer.cycle_fractal(),
                Action::ToggleComplexOffset => renderer.toggle_complex_offset(),
                Action::MouseMoved(x, y) => renderer.mouse_moved(&vp, x, y),
            };
            renderer.redraw = renderer.redraw.merge(r);
        }

        // println!("fractal -> {:?}", renderer.params.fractal);
        match renderer.render(&vp) {
            Redraw::Full | Redraw::Partial(_, _) => {
                window
                    .update_with_buffer(&renderer.buf, vp.width, vp.height)
                    .unwrap();
            }
            Redraw::None => window.update(),
        }

        // fps
        if start.elapsed() >= Duration::from_secs(1) {
            start = Instant::now();
            println!("fps -> {frames}");
            frames = 0;
        }
        frames += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::complex::Complex;

    const VP: Viewpoint = Viewpoint {
        center: Complex { re: 0.0, im: 0.0 },
        scale: 2.0 / 2000.0,
        width: 2000,
        height: 2000,
    };

    #[track_caller]
    fn assert_close(expected: Complex, got: Complex) {
        const EPS: f64 = 1e-5;
        assert!((expected.re - got.re).abs() < EPS && (expected.im - got.im).abs() < EPS)
    }

    #[test]
    fn center() {
        let res = VP.pixel_to_point((1000, 1000));
        let expected = Complex {
            re: 0.0005,
            im: -0.0005,
        };
        assert_close(expected, res)
    }

    #[test]
    fn top_left() {
        let res = VP.pixel_to_point((100, 100));
        let expected = Complex {
            re: -0.8995,
            im: 0.8995,
        };
        assert_close(expected, res)
    }
    #[test]
    fn bot_right() {
        let res = VP.pixel_to_point((1900, 1900));
        let expected = Complex {
            re: 0.9005,
            im: -0.9005,
        };
        assert_close(expected, res)
    }

    #[test]
    fn color_translations() {
        assert_eq!(0x00ff0000, utils::from_u8_rgb(255, 0, 0));
        assert_eq!(0x0000ff00, utils::from_u8_rgb(0, 255, 0));
        assert_eq!(0x000000ff, utils::from_u8_rgb(0, 0, 255));
        assert_eq!(0x00ffffff, utils::from_u8_rgb(255, 255, 255));
        assert_eq!(0x00000000, utils::from_u8_rgb(0, 0, 0));
        assert_eq!(0x007f7f7f, utils::from_u8_rgb(127, 127, 127));
    }

    #[test]
    fn vec_rotate_90() {
        let a = Complex { re: 3.0, im: 1.0 };
        let b = Complex { re: 0.0, im: 1.0 };
        let expected = Complex { re: -1.0, im: 3.0 };
        let res = a * b;
        assert_close(expected, res)
    }
}
