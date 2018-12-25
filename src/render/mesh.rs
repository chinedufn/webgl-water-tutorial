use crate::app::Assets;
use crate::app::State;
use crate::render::Render;
use crate::render::TextureUnit;
use crate::shader::Shader;
use crate::shader::ShaderKind;
use blender_mesh::BlenderMesh;
use nalgebra;
use nalgebra::{Isometry3, Vector3};
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub struct RenderableMesh<'a> {
    pub mesh: &'a BlenderMesh,
    pub shader: &'a Shader,
    pub opts: &'a MeshRenderOpts,
    // TODO: pub buffers
}

pub struct MeshRenderOpts {
    pub pos: (f32, f32, f32),
    pub clip_plane: [f32; 4],
    // FIXME: Better name
    pub flip_camera_y: bool,
}

impl<'a> Render<'a> for RenderableMesh<'a> {
    fn shader_kind() -> ShaderKind {
        ShaderKind::Mesh
    }

    fn shader(&'a self) -> &'a Shader {
        &self.shader
    }

    fn render(&self, gl: &WebGlRenderingContext, state: &State, assets: &Assets) {
        let shader = self.shader();

        let mesh = self.mesh;
        let opts = self.opts;
        let pos = opts.pos;

        // FIXME: Use VAO's

        let model_uni = shader.get_uniform_location(gl, "model");
        let view_uni = shader.get_uniform_location(gl, "view");
        let camera_pos_uni = shader.get_uniform_location(gl, "cameraPos");
        let perspective_uni = shader.get_uniform_location(gl, "perspective");
        let clip_plane_uni = shader.get_uniform_location(gl, "clipPlane");
        let mesh_texture_uni = shader.get_uniform_location(gl, "meshTexture");

        let view = if opts.flip_camera_y {
            state.camera().view_flipped_y()
        } else {
            state.camera().view()
        };

        let model = Isometry3::new(Vector3::new(pos.0, pos.1, pos.2), nalgebra::zero());

        let mut model_array = [0.; 16];
        let mut view_array = [0.; 16];

        model_array.copy_from_slice(model.to_homogeneous().as_slice());
        view_array.copy_from_slice(view.to_homogeneous().as_slice());

        let perspective = state.camera().projection();
        let mut perspective_array = [0.; 16];
        perspective_array.copy_from_slice(perspective.as_matrix().as_slice());

        let camera_pos = state.camera().get_eye_pos();
        let mut camera_pos = [camera_pos.x, camera_pos.y, camera_pos.z];

        // FIXME: Get rid of clone.. needed atm since render func isn't mut
        gl.uniform4fv_with_f32_array(clip_plane_uni.as_ref(), &mut opts.clip_plane.clone()[..]);

        gl.uniform3fv_with_f32_array(camera_pos_uni.as_ref(), &mut camera_pos);
        gl.uniform1i(mesh_texture_uni.as_ref(), TextureUnit::Stone as i32);
        gl.uniform_matrix4fv_with_f32_array(
            perspective_uni.as_ref(),
            false,
            &mut perspective_array,
        );

        gl.uniform_matrix4fv_with_f32_array(model_uni.as_ref(), false, &mut model_array);
        gl.uniform_matrix4fv_with_f32_array(view_uni.as_ref(), false, &mut view_array);

        let num_indices = mesh.vertex_position_indices.len();
        gl.draw_elements_with_i32(GL::TRIANGLES, num_indices as i32, GL::UNSIGNED_SHORT, 0);
    }

    fn buffer_attributes(&self, gl: &WebGlRenderingContext) {
        let shader = self.shader();
        let mesh = self.mesh;

        let pos_attrib = gl.get_attrib_location(&shader.program, "position");
        let normal_attrib = gl.get_attrib_location(&shader.program, "normal");
        let uv_attrib = gl.get_attrib_location(&shader.program, "uvs");

        gl.enable_vertex_attrib_array(pos_attrib as u32);
        gl.enable_vertex_attrib_array(normal_attrib as u32);
        gl.enable_vertex_attrib_array(uv_attrib as u32);

        RenderableMesh::buffer_f32_data(&gl, &mesh.vertex_positions[..], pos_attrib as u32, 3);
        RenderableMesh::buffer_f32_data(&gl, &mesh.vertex_normals[..], normal_attrib as u32, 3);
        RenderableMesh::buffer_f32_data(
            &gl,
            &mesh.vertex_uvs.as_ref().expect("Mesh uvs")[..],
            uv_attrib as u32,
            2,
        );
        RenderableMesh::buffer_u16_indices(&gl, &mesh.vertex_position_indices[..]);
    }
}
