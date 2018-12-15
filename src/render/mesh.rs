//  https://github.com/chinedufn/akigi/blob/d73db7e62565bce706dd1c62d385115db80460c6/game-client/web-client/src/render/mesh.rs#L21

use crate::app::Assets;
use crate::app::State;
use crate::render::Render;
use crate::shader::Shader;
use crate::shader::ShaderKind;
use blender_mesh::BlenderMesh;
use js_sys::WebAssembly;
use nalgebra;
use nalgebra::{Isometry3, Point3, Vector3};
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub struct RenderableMesh<'a> {
    pub mesh: &'a BlenderMesh,
    pub opts: &'a MeshRenderOpts,
    // TODO: pub buffers
}

pub struct MeshRenderOpts {
    pub pos: (f32, f32, f32),
}

impl<'a> Render for RenderableMesh<'a> {
    fn shader_kind() -> ShaderKind {
        ShaderKind::Mesh
    }

    fn render(&self, gl: &WebGlRenderingContext, state: &State, assets: &Assets, shader: &Shader) {
        let mesh = self.mesh;
        let opts = self.opts;
        let pos = opts.pos;

        let pos_attrib = gl.get_attrib_location(&shader.program, "position");
        gl.enable_vertex_attrib_array(pos_attrib as u32);

        let normal_attrib = gl.get_attrib_location(&shader.program, "normal");
        gl.enable_vertex_attrib_array(normal_attrib as u32);

        let view = state.camera().view();
        let model = Isometry3::new(Vector3::new(pos.0, pos.1, pos.2), nalgebra::zero());
        let mut model_view_array = [0.; 16];

        let model_view = view.to_homogeneous() * model.to_homogeneous();

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

        let indices = &mesh.vertex_position_indices[..];

        RenderableMesh::buffer_f32_data(&gl, &mesh.vertex_positions[..], pos_attrib as u32, 3);
        RenderableMesh::buffer_f32_data(&gl, &mesh.vertex_normals[..], normal_attrib as u32, 3);
        RenderableMesh::buffer_u16_indices(&gl, indices);

        gl.draw_elements_with_i32(GL::TRIANGLES, indices.len() as i32, GL::UNSIGNED_SHORT, 0);
    }
}
