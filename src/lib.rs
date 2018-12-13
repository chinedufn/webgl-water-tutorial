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

/// Create the vertex shader for our water.
///
/// In a real application you _might_ store this in a `.glsl` file so that you have better syntax
/// highlighting and then use `include_str!` to import it.
static WATER_VERTEX_SHADER: &'static str = r#"
attribute vec3 position;

uniform mat4 perspective;
uniform mat4 modelView;

void main() {
    gl_Position = perspective * modelView * vec4(position, 1.0);
}
"#;

/// Create the fragment shader for our water.
static WATER_FRAGMENT_SHADER: &'static str = r#"
void main() {
    gl_FragColor = vec4(0.0, 0.0, 1.0, 1.0);
}
"#;

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

        let gl = Rc::new(WebClient::create_webgl_context(Rc::clone(&app)).unwrap());

        let renderer = WebRenderer::new(Rc::clone(&gl));

        WebClient { app, gl, renderer }
    }

    /// Start our WebGL Water application. `index.html` will call this function in order
    /// to begin rendering.
    pub fn start(&self) -> Result<(), JsValue> {
        Ok(())
    }

    fn create_webgl_context(app: Rc<App>) -> Result<WebGlRenderingContext, JsValue> {
        let window = window().unwrap();
        let document = window.document().unwrap();

        let canvas: HtmlCanvasElement = document.create_element("canvas").unwrap().dyn_into()?;

        canvas.set_width(500);
        canvas.set_height(500);

        {
            let app = Rc::clone(&app);

            let on_mouse_down = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                let x = event.client_x();
                let y = event.client_y();
                app.store.borrow_mut().msg(&Msg::MouseDown(x, y));
            }) as Box<FnMut(_)>);

            canvas.add_event_listener_with_callback(
                "mousedown",
                on_mouse_down.as_ref().unchecked_ref(),
            )?;

            on_mouse_down.forget();
        }

        {
            let app = Rc::clone(&app);

            let on_mouse_up = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                app.store.borrow_mut().msg(&Msg::MouseUp);
            }) as Box<FnMut(_)>);

            canvas.add_event_listener_with_callback(
                "mouseup",
                on_mouse_up.as_ref().unchecked_ref(),
            )?;

            on_mouse_up.forget();
        }
        {
            let app = Rc::clone(&app);

            let on_mouse_move = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                let x = event.client_x();
                let y = event.client_y();
                app.store.borrow_mut().msg(&Msg::MouseMove(x, y));
            }) as Box<FnMut(_)>);

            canvas.add_event_listener_with_callback(
                "mousemove",
                on_mouse_move.as_ref().unchecked_ref(),
            )?;

            on_mouse_move.forget();
        }
        {
            let app = Rc::clone(&app);

            let on_mouse_out = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                app.store.borrow_mut().msg(&Msg::MouseOut);
            }) as Box<FnMut(_)>);

            canvas.add_event_listener_with_callback(
                "mouseout",
                on_mouse_out.as_ref().unchecked_ref(),
            )?;

            on_mouse_out.forget();
        }

        let gl: WebGlRenderingContext = canvas.get_context("webgl")?.unwrap().dyn_into()?;

        gl.clear_color(0.0, 0.0, 0.0, 1.0);

        document.body().unwrap().append_child(&canvas)?;

        Ok(gl)
    }

    /// Render the scene. `index.html` will call this once every requestAnimationFrame
    pub fn render(&self) {
        self.renderer
            .render(&self.gl, &self.app.store.borrow().state);
    }
}

struct WebRenderer {
    gl: Rc<WebGlRenderingContext>,
    shader_sys: ShaderSystem,
}

impl WebRenderer {
    pub fn new(gl: Rc<WebGlRenderingContext>) -> WebRenderer {
        let shader_sys = ShaderSystem::new(&gl);

        WebRenderer { gl, shader_sys }
    }

    pub fn render(&self, gl: &WebGlRenderingContext, state: &State) {
        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);

        let water_shader = self.shader_sys.get_shader(&ShaderKind::Water).unwrap();
        gl.use_program(Some(&water_shader.program));

        let water_tile = WaterTile::new();

        water_tile.render(&gl, &state, water_shader);
    }
}

struct WaterTile {}

impl WaterTile {
    pub fn new() -> WaterTile {
        WaterTile {}
    }
}

impl Render for WaterTile {
    fn shader_kind() -> ShaderKind {
        ShaderKind::Water
    }

