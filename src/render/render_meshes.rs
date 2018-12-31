use crate::render::MeshRenderOpts;
use crate::render::NonSkinnedMesh;
use crate::render::Render;
use crate::render::SkinnedMesh;
use crate::render::WebRenderer;
use crate::shader::ShaderKind;
use crate::Assets;
use crate::State;
use web_sys::WebGlRenderingContext as GL;

static BIRD_SPEED: f32 = 3.5;
static BIRD_START_Z: f32 = -30.0;
static BIRD_END_Z: f32 = 30.0;

impl WebRenderer {
    pub(in crate::render) fn render_meshes(
        &self,
        gl: &GL,
        state: &State,
        assets: &Assets,
        clip_plane: [f32; 4],
        flip_camera_y: bool,
    ) {
        let (skin, no_skin) = (ShaderKind::SkinnedMesh, ShaderKind::NonSkinnedMesh);

        // Render Terrain

        let non_skinned_shader = self.shader_sys.get_shader(&no_skin).unwrap();
        self.shader_sys.use_program(gl, ShaderKind::NonSkinnedMesh);

        let mesh_opts = MeshRenderOpts {
            pos: (0., 0., 0.),
            clip_plane,
            flip_camera_y,
        };

        let mesh_name = "Terrain";
        let terrain = NonSkinnedMesh {
            mesh: assets.get_mesh(mesh_name).expect("Terrain mesh"),
            shader: non_skinned_shader,
            opts: &mesh_opts,
        };

        self.prepare_for_render(gl, &terrain, mesh_name);
        terrain.render(gl, state, assets);

        // Render Bird

        let skinned_shader = self.shader_sys.get_shader(&skin).unwrap();
        self.shader_sys.use_program(gl, ShaderKind::SkinnedMesh);

        let bird_traveled = (state.clock() / 1000.0) * BIRD_SPEED;
        let z = BIRD_START_Z + (bird_traveled % (BIRD_END_Z - BIRD_START_Z));

        let mesh_opts = MeshRenderOpts {
            pos: (0., 6., z),
            clip_plane,
            flip_camera_y,
        };

        let mesh_name = "Bird";
        let armature_name = "Armature.001";
        let bird = SkinnedMesh {
            mesh: assets.get_mesh(mesh_name).expect("Bird mesh"),
            armature: assets.get_armature(armature_name).expect("Bird armature"),
            shader: skinned_shader,
            opts: &mesh_opts,
        };

        self.prepare_for_render(gl, &bird, mesh_name);
        bird.render(gl, state, assets);
    }
}
