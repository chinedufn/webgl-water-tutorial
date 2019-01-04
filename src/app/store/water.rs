pub struct Water {
    pub reflectivity: f32,
    pub fresnel_strength: f32,
    pub wave_speed: f32,
    pub use_reflection: bool,
    pub use_refraction: bool,
}

impl Water {
    pub fn new() -> Water {
        Water {
            reflectivity: 0.5,
            fresnel_strength: 1.5,
            wave_speed: 0.06,
            use_reflection: true,
            use_refraction: true,
        }
    }
}
