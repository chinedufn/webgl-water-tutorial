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
use console_error_panic_hook;
use js_sys::WebAssembly;
use nalgebra;
use nalgebra::{Isometry3, Matrix4, Perspective3, Point3, Vector3};
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::WebGlRenderingContext as GL;
/// web_sys gives us access to browser APIs such as HtmlCanvasElement and WebGlRenderingContext
///
/// web_sys API docs
///  https://rustwasm.github.io/wasm-bindgen/api/web_sys/
use web_sys::*;

mod app;
use self::app::*;

// TODO: Use WebGlVertexArrayObject when we refactor and clean up

// TODO: Instruct reader on what version of Rust to use in README and in tutorial post

/// Used to run the application from the web
#[wasm_bindgen]
pub struct WebClient {
    app: Rc<App>,
    gl: Rc<WebGlRenderingContext>,
    renderer: WebRenderer,
}

mod render;
use self::render::*;

mod shader; // FIXME: create module file

mod canvas;
use self::canvas::*;
use std::cell::RefCell;

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
        // FIXME: Request animation frame in here
        self.create_du_dv_texture(Rc::clone(&self.gl));
        self.create_normal_map_texture(Rc::clone(&self.gl));

        Ok(())
    }

    /// Update our simulation
    pub fn update(&self, dt: f32) {
        self.app.store.borrow_mut().msg(&Msg::AdvanceClock(dt));
    }

    /// Render the scene. `index.html` will call this once every requestAnimationFrame
    pub fn render(&self) {
        self.renderer
            .render(&self.gl, &self.app.store.borrow().state, &self.app.assets());
    }
}

impl WebClient {
    /// FIXME: Better home for this... ... ?
    fn create_du_dv_texture(&self, gl: Rc<GL>) {
        let dudv_map = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));
        let dudv_map_clone = Rc::clone(&dudv_map);

        let onload = Closure::wrap(Box::new(move || {
            let texture = gl.create_texture();

            gl.active_texture(TextureUnit::Dudv.get());

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
                &dudv_map_clone.borrow(),
            )
            .expect("Dudv tex image 2d");
        }) as Box<dyn Fn()>);

        let dudv_map = dudv_map.borrow_mut();

        dudv_map.set_onload(Some(onload.as_ref().unchecked_ref()));
        dudv_map.set_src("/dudvmap.png");

        onload.forget();
    }

    // FIXME: Normalize with texture creation above.. Just need to pass in the texture unit everything
    // else is the same..
    fn create_normal_map_texture(&self, gl: Rc<GL>) {
        let dudv_map = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));
        let dudv_map_clone = Rc::clone(&dudv_map);

        let onload = Closure::wrap(Box::new(move || {
            let texture = gl.create_texture();

            gl.active_texture(TextureUnit::NormalMap.get());

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
                &dudv_map_clone.borrow(),
            )
            .expect("Dudv tex image 2d");
        }) as Box<dyn Fn()>);

        let dudv_map = dudv_map.borrow_mut();

        dudv_map.set_onload(Some(onload.as_ref().unchecked_ref()));
        dudv_map.set_src("/normalmap.png");

        onload.forget();
    }
}
