use crate::app::Assets;
use crate::app::State;
use crate::canvas::{CANVAS_HEIGHT, CANVAS_WIDTH};
use crate::render::Render;
use crate::shader::Shader;
use crate::shader::ShaderKind;
// FIXME: Remove all of the * and instead import exactly what we need so reader can learn
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub struct TexturedQuad<'a> {
    /// Left most part of canvas is 0, rightmost is CANVAS_WIDTH
    left: u16,
    /// Bottom of canvas is 0, top is CANVAS_HEIGHT
    top: u16,
    /// How many pixels wide
    width: u16,
    /// How many pixels tall
    height: u16,
    /// The texture unit to use
    texture_unit: u8,
    /// The shader to use when rendering
    shader: &'a Shader,
}

impl<'a> TexturedQuad<'a> {
    pub fn new(
        left: u16,
        top: u16,
        width: u16,
        height: u16,
        texture_unit: u8,
        shader: &Shader,
    ) -> TexturedQuad {
        TexturedQuad {
            left,
            top,
            width,
            height,
            texture_unit,
            shader,
        }
    }
}

impl<'a> Render<'a> for TexturedQuad<'a> {
    fn shader_kind() -> ShaderKind {
        ShaderKind::TexturedQuad
    }

    fn shader(&'a self) -> &'a Shader {
        &self.shader
    }

    fn render(&self, gl: &WebGlRenderingContext, state: &State, assets: &Assets) {
        let shader = self.shader();

        let vertex_data = self.make_textured_quad_vertices(CANVAS_WIDTH, CANVAS_HEIGHT);

        gl.uniform1i(
            shader.get_uniform_location(gl, "texture").as_ref(),
            self.texture_unit as i32,
        );

        let vertex_data_attrib = gl.get_attrib_location(&shader.program, "vertexData");
        gl.enable_vertex_attrib_array(vertex_data_attrib as u32);

        // FIXME: This repeatedly creates new buffers. Not what we want. Use VAOs
        TexturedQuad::buffer_f32_data(&gl, &vertex_data[..], vertex_data_attrib as u32, 4);

        gl.draw_arrays(GL::TRIANGLES, 0, 6);
    }
}

impl<'a> TexturedQuad<'a> {
    // Combine our vertex data so that we can pass one array to the GPU
    fn make_textured_quad_vertices(&self, viewport_width: i32, viewport_height: i32) -> Vec<f32> {
        let viewport_width = viewport_width as f32;
        let viewport_height = viewport_height as f32;

        let left_x = self.left as f32 / viewport_width;
        let top_y = self.top as f32 / viewport_height;
        let right_x = (self.left as f32 + self.width as f32) / viewport_width;
        let bottom_y = (self.top as f32 - self.height as f32) / viewport_height;

        let left_x = 2.0 * left_x - 1.0;
        let right_x = 2.0 * right_x - 1.0;

        let bottom_y = 2.0 * bottom_y - 1.0;
        let top_y = 2.0 * top_y - 1.0;

        // All of the positions of our quad in screen space
        let positions = [
            left_x, top_y, // Top Left
            right_x, bottom_y, // Bottom Right
            left_x, bottom_y, // Bottom Left
            left_x, top_y, // Top Left
            right_x, top_y, // Top Right
            right_x, bottom_y, // Bottom Right
        ];

        let texture_coords = [
            0., 1., // Top left
            1., 0., // Bottom Right
            0., 0., // Bottom Left
            0., 1., // Top Left
            1., 1., // Top Right
            1., 0., // Bottom Right
        ];

        let mut vertices = vec![];

        for i in 0..positions.len() {
            // Skip odd indices
            if i % 2 == 1 {
                continue;
            }

            vertices.push(positions[i]);
            vertices.push(positions[i + 1]);
            vertices.push(texture_coords[i]);
            vertices.push(texture_coords[i + 1]);
        }

        vertices
    }
}
