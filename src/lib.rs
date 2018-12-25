//! An example of how to render water using WebGL + Rust + WebAssembly.
//!
//! We'll try to heavily over comment the code so that it's more accessible to those that
//! are less familiar with the techniques that are used.
//!
//! In a real application you'd split things up into different modules and files,
//! but I tend to prefer tutorials that are all in one file that you can scroll up and down in
//! and soak up what you see vs. needing to hop around different files.
//!
//! If you have any questions or comments feel free to open an issue on GitHub!
//!
//! https://github.com/chinedufn/webgl-water-tutorial

#![deny(missing_docs)]
#![feature(custom_attribute)]

extern crate wasm_bindgen;
use self::canvas::*;
use self::render::*;
use console_error_panic_hook;
use js_sys::WebAssembly;
use nalgebra;
use nalgebra::{Isometry3, Matrix4, Perspective3, Point3, Vector3};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
mod app;
pub (in crate) use self::app::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
/// web_sys gives us access to browser APIs such as HtmlCanvasElement and WebGlRenderingContext
///
/// web_sys API docs
///  https://rustwasm.github.io/wasm-bindgen/api/web_sys/
use web_sys::*;

mod canvas;
mod render;
mod shader;

// TODO: Use WebGlVertexArrayObject when we refactor and clean up

// TODO: Instruct reader on what version of Rust to use in README and in tutorial post

/// Used to run the application from the web
#[wasm_bindgen]
pub struct WebClient {
    app: Rc<App>,
    gl: Rc<WebGlRenderingContext>,
    renderer: WebRenderer,
}
#[wasm_bindgen]
impl WebClient {
    /// Create a new web client
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebClient {
        console_error_panic_hook::set_once();

        let app = Rc::new(App::new());

        let gl = Rc::new(create_webgl_context(Rc::clone(&app)).unwrap());

        let renderer = WebRenderer::new(&gl);

        WebClient { app, gl, renderer }
    }

    /// Start our WebGL Water application. `index.html` will call this function in order
    /// to begin rendering.
    pub fn start(&self) -> Result<(), JsValue> {
        // FIXME: Request animation frame in here (compare performance)

        let gl = &self.gl;

        self.load_texture_image(Rc::clone(gl), "/dudvmap.png", TextureUnit::Dudv);
        self.load_texture_image(Rc::clone(gl), "/normalmap.png", TextureUnit::NormalMap);
        self.load_texture_image(Rc::clone(gl), "/stone-texture.png", TextureUnit::Stone);

        Ok(())
    }

    /// Update our simulation
    pub fn update(&self, dt: f32) {
        self.app.store.borrow_mut().msg(&Msg::AdvanceClock(dt));
    }

    /// Render the scene. `index.html` will call this once every requestAnimationFrame
    pub fn render(&mut self) {
        self.renderer
            .render(&self.gl, &self.app.store.borrow().state, &self.app.assets());
    }
}

impl WebClient {
    fn load_texture_image(&self, gl: Rc<GL>, src: &str, texture_unit: TextureUnit) {
        let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));
        let image_clone = Rc::clone(&image);

        let onload = Closure::wrap(Box::new(move || {
            let texture = gl.create_texture();

            gl.active_texture(texture_unit.get());

            gl.bind_texture(GL::TEXTURE_2D, texture.as_ref());

            gl.pixel_storei(GL::UNPACK_FLIP_Y_WEBGL, 1);

            gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
            gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);

            gl.tex_image_2d_with_u32_and_u32_and_image(
                GL::TEXTURE_2D,
                0,
                GL::RGBA as i32,
                GL::RGBA,
                GL::UNSIGNED_BYTE,
                &image_clone.borrow(),
            )
            .expect("Texture image 2d");
        }) as Box<dyn Fn()>);

        let image = image.borrow_mut();

        image.set_onload(Some(onload.as_ref().unchecked_ref()));
        image.set_src(src);

        onload.forget();
    }
}
