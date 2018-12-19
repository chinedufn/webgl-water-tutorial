use crate::app::Assets;
use crate::app::State;
use crate::shader::Shader;
use crate::shader::ShaderKind;
use crate::shader::ShaderSystem;
use js_sys::WebAssembly;
use nalgebra;
use nalgebra::{Isometry3, Matrix4, Perspective3, Point3, Vector3};
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

mod water_tile;
use self::water_tile::*;

mod mesh;
use self::mesh::*;
use crate::canvas::CANVAS_HEIGHT;
use crate::canvas::CANVAS_WIDTH;
use crate::render::textured_quad::TexturedQuad;
use wasm_bindgen::JsValue;

// FIXME: Use these.. Look at framebuffer tutorial (2)
static REFLECTION_TEXTURE_WIDTH: i32 = 128;
static REFLECTION_TEXTURE_HEIGHT: i32 = 128;

// FIXME: Experiment with 256x256
static REFRACTION_TEXTURE_WIDTH: i32 = 512;
static REFRACTION_TEXTURE_HEIGHT: i32 = 512;

mod textured_quad;

pub struct WebRenderer {
    shader_sys: ShaderSystem,
    depth_texture_ext: Option<js_sys::Object>,
    refraction_framebuffer: Framebuffer,
    reflection_framebuffer: Framebuffer,
}

impl WebRenderer {
    pub fn new(gl: &WebGlRenderingContext) -> WebRenderer {
        let shader_sys = ShaderSystem::new(&gl);

        let depth_texture_ext = gl
            .get_extension("WEBGL_depth_texture")
            .expect("Depth texture extension");

        let refraction_framebuffer = WebRenderer::create_refraction_framebuffer(&gl).unwrap();
        let reflection_framebuffer = WebRenderer::create_reflection_framebuffer(&gl).unwrap();

        WebRenderer {
            depth_texture_ext,
            shader_sys,
            refraction_framebuffer,
            reflection_framebuffer,
        }
    }

    pub fn render(&self, gl: &WebGlRenderingContext, state: &State, assets: &Assets) {
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
        // FIXME: Auto generated enum from build.rs instead of stringly typed.. Model::Terrain.to_str()
        let renderable_mesh = RenderableMesh {
            mesh: assets.get_mesh("Terrain").unwrap(),
            opts: &mesh_opts,
        };

        // TODO: add a texture quad to the top left corner of experience (75x75) and render
        // refraction texture to it

        renderable_mesh.render(gl, state, assets, mesh_shader);
    }

    fn render_water(&self, gl: &WebGlRenderingContext, state: &State, assets: &Assets) {
        self.render_refraction(gl, state, assets);
        self.render_reflection(gl, state, assets);

        //        let Framebuffer { framebuffer, .. } = &self.refraction_framebuffer;
        //        gl.bind_framebuffer(GL::FRAMEBUFFER, framebuffer.as_ref());
        //        self.render_reflection(gl, state, assets);

        gl.bind_framebuffer(GL::FRAMEBUFFER, None);

        let water_shader = self.shader_sys.get_shader(&ShaderKind::Water).unwrap();
        gl.use_program(Some(&water_shader.program));

        let water_tile = WaterTile::new();

        water_tile.render(gl, state, assets, water_shader);

        self.render_refraction_visual(gl, state, assets);
        self.render_reflection_visual(gl, state, assets);
    }

    fn render_refraction(&self, gl: &WebGlRenderingContext, state: &State, assets: &Assets) {
        let Framebuffer { framebuffer, .. } = &self.refraction_framebuffer;
        gl.bind_framebuffer(GL::FRAMEBUFFER, framebuffer.as_ref());

        gl.clear_color(0.53, 0.8, 0.98, 1.);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        // FIXME: Base distance on a water tile height variable -0.5
        let water_tile_y = 0.0;
        let clip_plane = [0., -1., 0., water_tile_y];

        self.render_meshes(gl, state, assets, clip_plane, false);
    }

