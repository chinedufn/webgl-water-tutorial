use self::framebuffer::*;
use self::mesh::*;
pub(self) use self::render_trait::*;
use self::water_tile::*;
use crate::app::Assets;
use crate::app::State;
use crate::canvas::CANVAS_HEIGHT;
use crate::canvas::CANVAS_WIDTH;
use crate::render::textured_quad::TexturedQuad;
use crate::shader::ShaderKind;
use crate::shader::ShaderSystem;
use js_sys::Reflect;
use nalgebra;
use nalgebra::{Isometry3, Matrix4, Perspective3, Point3, Vector3};
use std::cell::RefCell;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

mod framebuffer;
mod mesh;
mod render_trait;
mod textured_quad;
mod water_tile;

struct VAO_Extension {
    oes_vao_ext: js_sys::Object,
    vaos: RefCell<HashMap<String, js_sys::Object>>,
}

pub struct WebRenderer {
    shader_sys: ShaderSystem,
    #[allow(unused)]
    depth_texture_ext: Option<js_sys::Object>,
    refraction_framebuffer: Framebuffer,
    reflection_framebuffer: Framebuffer,
    vao_ext: VAO_Extension,
}

impl WebRenderer {
    pub fn new(gl: &WebGlRenderingContext) -> WebRenderer {
        let shader_sys = ShaderSystem::new(&gl);

        let depth_texture_ext = gl
            .get_extension("WEBGL_depth_texture")
            .expect("Depth texture extension");

        let oes_vao_ext = gl
            .get_extension("OES_vertex_array_object")
            .expect("Get OES vao ext")
            .expect("OES vao ext");

        let vao_ext = VAO_Extension {
            oes_vao_ext,
            vaos: RefCell::new(HashMap::new()),
        };

        let refraction_framebuffer = WebRenderer::create_refraction_framebuffer(&gl).unwrap();
        let reflection_framebuffer = WebRenderer::create_reflection_framebuffer(&gl).unwrap();

        WebRenderer {
            depth_texture_ext,
            shader_sys,
            refraction_framebuffer,
            reflection_framebuffer,
            vao_ext,
        }
    }

    pub fn render(&mut self, gl: &WebGlRenderingContext, state: &State, assets: &Assets) {
        gl.clear_color(0.53, 0.8, 0.98, 1.);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        // FIXME: Base distance on a water tile height variable -0.5
        let above = -1000000.0;
        // Have to flip it for.. mathematical reasons..
        let clip_plane = [0., 1., 0., -above];

        self.render_meshes(gl, state, assets, clip_plane, false);
        self.render_water(gl, state, assets);
    }

    // FIXME: Fewer args...
    fn render_meshes(
        &self,
        gl: &GL,
        state: &State,
        assets: &Assets,
        clip_plane: [f32; 4],
        flip_camera_y: bool,
    ) {
        let mesh_shader = self.shader_sys.get_shader(&ShaderKind::Mesh).unwrap();
        gl.use_program(Some(&mesh_shader.program));

        let mesh_opts = MeshRenderOpts {
            pos: (0., 0., 0.),
            clip_plane,
            flip_camera_y,
        };

        let mesh_name = "Terrain";
        let terrain = RenderableMesh {
            mesh: assets.get_mesh(mesh_name).expect("Terrain mesh"),
            shader: mesh_shader,
            opts: &mesh_opts,
        };

        // FIXME: Push everything that we want to render into a vector and have a separate
        // method to render that vector
        self.prepare(gl, &terrain, mesh_name);
        terrain.render(gl, state, assets);

        // FIXME: Normalize with above

        let mesh_opts = MeshRenderOpts {
            pos: (0., 10., 0.),
            clip_plane,
            flip_camera_y,
        };

        let mesh_name = "Bird";
        let bird = RenderableMesh {
            mesh: assets.get_mesh(mesh_name).expect("Bird mesh"),
            shader: mesh_shader,
            opts: &mesh_opts,
        };

        // FIXME: Push everything that we want to render into a vector and have a separate
        // method to render that vector
        self.prepare(gl, &bird, mesh_name);
        bird.render(gl, state, assets);
    }

    fn render_water(&mut self, gl: &WebGlRenderingContext, state: &State, assets: &Assets) {
        self.render_refraction_fbo(gl, state, assets);
        self.render_reflection_fbo(gl, state, assets);

        gl.bind_framebuffer(GL::FRAMEBUFFER, None);

        let water_shader = self.shader_sys.get_shader(&ShaderKind::Water).unwrap();
        gl.use_program(Some(&water_shader.program));

        let water_tile = RenderableWaterTile::new(water_shader);

        // FIXME: Enum for key
        self.prepare(gl, &water_tile, "water");
        water_tile.render(gl, state, assets);

        self.render_refraction_visual(gl, state, assets);
        self.render_reflection_visual(gl, state, assets);
    }

