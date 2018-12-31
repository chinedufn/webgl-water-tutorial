use self::framebuffer::*;
pub(self) use self::mesh::*;
pub(self) use self::render_trait::*;
pub use self::texture_unit::*;
use self::water_tile::*;
use crate::app::Assets;
use crate::app::State;
use crate::canvas::{CANVAS_HEIGHT, CANVAS_WIDTH};
use crate::render::textured_quad::TexturedQuad;
use crate::shader::ShaderKind;
use crate::shader::ShaderSystem;
use js_sys::Reflect;
use std::cell::RefCell;
use std::collections::HashMap;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub static WATER_TILE_Y_POS: f32 = 0.0;

mod framebuffer;
mod mesh;
mod render_meshes;
mod render_trait;
mod texture_unit;
mod textured_quad;
mod water_tile;

struct VaoExtension {
    oes_vao_ext: js_sys::Object,
    vaos: RefCell<HashMap<String, Vao>>,
}

struct Vao(js_sys::Object);

pub struct WebRenderer {
    shader_sys: ShaderSystem,
    #[allow(unused)]
    depth_texture_ext: Option<js_sys::Object>,
    refraction_framebuffer: Framebuffer,
    reflection_framebuffer: Framebuffer,
    vao_ext: VaoExtension,
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

        let vao_ext = VaoExtension {
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

        let above = 1000000.0;
        // Position is positive instead of negative for.. mathematical reasons..
        let clip_plane = [0., 1., 0., above];

        self.render_refraction_fbo(gl, state, assets);
        self.render_reflection_fbo(gl, state, assets);

        gl.viewport(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);

        self.render_water(gl, state);
        self.render_meshes(gl, state, assets, clip_plane, false);

        self.render_refraction_visual(gl, state);
        self.render_reflection_visual(gl, state);
    }

    fn render_water(&mut self, gl: &WebGlRenderingContext, state: &State) {
        gl.bind_framebuffer(GL::FRAMEBUFFER, None);

        let water_shader = self.shader_sys.get_shader(&ShaderKind::Water).unwrap();
        self.shader_sys.use_program(gl, ShaderKind::Water);

        let water_tile = RenderableWaterTile::new(water_shader);

        self.prepare_for_render(gl, &water_tile, "water");
        water_tile.render(gl, state);
    }

    fn render_refraction_fbo(
        &mut self,
        gl: &WebGlRenderingContext,
        state: &State,
        assets: &Assets,
    ) {
        let Framebuffer { framebuffer, .. } = &self.refraction_framebuffer;
        gl.bind_framebuffer(GL::FRAMEBUFFER, framebuffer.as_ref());

        gl.viewport(0, 0, REFRACTION_TEXTURE_WIDTH, REFRACTION_TEXTURE_HEIGHT);

        gl.clear_color(0.53, 0.8, 0.98, 1.);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let clip_plane = [0., -1., 0., WATER_TILE_Y_POS];

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

        gl.viewport(0, 0, REFLECTION_TEXTURE_WIDTH, REFLECTION_TEXTURE_HEIGHT);

        gl.clear_color(0.53, 0.8, 0.98, 1.);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let clip_plane = [0., 1., 0., -WATER_TILE_Y_POS];

        self.render_meshes(gl, state, assets, clip_plane, true);
    }

    fn render_refraction_visual(&self, gl: &WebGlRenderingContext, state: &State) {
        let quad_shader = self
            .shader_sys
            .get_shader(&ShaderKind::TexturedQuad)
            .unwrap();
        self.shader_sys.use_program(gl, ShaderKind::TexturedQuad);
        let textured_quad = TexturedQuad::new(
            0,
            CANVAS_HEIGHT as u16,
            75,
            75,
            TextureUnit::Refraction as u8,
            quad_shader,
        );
        self.prepare_for_render(gl, &textured_quad, "RefractionVisual");
        textured_quad.render(gl, state);
    }

    fn render_reflection_visual(&self, gl: &WebGlRenderingContext, state: &State) {
        let quad_shader = self
            .shader_sys
            .get_shader(&ShaderKind::TexturedQuad)
            .unwrap();
        self.shader_sys.use_program(gl, ShaderKind::TexturedQuad);
        let textured_quad = TexturedQuad::new(
            CANVAS_WIDTH as u16 - 75,
            CANVAS_HEIGHT as u16,
            75,
            75,
            TextureUnit::Reflection as u8,
            quad_shader,
        );

        self.prepare_for_render(gl, &textured_quad, "ReflectionVisual");
        textured_quad.render(gl, state);
    }

    fn create_vao(&self) -> Vao {
        let oes_vao_ext = &self.vao_ext.oes_vao_ext;

        let create_vao_ext = Reflect::get(oes_vao_ext, &"createVertexArrayOES".into())
            .expect("Create vao func")
            .into();

        Vao(
            Reflect::apply(&create_vao_ext, oes_vao_ext, &js_sys::Array::new())
                .expect("Created vao")
                .into(),
        )
    }

    fn prepare_for_render<'a>(
        &self,
        gl: &WebGlRenderingContext,
        renderable: &impl Render<'a>,
        key: &str,
    ) {
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

    fn bind_vao(&self, vao: &Vao) {
        let oes_vao_ext = &self.vao_ext.oes_vao_ext;

        let bind_vao_ext = Reflect::get(&oes_vao_ext, &"bindVertexArrayOES".into())
            .expect("Create vao func")
            .into();

        let args = js_sys::Array::new();
        args.push(&vao.0);

        Reflect::apply(&bind_vao_ext, oes_vao_ext, &args).expect("Bound VAO");
    }
}
