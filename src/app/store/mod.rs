use std::ops::Deref;

mod mouse;
use self::mouse::*;

mod camera;
use self::camera::*;

mod water;
use self::water::*;

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
    clock: f32,
    camera: Camera,
    mouse: Mouse,
    water: Water,
    show_scenery: bool
}

impl State {
    fn new() -> State {
        State {
            /// Time elapsed since the application started, in milliseconds
            clock: 0.,
            camera: Camera::new(),
            mouse: Mouse::default(),
            water: Water::new(),
            show_scenery: true
        }
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn water(&self) -> &Water {
        &self.water
    }

    /// The current time in milliseconds
    pub fn clock(&self) -> f32 {
        self.clock
    }

    pub fn show_scenery(&self) -> bool {
        self.show_scenery
    }

    pub fn msg(&mut self, msg: &Msg) {
        match msg {
            Msg::AdvanceClock(dt) => {
                self.clock += dt;
            }
            Msg::MouseDown(x, y) => {
                self.mouse.set_pressed(true);
                self.mouse.set_pos(*x, *y);
            }
            Msg::MouseUp => {
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
            Msg::Zoom(zoom) => {
                self.camera.zoom(*zoom);
            }
            Msg::SetReflectivity(reflectivity) => {
                self.water.reflectivity = *reflectivity;
            }
            Msg::SetFresnel(fresnel) => {
                self.water.fresnel_strength = *fresnel;
            }
            Msg::SetWaveSpeed(wave_speed) => {
                self.water.wave_speed = *wave_speed;
            }
            Msg::UseReflection(use_reflection) => {
                self.water.use_reflection = *use_reflection;
            }
            Msg::UseRefraction(use_refraction) => {
                self.water.use_refraction = *use_refraction;
            }
            Msg::ShowScenery(show_scenery) => {
                self.show_scenery = *show_scenery;
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
        let _ = &self.0.msg(msg);
    }
}

pub enum Msg {
    AdvanceClock(f32),
    MouseDown(i32, i32),
    MouseUp,
    MouseMove(i32, i32),
    Zoom(f32),
    SetReflectivity(f32),
    SetFresnel(f32),
    SetWaveSpeed(f32),
    UseReflection(bool),
    UseRefraction(bool),
    ShowScenery(bool),
}
