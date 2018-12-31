use crate::app::Assets;
use crate::app::State;
use crate::render::mesh::non_skinned_mesh::MeshRenderOpts;
use crate::render::Render;
use crate::render::TextureUnit;
use crate::shader::Shader;
use crate::shader::ShaderKind;
use blender_armature::ActionSettings;
use blender_armature::BlenderArmature;
use blender_armature::InterpolationSettings;
use blender_mesh::BlenderMesh;
use nalgebra;
use nalgebra::{Isometry3, Vector3};
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub struct SkinnedMesh<'a> {
    pub mesh: &'a BlenderMesh,
    pub armature: &'a BlenderArmature,
    pub shader: &'a Shader,
    pub opts: &'a MeshRenderOpts,
}

impl<'a> Render<'a> for SkinnedMesh<'a> {
    fn shader_kind() -> ShaderKind {
        ShaderKind::SkinnedMesh
    }

    fn shader(&'a self) -> &'a Shader {
        &self.shader
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

        let joint_indices_attrib = gl.get_attrib_location(&shader.program, "jointIndices");
        let joint_weights_attrib = gl.get_attrib_location(&shader.program, "jointWeights");
        gl.enable_vertex_attrib_array(joint_indices_attrib as u32);
        gl.enable_vertex_attrib_array(joint_weights_attrib as u32);
        SkinnedMesh::buffer_u8_data(
            &gl,
            &mesh.vertex_group_indices.as_ref().expect("Group indices")[..],
            joint_indices_attrib as u32,
            4,
        );
        SkinnedMesh::buffer_f32_data(
            &gl,
            &mesh.vertex_group_weights.as_ref().expect("Group weights")[..],
            joint_weights_attrib as u32,
            4,
        );

        SkinnedMesh::buffer_f32_data(&gl, &mesh.vertex_positions[..], pos_attrib as u32, 3);
        SkinnedMesh::buffer_f32_data(&gl, &mesh.vertex_normals[..], normal_attrib as u32, 3);
        SkinnedMesh::buffer_f32_data(
            &gl,
            &mesh.vertex_uvs.as_ref().expect("Mesh uvs")[..],
            uv_attrib as u32,
            2,
        );
        SkinnedMesh::buffer_u16_indices(&gl, &mesh.vertex_position_indices[..]);
    }

    fn render(&self, gl: &WebGlRenderingContext, state: &State, assets: &Assets) {
        let shader = self.shader();

        let mesh = self.mesh;
        let opts = self.opts;
        let pos = opts.pos;

        let model_uni = shader.get_uniform_location(gl, "model");
        let view_uni = shader.get_uniform_location(gl, "view");
        let camera_pos_uni = shader.get_uniform_location(gl, "cameraPos");
        let perspective_uni = shader.get_uniform_location(gl, "perspective");
        let clip_plane_uni = shader.get_uniform_location(gl, "clipPlane");
        let mesh_texture_uni = shader.get_uniform_location(gl, "meshTexture");

        gl.uniform4fv_with_f32_array(clip_plane_uni.as_ref(), &mut opts.clip_plane.clone()[..]);

        let mut view = if opts.flip_camera_y {
            state.camera().view_flipped_y()
        } else {
            state.camera().view()
        };
        gl.uniform_matrix4fv_with_f32_array(view_uni.as_ref(), false, &mut view);

        let model = Isometry3::new(Vector3::new(pos.0, pos.1, pos.2), nalgebra::zero());
        let mut model_array = [0.; 16];
        model_array.copy_from_slice(model.to_homogeneous().as_slice());
        gl.uniform_matrix4fv_with_f32_array(model_uni.as_ref(), false, &mut model_array);

        let mut perspective = state.camera().projection();
        gl.uniform_matrix4fv_with_f32_array(perspective_uni.as_ref(), false, &mut perspective);

        let camera_pos = state.camera().get_eye_pos();
        let mut camera_pos = [camera_pos.x, camera_pos.y, camera_pos.z];
        gl.uniform3fv_with_f32_array(camera_pos_uni.as_ref(), &mut camera_pos);

        gl.uniform1i(mesh_texture_uni.as_ref(), TextureUnit::Stone.texture_unit());

        self.set_armature_uniforms(gl, state);

        let num_indices = mesh.vertex_position_indices.len();
        gl.draw_elements_with_i32(GL::TRIANGLES, num_indices as i32, GL::UNSIGNED_SHORT, 0);
    }
}

impl<'a> SkinnedMesh<'a> {
    fn set_armature_uniforms(&self, gl: &WebGlRenderingContext, state: &State) {
        let shader = self.shader();
        let armature = &self.armature;

        let clock = state.clock();
        let current_time_secs = clock / 1000.0;

        let interp_opts = InterpolationSettings {
            current_time: current_time_secs,
            joint_indices: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
            blend_fn: None,
            current_action: ActionSettings::new("Fly.001", 0.0, true),
            previous_action: None,
        };
        let bones = armature.interpolate_bones(&interp_opts);

        let bone_count = bones.len() as u8;

        for index in 0..bone_count {
            let bone = bones.get(&index).expect("Interpolated bone");
            let bone = bone.as_slice();

            let (rot_quat, trans_quat) = bone.split_at(4);
            let (rq, tq) = (rot_quat, trans_quat);

            let rot_quat_uni =
                shader.get_uniform_location(gl, &format!("boneRotQuaternions[{}]", index));
            gl.uniform4f(rot_quat_uni.as_ref(), rq[0], rq[1], rq[2], rq[3]);

            let trans_quat_uni =
                shader.get_uniform_location(gl, &format!("boneTransQuaternions[{}]", index));
            gl.uniform4f(trans_quat_uni.as_ref(), tq[0], tq[1], tq[2], tq[3]);
        }
    }
}