    fn render(&self, gl: &WebGlRenderingContext, state: &State, shader: &Shader) {
        let pos_attrib = gl.get_attrib_location(&shader.program, "position");

        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let pos = (0., 0., 0.);

        let view = state.camera().view();
        let model = Isometry3::new(Vector3::new(pos.0, pos.1, pos.2), nalgebra::zero());

        let x_scale = 7.0;
        let z_scale = 7.0;

        let scale = Matrix4::new_nonuniform_scaling(&Vector3::new(x_scale, 1.0, z_scale));

        let mut model_view_array = [0.; 16];

        let model_view = view.to_homogeneous() * scale * model.to_homogeneous();

        model_view_array.copy_from_slice(model_view.as_slice());

        let model_view_uni = gl.get_uniform_location(&shader.program, "modelView");
        let model_view_uni = model_view_uni.as_ref();

        gl.uniform_matrix4fv_with_f32_array(model_view_uni, false, &mut model_view_array);

        let perspective = state.camera().projection();
        let mut perspective_array = [0.; 16];
        perspective_array.copy_from_slice(perspective.as_matrix().as_slice());

        let perspective_uni = gl.get_uniform_location(&shader.program, "perspective");
        let perspective_uni = perspective_uni.as_ref();
        gl.uniform_matrix4fv_with_f32_array(perspective_uni, false, &mut perspective_array);

        // TODO: Generate vertices based on WaterTile's fields (pos.. width.. height..)
        let vertices: [f32; 12] = [
            -0.5, 0., 0.5, // Bottom Left
            0.5, 0., 0.5, // Bottom Right
            0.5, 0., -0.5, // Top Right
            -0.5, 0., -0.5, // Top Left
        ];

        let vertices_location = vertices.as_ptr() as u32 / 4;

        let vert_array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(vertices_location, vertices_location + vertices.len() as u32);

        // TODO: Do this outside of the loop using a vertex array object. We don't
        // need to repeatedly buffer this.. Do this before moving on to rendering the
        // water.
        let buffer = gl.create_buffer().unwrap();

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);
        gl.vertex_attrib_pointer_with_i32(pos_attrib as u32, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(pos_attrib as u32);

        // TODO: Breadcrumb - u16
        let mut indices: [u16; 6] = [0, 1, 2, 0, 2, 3];

        let indices_location = indices.as_ptr() as u32 / 2;
        let indices_array = js_sys::Uint16Array::new(&memory_buffer)
            .subarray(indices_location, indices_location + indices.len() as u32);

        let index_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
        gl.buffer_data_with_array_buffer_view(
            GL::ELEMENT_ARRAY_BUFFER,
            &indices_array,
            GL::STATIC_DRAW,
        );

        // TODO: unsigned_short + buffer_data_with_u16_array
        gl.draw_elements_with_i32(GL::TRIANGLES, indices.len() as i32, GL::UNSIGNED_SHORT, 0);
    }
}

// TODO: Implement RAF in Rust
//        let cb = Closure::wrap(Box::new(move || {
//             web_sys::console::log_1(&"raf called".into());
//        }) as Box<FnMut()>);
//
//        cb.forget();
//        window.request_animation_frame(cb.as_ref().unchecked_ref())?;

trait Render {
    fn shader_kind() -> ShaderKind;

    fn render(&self, gl: &WebGlRenderingContext, state: &State, shader: &Shader);
}

struct ShaderSystem {
    programs: HashMap<ShaderKind, Shader>,
}

impl ShaderSystem {
    pub fn new(gl: &WebGlRenderingContext) -> ShaderSystem {
        let mut programs = HashMap::new();
        programs.insert(
            ShaderKind::Water,
            Shader::new(&gl, WATER_VERTEX_SHADER, WATER_FRAGMENT_SHADER).unwrap(),
        );

        ShaderSystem { programs }
    }

    pub fn get_shader(&self, shader_kind: &ShaderKind) -> Option<&Shader> {
        self.programs.get(shader_kind)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum ShaderKind {
    Water,
}

struct Shader {
    pub program: WebGlProgram,
}

impl Shader {
    /// Create a new Shader program from a vertex and fragment shader
    fn new(
        gl: &WebGlRenderingContext,
        vert_shader: &str,
        frag_shader: &str,
    ) -> Result<Shader, JsValue> {
        let vert_shader = compile_shader(&gl, WebGlRenderingContext::VERTEX_SHADER, vert_shader)?;
        let frag_shader = compile_shader(&gl, WebGlRenderingContext::FRAGMENT_SHADER, frag_shader)?;
        let program = link_program(&gl, &vert_shader, &frag_shader)?;

        Ok(Shader { program })
    }
}

/// Create a shader program using the WebGL APIs
fn compile_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| "Could not create shader".to_string())?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error creating shader".to_string()))
    }
}

/// Link a shader program using the WebGL APIs
fn link_program(
    gl: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| "Unable to create shader program".to_string())?;

    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);

    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| "Unknown error creating program".to_string()))
    }
}
