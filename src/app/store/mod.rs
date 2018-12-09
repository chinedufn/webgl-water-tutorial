use std::ops::Deref;

mod mouse;
use self::mouse::*;

mod camera;
use self::camera::*;

pub struct Store {
    pub state: StateWrapper,
}

impl Store {
    pub fn new() -> Store {
        Store {
            state: StateWrapper(State::new()),
        }
    }

    pub fn msg(&mut self, msg: &Msg) {
        match msg {
            _ => self.state.msg(msg),
        }
    }
}

pub struct State {
    camera: Camera,
    mouse: Mouse,
}

impl State {
    fn new() -> State {
        State {
            camera: Camera::new(),
            mouse: Mouse::default(),
        }
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn msg(&mut self, msg: &Msg) {
        match msg {
            Msg::MouseDown(x, y) => {
                self.mouse.set_pressed(true);
                self.mouse.set_pos(*x, *y);
            }
            Msg::MouseUp | Msg::MouseOut => {
                self.mouse.set_pressed(false);
            }
            Msg::MouseMove(x, y) => {
                if !self.mouse.get_pressed() {
                    return;
                }

                let (old_x, old_y) = self.mouse.get_pos();

                let x_delta = old_x as i32 - x;
                let y_delta = y - old_y as i32;

                self.camera.orbit_left_right(x_delta as f32 / 50.0);
                self.camera.orbit_up_down(y_delta as f32 / 50.0);

                self.mouse.set_pos(*x, *y);
            }
        }
    }
}

pub struct StateWrapper(State);

impl Deref for StateWrapper {
    type Target = State;

    fn deref(&self) -> &State {
        &self.0
    }
}

impl StateWrapper {
    pub fn msg(&mut self, msg: &Msg) {
        &self.0.msg(msg);
    }
}

pub enum Msg {
    MouseDown(i32, i32),
    MouseUp,
    MouseOut,
    MouseMove(i32, i32),
}
