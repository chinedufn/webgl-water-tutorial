#[derive(Default)]
pub struct Mouse {
    pressed: bool,
    x: u16,
    y: u16,
}

impl Mouse {
    pub fn get_pressed(&self) -> bool {
        self.pressed
    }

    pub fn set_pressed(&mut self, pressed: bool) {
        self.pressed = pressed;
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x as u16;
        self.y = y as u16;
    }

    pub fn get_pos(&self) -> (u16, u16) {
        (self.x, self.y)
    }
}