    fn render_refraction_fbo(
        &mut self,
        gl: &WebGlRenderingContext,
        state: &State,
        assets: &Assets,
    ) {
        let Framebuffer { framebuffer, .. } = &self.refraction_framebuffer;
        gl.bind_framebuffer(GL::FRAMEBUFFER, framebuffer.as_ref());

        gl.clear_color(0.53, 0.8, 0.98, 1.);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        // FIXME: Base distance on a water tile height variable -0.5
        let water_tile_y = 0.0;
        let clip_plane = [0., -1., 0., water_tile_y];

        self.render_meshes(gl, state, assets, clip_plane, false);
    }

    fn render_reflection_fbo(
        &mut self,
        gl: &WebGlRenderingContext,
        state: &State,
        assets: &Assets,
    ) {
        let Framebuffer { framebuffer, .. } = &self.reflection_framebuffer;
        gl.bind_framebuffer(GL::FRAMEBUFFER, framebuffer.as_ref());

        gl.clear_color(0.53, 0.8, 0.98, 1.);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        // FIXME: Base distance on a water tile height variable -0.5
        let water_tile_y = 0.0;
        // FIXME: Soft edges tutorial talks about how to adjust these clipping planes (near the end)
        let clip_plane = [0., 1., 0., -water_tile_y];

        self.render_meshes(gl, state, assets, clip_plane, true);
    }

    fn render_refraction_visual(&self, gl: &WebGlRenderingContext, state: &State, assets: &Assets) {
        let quad_shader = self
            .shader_sys
            .get_shader(&ShaderKind::TexturedQuad)
            .unwrap();
        gl.use_program(Some(&quad_shader.program));
        let textured_quad = TexturedQuad::new(
            0,
            CANVAS_HEIGHT as u16,
            75,
            75,
            TextureUnit::Refraction as u8,
            quad_shader,
        );
        self.prepare(gl, &textured_quad, "RefractionVisual");
        textured_quad.render(gl, state, assets);
    }

    fn render_reflection_visual(&self, gl: &WebGlRenderingContext, state: &State, assets: &Assets) {
        let quad_shader = self
            .shader_sys
            .get_shader(&ShaderKind::TexturedQuad)
            .unwrap();
        gl.use_program(Some(&quad_shader.program));
        let textured_quad = TexturedQuad::new(
            CANVAS_WIDTH as u16 - 75,
            CANVAS_HEIGHT as u16,
            75,
            75,
            TextureUnit::Reflection as u8,
            quad_shader,
        );

        self.prepare(gl, &textured_quad, "ReflectionVisual");
        textured_quad.render(gl, state, assets);
    }

    // FIXME: Wrap object in VAO() struct
    fn create_vao(&self) -> js_sys::Object {
        let oes_vao_ext = &self.vao_ext.oes_vao_ext;

        let create_vao_ext = Reflect::get(oes_vao_ext, &"createVertexArrayOES".into())
            .expect("Create vao func")
            .into();

        Reflect::apply(&create_vao_ext, oes_vao_ext, &js_sys::Array::new())
            .expect("Created vao")
            .into()
    }

    // FIXME: Rename... Just getting it working
    // FIXME: Move into trait?
    fn prepare<'a>(&self, gl: &WebGlRenderingContext, renderable: &impl Render<'a>, key: &str) {
        if self.vao_ext.vaos.borrow().get(key).is_none() {
            let vao = self.create_vao();
            self.bind_vao(&vao);
            renderable.buffer_attributes(gl);
            self.vao_ext.vaos.borrow_mut().insert(key.to_string(), vao);
            return;
        }

        let vaos = self.vao_ext.vaos.borrow();
        let vao = vaos.get(key).unwrap();
        self.bind_vao(vao);
    }

    fn bind_vao(&self, vao: &js_sys::Object) {
        let oes_vao_ext = &self.vao_ext.oes_vao_ext;

        let bind_vao_ext = Reflect::get(&oes_vao_ext, &"bindVertexArrayOES".into())
            .expect("Create vao func")
            .into();

        let args = js_sys::Array::new();
        args.push(vao);

        Reflect::apply(&bind_vao_ext, oes_vao_ext, &args).expect("Bound VAO");
    }
}
pub enum TextureUnit {
    Refraction = 0,
    Reflection = 1,
    Dudv = 2,
    NormalMap = 3,
    RefractionDepth = 4,
    Stone = 5,
}

impl TextureUnit {
    // FIXME: Rename
    pub fn get(&self) -> u32 {
        match self {
            TextureUnit::Refraction => GL::TEXTURE0,
            TextureUnit::Reflection => GL::TEXTURE1,
            TextureUnit::Dudv => GL::TEXTURE2,
            TextureUnit::NormalMap => GL::TEXTURE3,
            TextureUnit::RefractionDepth => GL::TEXTURE4,
            TextureUnit::Stone => GL::TEXTURE5,
        }
    }
}