    fn render_reflection(&self, gl: &WebGlRenderingContext, state: &State, assets: &Assets) {
        let Framebuffer { framebuffer, .. } = &self.reflection_framebuffer;
        gl.bind_framebuffer(GL::FRAMEBUFFER, framebuffer.as_ref());

        gl.clear_color(0.53, 0.8, 0.98, 1.);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        // FIXME: Base distance on a water tile height variable -0.5
        let water_tile_y = 0.0;
        let clip_plane = [0., 1., 0., -water_tile_y];

        self.render_meshes(gl, state, assets, clip_plane, true);
    }

    fn render_refraction_visual(&self, gl: &WebGlRenderingContext, state: &State, assets: &Assets) {
        let quad_shader = self
            .shader_sys
            .get_shader(&ShaderKind::TexturedQuad)
            .unwrap();
        gl.use_program(Some(&quad_shader.program));
        TexturedQuad::new(
            0,
            CANVAS_HEIGHT as u16,
            75,
            75,
            TextureUnit::Refraction as u8,
        )
        .render(gl, state, assets, quad_shader);
    }

    // FIXME: Normalize with code above... We're really just rendering a textured quad with a certain
    // texture unit so move this code to TexturedQuad...
    fn render_reflection_visual(&self, gl: &WebGlRenderingContext, state: &State, assets: &Assets) {
        let quad_shader = self
            .shader_sys
            .get_shader(&ShaderKind::TexturedQuad)
            .unwrap();
        gl.use_program(Some(&quad_shader.program));
        TexturedQuad::new(
            CANVAS_WIDTH as u16 - 75,
            CANVAS_HEIGHT as u16,
            75,
            75,
            TextureUnit::Reflection as u8,
        )
        .render(gl, state, assets, quad_shader);
    }
}

struct Framebuffer {
    framebuffer: Option<WebGlFramebuffer>,
    color_texture: Option<WebGlTexture>,
    depth_texture: Option<WebGlTexture>,
}

pub enum TextureUnit {
    Refraction = 0,
    Reflection = 1,
    Dudv = 2,
    NormalMap = 3,
    RefractionDepth = 4,
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
        }
    }
}

impl WebRenderer {
    // TODO: Breadcrumb -> if texture_unit is refraction we need to attach a depth texture
    fn create_refraction_framebuffer(gl: &WebGlRenderingContext) -> Result<Framebuffer, JsValue> {
        let framebuffer = gl.create_framebuffer();
        gl.bind_framebuffer(GL::FRAMEBUFFER, framebuffer.as_ref());

        let color_texture = gl.create_texture();
        gl.active_texture(TextureUnit::Refraction.get());
        gl.bind_texture(GL::TEXTURE_2D, color_texture.as_ref());

        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
            GL::TEXTURE_2D,
            0,
            GL::RGBA as i32,
            // FIXME: Play with different refratin and reflection sizes to see whwat looks good
            CANVAS_WIDTH,
            CANVAS_HEIGHT,
            0,
            GL::RGBA as u32,
            GL::UNSIGNED_BYTE,
            None,
        )?;

