use nalgebra;
use crate::app::State;
use js_sys::WebAssembly;
use nalgebra::{Isometry3, Matrix4, Perspective3, Point3, Vector3};
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use crate::shader::ShaderSystem;
use crate::shader::ShaderKind;
use crate::shader::Shader;

pub struct WebRenderer {
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

pub trait Render {
    fn shader_kind() -> ShaderKind;

    fn render(&self, gl: &WebGlRenderingContext, state: &State, shader: &Shader);
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
