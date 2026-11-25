use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window};

pub enum Action {
    Quit,
    Pan(isize, isize),
    Zoom(f64),
    IterMul(u32),
    IterDiv(u32),
    CycleFractal,
    ToggleComplexOffset,
    MouseMoved(f32, f32),
}

pub struct Input {
    last_mouse: Option<(f32, f32)>,
    pub pan_carry: (f32, f32),
}

impl Input {
    pub fn new() -> Self {
        Self {
            last_mouse: None,
            pan_carry: (0.0, 0.0),
        }
    }

    pub fn poll(&mut self, window: &Window) -> Vec<Action> {
        let mut actions: Vec<Action> = vec![];
        // escape
        if window.is_key_down(Key::Escape) {
            actions.push(Action::Quit)
        }

        // mouse
        if let Some(pos) = window.get_mouse_pos(MouseMode::Discard) {
            // mouse moved
            if Some(pos) != self.last_mouse {
                actions.push(Action::MouseMoved(pos.0, pos.1));
            }
            // pan
            if window.get_mouse_down(MouseButton::Left) {
                let shift = self.pan_delta(pos);
                actions.push(Action::Pan(shift.0, shift.1));
            } else {
                self.pan_carry = (0.0, 0.0);
            }
            self.last_mouse = Some(pos);
        }
        // complex offset
        if window.is_key_pressed(Key::C, KeyRepeat::No) {
            actions.push(Action::ToggleComplexOffset);
        }

        // zoom
        if let Some((_, y)) = window.get_scroll_wheel() {
            match y {
                n if n < 0.0 => actions.push(Action::Zoom(0.9)),
                n if n > 0.0 => actions.push(Action::Zoom(1.1)),
                _ => {}
            }
        }

        // keybinds
        if window.is_key_pressed(Key::W, KeyRepeat::No) {
            actions.push(Action::IterMul(2));
        }
        if window.is_key_pressed(Key::S, KeyRepeat::No) {
            actions.push(Action::IterDiv(2));
        }
        if window.is_key_pressed(Key::F, KeyRepeat::No) {
            actions.push(Action::CycleFractal);
        }

        actions
    }

    fn pan_delta(&mut self, (x, y): (f32, f32)) -> (isize, isize) {
        let Some((lx, ly)) = self.last_mouse else {
            return (0, 0);
        };

        let fx = x - lx + self.pan_carry.0;
        let fy = y - ly + self.pan_carry.1;
        let (dx, dy) = (fx.trunc(), fy.trunc());
        self.pan_carry = (dx - fx, dy - fy);

        (dx as isize, dy as isize)
    }
}
