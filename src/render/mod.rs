use crate::app::Assets;
use crate::app::State;
use crate::shader::Shader;
use crate::shader::ShaderKind;
use crate::shader::ShaderSystem;
use nalgebra;
use nalgebra::{Isometry3, Matrix4, Perspective3, Point3, Vector3};
use std::rc::Rc;
use web_sys::*;

mod water_tile;
use self::water_tile::*;

mod mesh;
use self::mesh::*;

pub struct WebRenderer {
    shader_sys: ShaderSystem,
}

impl WebRenderer {
    pub fn new(gl: &WebGlRenderingContext) -> WebRenderer {
        let shader_sys = ShaderSystem::new(&gl);

        WebRenderer { shader_sys }
    }

    pub fn render(&self, gl: &WebGlRenderingContext, state: &State, assets: &Assets) {
        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);

        let water_shader = self.shader_sys.get_shader(&ShaderKind::Water).unwrap();
        gl.use_program(Some(&water_shader.program));

        let water_tile = WaterTile::new();

        water_tile.render(gl, state, assets, water_shader);

        let mesh_opts = MeshRenderOpts { pos: (0., 0., 0.) };
        // FIXME: Auto generated enum from build.rs instead of stringly typed.. Model::Terrain.to_str()
        let renderable_mesh = RenderableMesh {
            mesh: assets.get_mesh("Terrain").unwrap(),
            opts: &mesh_opts,
        };

        let mesh_shader = self.shader_sys.get_shader(&ShaderKind::Mesh).unwrap();
        gl.use_program(Some(&mesh_shader.program));
        renderable_mesh.render(gl, state, assets, mesh_shader);
    }
}

pub trait Render {
    fn shader_kind() -> ShaderKind;

    fn render(&self, gl: &WebGlRenderingContext, state: &State, assets: &Assets, shader: &Shader);
}

struct WaterTile {}

impl WaterTile {
    pub fn new() -> WaterTile {
        WaterTile {}
    }
}
