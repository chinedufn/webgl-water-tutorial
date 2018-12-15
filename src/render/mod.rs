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
use wasm_bindgen::JsValue;

pub struct WebRenderer {
    shader_sys: ShaderSystem,
}

impl WebRenderer {
    pub fn new(gl: &WebGlRenderingContext) -> WebRenderer {
        let shader_sys = ShaderSystem::new(&gl);

        WebRenderer { shader_sys }
    }

    pub fn render(&self, gl: &WebGlRenderingContext, state: &State, assets: &Assets) {
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

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

    fn create_refraction_framebuffer(
        &self,
        gl: &WebGlRenderingContext,
    ) -> Result<Option<WebGlFramebuffer>, JsValue> {
        let framebuffer = gl.create_framebuffer();
        gl.bind_framebuffer(GL::FRAMEBUFFER, framebuffer.as_ref());

        let texture = gl.create_texture();
        gl.bind_texture(GL::TEXTURE_2D, texture.as_ref());

        // FIXME: Confirm that these are the proper settings and understand why
        // FIXME: Constant for canvas width and height that we get from the canvas module
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
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
            texture.as_ref(),
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
        gl.bind_texture(GL::TEXTURE_2D, None);

        Ok(framebuffer)
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
