use crate::shader::Shader;
use crate::shader::ShaderKind;
use crate::Assets;
use crate::State;
use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;

pub trait Render<'a> {
    fn shader_kind() -> ShaderKind;

    fn shader(&'a self) -> &'a Shader;

    // FIXME: Buffers need to be created during shader creation, not just randomly
    // in the middle of buffer functions. This allows different meshes to share the
    // same skeleton since their VAO's will use the same buffers. For example a
    // helmet and torso can both share the joint buffers for the human armature.
    // Look over all of our implementations of buffer attributes..
    fn buffer_attributes(&self, gl: &GL);

    fn render(&self, gl: &GL, state: &State, assets: &Assets);

    fn buffer_f32_data(gl: &GL, data: &[f32], attrib: u32, size: i32) {
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let data_location = data.as_ptr() as u32 / 4;

        let data_array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(data_location, data_location + data.len() as u32);

        // FIXME: Create buffers at shader creation time
        let buffer = gl.create_buffer().unwrap();

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &data_array, GL::STATIC_DRAW);
        gl.vertex_attrib_pointer_with_i32(attrib, size, GL::FLOAT, false, 0, 0);
    }

    // FIXME: Normalize with buffer_f32_data
    fn buffer_u8_data(gl: &GL, data: &[u8], attrib: u32, size: i32) {
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let data_location = data.as_ptr() as u32;

        let data_array = js_sys::Uint8Array::new(&memory_buffer)
            .subarray(data_location, data_location + data.len() as u32);

        // FIXME: Create buffers at shader creation time
        let buffer = gl.create_buffer().unwrap();

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &data_array, GL::STATIC_DRAW);
        gl.vertex_attrib_pointer_with_i32(attrib, size, GL::UNSIGNED_BYTE, false, 0, 0);
    }

    // FIXME: Rename and normalize with other funcs.. move to Render trait.. Actually just
    // create the VAOs at the beginning of the application
    fn buffer_u16_indices(gl: &GL, indices: &[u16]) {
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

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
    }
}
