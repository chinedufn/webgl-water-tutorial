use crate::app::Assets;
use crate::app::State;
use crate::render::Render;
use crate::render::TextureUnit;
use crate::render::WaterTile;
use crate::shader::Shader;
use crate::shader::ShaderKind;
use js_sys::WebAssembly;
use nalgebra;
use nalgebra::{Isometry3, Matrix4, Vector3};
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

static WAVE_SPEED: f32 = 0.03;

impl Render for WaterTile {
    fn shader_kind() -> ShaderKind {
        ShaderKind::Water
    }

    fn render(&self, gl: &WebGlRenderingContext, state: &State, assets: &Assets, shader: &Shader) {
        let pos_attrib = gl.get_attrib_location(&shader.program, "position");
        gl.enable_vertex_attrib_array(pos_attrib as u32);

        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let pos = (0., 0.0, 0.);

        // FIXME: Move some of this to the trait for re-usability. Then normalize with mesh.rs
        let view = state.camera().view();
        let view = view.to_homogeneous();

        let x_scale = 18.;
        let z_scale = 18.;
        let scale = Matrix4::new_nonuniform_scaling(&Vector3::new(x_scale, 1.0, z_scale));

        let model = Isometry3::new(Vector3::new(pos.0, pos.1, pos.2), nalgebra::zero());
        let model = model.to_homogeneous();
        let model = scale * model;

        let mut model_array = [0.; 16];
        let mut view_array = [0.; 16];

        model_array.copy_from_slice(model.as_slice());
        view_array.copy_from_slice(view.as_slice());

        let model_uni = gl.get_uniform_location(&shader.program, "model");
        let model_uni = model_uni.as_ref();

        let view_uni = gl.get_uniform_location(&shader.program, "view");
        let view_uni = view_uni.as_ref();

        gl.uniform_matrix4fv_with_f32_array(model_uni, false, &mut model_array);
        gl.uniform_matrix4fv_with_f32_array(view_uni, false, &mut view_array);

        // FIXME: We should only do this once and cache it in the `shader`
        // Shader.get_uniform_location ... Shader.uniforms: HashMap<String, u8>
        // This way we don't hit the GPU over and over again for no reason
        gl.uniform1i(
            gl.get_uniform_location(&shader.program, "refractionTexture")
                .as_ref(),
            TextureUnit::Refraction as i32,
        );
        gl.uniform1i(
            gl.get_uniform_location(&shader.program, "reflectionTexture")
                .as_ref(),
            TextureUnit::Reflection as i32,
        );
        gl.uniform1i(
            gl.get_uniform_location(&shader.program, "dudvTexture")
                .as_ref(),
            TextureUnit::Dudv as i32,
        );
        gl.uniform1i(
            gl.get_uniform_location(&shader.program, "normalMap")
                .as_ref(),
            TextureUnit::NormalMap as i32,
        );
        gl.uniform1i(
            gl.get_uniform_location(&shader.program, "waterDepthTexture")
                .as_ref(),
            TextureUnit::RefractionDepth as i32,
        );

        let seconds_elapsed = state.clock() / 1000.;
        let dudv_offset = (WAVE_SPEED * seconds_elapsed) % 1.;

        // FIXME: Pass in `program` variable to save repeating `shader` over and over again
        gl.uniform1f(
            gl.get_uniform_location(&shader.program, "dudvOffset")
                .as_ref(),
            dudv_offset,
        );

        let camera_pos = state.camera().get_eye_pos();
        let mut camera_pos = [camera_pos.x, camera_pos.y, camera_pos.z];

        gl.uniform3fv_with_f32_array(
            gl.get_uniform_location(&shader.program, "cameraPos")
                .as_ref(),
            &mut camera_pos,
        );

        let perspective = state.camera().projection();
        let mut perspective_array = [0.; 16];
        perspective_array.copy_from_slice(perspective.as_matrix().as_slice());

        let perspective_uni = gl.get_uniform_location(&shader.program, "perspective");
        let perspective_uni = perspective_uni.as_ref();
        gl.uniform_matrix4fv_with_f32_array(perspective_uni, false, &mut perspective_array);

        // FIXME: Explain this better
        // x and z values, y is ommited since this is a flat surface. We set it in the vertex shader
        let vertices: [f32; 8] = [
            -0.5, 0.5, // Bottom Left
            0.5, 0.5, // Bottom Right
            0.5, -0.5, // Top Right
            -0.5, -0.5, // Top Left
        ];

        let mut indices: [u16; 6] = [0, 1, 2, 0, 2, 3];

        WaterTile::buffer_f32_data(&gl, &vertices, pos_attrib as u32, 2);
        WaterTile::buffer_u16_indices(&gl, &mut indices);

        gl.enable(GL::BLEND);
        gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);

        gl.draw_elements_with_i32(GL::TRIANGLES, indices.len() as i32, GL::UNSIGNED_SHORT, 0);

        gl.disable(GL::BLEND);
    }
}
