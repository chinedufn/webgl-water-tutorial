use nalgebra;
use nalgebra::{Isometry3, Matrix4, Perspective3, Point3, Vector3};
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub struct ShaderSystem {
    programs: HashMap<ShaderKind, Shader>,
}

impl ShaderSystem {
    pub fn new(gl: &WebGlRenderingContext) -> ShaderSystem {
        let mut programs = HashMap::new();

        programs.insert(
            ShaderKind::Water,
            Shader::new(
                &gl,
                include_str!("./water-vertex.glsl"),
                include_str!("./water-fragment.glsl"),
            )
            .unwrap(),
        );
        programs.insert(
            ShaderKind::Mesh,
            Shader::new(
                &gl,
                include_str!("./mesh-vertex.glsl"),
                include_str!("./mesh-fragment.glsl"),
            )
            .unwrap(),
        );
        programs.insert(
            ShaderKind::TexturedQuad,
            Shader::new(
                &gl,
                include_str!("./textured-quad-vertex.glsl"),
                include_str!("./textured-quad-fragment.glsl"),
            )
            .unwrap(),
        );

        ShaderSystem { programs }
    }

    pub fn get_shader(&self, shader_kind: &ShaderKind) -> Option<&Shader> {
        self.programs.get(shader_kind)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ShaderKind {
    Water,
    Mesh,
    TexturedQuad,
}

pub struct Shader {
    pub program: WebGlProgram,
}

impl Shader {
    /// Create a new Shader program from a vertex and fragment shader
    fn new(
        gl: &WebGlRenderingContext,
        vert_shader: &str,
        frag_shader: &str,
    ) -> Result<Shader, JsValue> {
        let vert_shader = compile_shader(&gl, WebGlRenderingContext::VERTEX_SHADER, vert_shader)?;
        let frag_shader = compile_shader(&gl, WebGlRenderingContext::FRAGMENT_SHADER, frag_shader)?;
        let program = link_program(&gl, &vert_shader, &frag_shader)?;

        Ok(Shader { program })
    }
}

/// Create a shader program using the WebGL APIs
fn compile_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| "Could not create shader".to_string())?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error creating shader".to_string()))
    }
}

/// Link a shader program using the WebGL APIs
fn link_program(
    gl: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| "Unable to create shader program".to_string())?;

    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);

    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| "Unknown error creating program".to_string()))
    }
}
