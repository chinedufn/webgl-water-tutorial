//! An example of how to render water using WebGL + Rust + WebAssembly.
//!
//! We'll try to heavily over comment the code so that it's more accessible to those that
//! are less familiar with the techniques that are used.
//!
//! If you have any questions or comments feel free to open an issue on GitHub!
//!
//! https://github.com/chinedufn/webgl-water-tutorial

#![deny(missing_docs)]
#![feature(custom_attribute)]

extern crate wasm_bindgen;
use console_error_panic_hook;
use wasm_bindgen::prelude::*;
use web_sys::*;
use wasm_bindgen::JsCast;

/// Used to instantiate our application
#[wasm_bindgen]
pub struct WebGLWaterApp {}

#[wasm_bindgen]
impl WebGLWaterApp {
    /// Create a new instance of our WebGL Water application
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebGLWaterApp {
        console_error_panic_hook::set_once();

        WebGLWaterApp {}
    }

    /// Start our WebGL Water application. `main.rs` will call this function in order
    /// to begin rendering.
    pub fn start(&self) -> Result<(), JsValue> {
        let document = window().unwrap().document().unwrap();

        let canvas: HtmlCanvasElement = document.create_element("canvas").unwrap().dyn_into()?;

        canvas.set_width(500);
        canvas.set_height(500);

        let gl: WebGlRenderingContext = canvas.get_context("webgl")?.unwrap().dyn_into()?;

        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);

        document.body().unwrap().append_child(&canvas)?;

        Ok(())
    }
}