        let depth_texture = gl.create_texture();
        gl.active_texture(TextureUnit::RefractionDepth.get());
        gl.bind_texture(GL::TEXTURE_2D, depth_texture.as_ref());
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
            GL::TEXTURE_2D,
            0,
            GL::DEPTH_COMPONENT as i32,
            // FIXME: Play with different refratin and reflection sizes to see whwat looks good
            CANVAS_WIDTH,
            CANVAS_HEIGHT,
            0,
            GL::DEPTH_COMPONENT as u32,
            // FIXME: UNSIGNED_BYTE should be fine here since we don't need as much precision
            // since it doesn't matter if there are two objects next to eachother and our
            // depth is very slightly off. Precision is more important in shadow mapping
            GL::UNSIGNED_SHORT,
            None,
        )?;

        gl.framebuffer_texture_2d(
            GL::FRAMEBUFFER,
            GL::COLOR_ATTACHMENT0,
            GL::TEXTURE_2D,
            color_texture.as_ref(),
            0,
        );

        gl.framebuffer_texture_2d(
            GL::FRAMEBUFFER,
            GL::DEPTH_ATTACHMENT,
            GL::TEXTURE_2D,
            depth_texture.as_ref(),
            0,
        );

        gl.bind_framebuffer(GL::FRAMEBUFFER, None);
        //                gl.bind_texture(GL::TEXTURE_2D, None);

        Ok(Framebuffer {
            framebuffer,
            color_texture,
            depth_texture,
        })
    }

    // FIXME: Normalize with refraction framebuffer
    fn create_reflection_framebuffer(gl: &WebGlRenderingContext) -> Result<Framebuffer, JsValue> {
        let framebuffer = gl.create_framebuffer();
        gl.bind_framebuffer(GL::FRAMEBUFFER, framebuffer.as_ref());

        let color_texture = gl.create_texture();

        gl.active_texture(TextureUnit::Reflection.get());
        gl.bind_texture(GL::TEXTURE_2D, color_texture.as_ref());
        // FIXME: Confirm that these are the proper settings and understand why
        // FIXME: Constant for canvas width and height that we get from the canvas module
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
            GL::TEXTURE_2D,
            0,
            GL::RGBA as i32,
            CANVAS_WIDTH,
            CANVAS_HEIGHT,
            0,
            GL::RGBA as u32,
            GL::UNSIGNED_BYTE,
            None,
        )?;

        // FIXME: Research render buffer so that I understand it and can describe it in comments.
        // Same with pretty much every WebGL API that we call
        let renderbuffer = gl.create_renderbuffer();
        gl.bind_renderbuffer(GL::RENDERBUFFER, renderbuffer.as_ref());
        gl.renderbuffer_storage(
            GL::RENDERBUFFER,
            GL::DEPTH_COMPONENT16,
            CANVAS_WIDTH,
            CANVAS_HEIGHT,
        );

        gl.framebuffer_texture_2d(
            GL::FRAMEBUFFER,
            GL::COLOR_ATTACHMENT0,
            GL::TEXTURE_2D,
            color_texture.as_ref(),
            0,
        );
        gl.framebuffer_renderbuffer(
            GL::FRAMEBUFFER,
            GL::DEPTH_ATTACHMENT,
            GL::RENDERBUFFER,
            renderbuffer.as_ref(),
        );

        gl.bind_renderbuffer(GL::RENDERBUFFER, None);
        gl.bind_framebuffer(GL::FRAMEBUFFER, None);
        //                gl.bind_texture(GL::TEXTURE_2D, None);

        Ok(Framebuffer {
            framebuffer,
            color_texture,
            depth_texture: None,
        })
    }
}

pub trait Render {
    fn shader_kind() -> ShaderKind;

    fn render(&self, gl: &GL, state: &State, assets: &Assets, shader: &Shader);

    // FIXME: Rename and normalize with other funcs.. move to Render trait.. Actually just
    // create the VAOs at the beginning of the application
    fn buffer_f32_data(gl: &GL, data: &[f32], attrib: u32, size: i32) {
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let data_location = data.as_ptr() as u32 / 4;

        let data_array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(data_location, data_location + data.len() as u32);

        // TODO: Do this outside of the loop using a vertex array object. We don't
        // need to repeatedly buffer this.. Do this before moving on to rendering the
        // water.
        let buffer = gl.create_buffer().unwrap();

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &data_array, GL::STATIC_DRAW);
        gl.vertex_attrib_pointer_with_i32(attrib, size, GL::FLOAT, false, 0, 0);
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

struct WaterTile {}

impl WaterTile {
    pub fn new() -> WaterTile {
        WaterTile {}
    }
}
