use nalgebra::{Isometry3, Perspective3, Point3, Vector3};
use std::f32::consts::PI;

pub struct Camera {
    projection: Perspective3<f32>,
    left_right_radians: f32,
    up_down_radians: f32,
    orbit_radius: f32,
}

impl Camera {
    pub fn new() -> Camera {
        let fovy = PI / 3.0;

        Camera {
            projection: Perspective3::new(fovy, 1.0, 0.1, 100.0),
            left_right_radians: 45.0f32.to_radians(),
            up_down_radians: 80.0f32.to_radians(),
            orbit_radius: 15.,
        }
    }

    pub fn view(&self) -> Isometry3<f32> {
        let eye = self.get_eye_pos();

        let target = Point3::new(0.0, 0.0, 0.0);

        Isometry3::look_at_rh(&eye, &target, &Vector3::y())
    }

    // FIXME: Better name
    pub fn view_flipped_y(&self) -> Isometry3<f32> {
        let mut eye = self.get_eye_pos();
        eye.y = -1.0 * eye.y;

        let target = Point3::new(0.0, 0.0, 0.0);

        Isometry3::look_at_rh(&eye, &target, &Vector3::y())
    }

    pub fn get_eye_pos(&self) -> Point3<f32> {
        let yaw = self.left_right_radians;
        let pitch = self.up_down_radians;

        let eye_x = self.orbit_radius * yaw.sin() * pitch.cos();
        let eye_y = self.orbit_radius * pitch.sin();
        let eye_z = self.orbit_radius * yaw.cos() * pitch.cos();

        Point3::new(eye_x, eye_y, eye_z)
    }

    pub fn projection(&self) -> &Perspective3<f32> {
        &self.projection
    }

    pub fn orbit_left_right(&mut self, delta: f32) {
        self.left_right_radians += delta;
    }

    pub fn orbit_up_down(&mut self, delta: f32) {
        self.up_down_radians += delta;

        // Make sure:
        // 0.1 <= radians <= PI / 2.1

        if self.up_down_radians - (PI / 2.1) > 0. {
            self.up_down_radians = PI / 2.1;
        }

        if self.up_down_radians - 0.1 < 0. {
            self.up_down_radians = 0.1;
        }
    }

    pub fn orbit_radius(&self) -> f32 {
        self.orbit_radius
    }

    pub fn zoom(&mut self, zoom: f32) {
        self.orbit_radius += zoom;

        if self.orbit_radius > 30. {
            self.orbit_radius = 30.;
        } else if self.orbit_radius < 5. {
            self.orbit_radius = 5.;
        }
    }
}
