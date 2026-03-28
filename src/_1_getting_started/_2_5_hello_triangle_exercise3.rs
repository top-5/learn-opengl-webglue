#![allow(non_upper_case_globals)]
extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;
use self::gl::types::*;

use std::cell::RefCell;
use std::sync::mpsc::Receiver;
use std::ffi::CString;
use std::ptr;
use std::str;
use std::mem;
use std::os::raw::c_void;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

const vertexShaderSource: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const fragmentShaderSource: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
       FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

const fragmentShader2Source: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
       FragColor = vec4(1.0f, 1.0f, 0.0f, 1.0f);
    }
"#;

struct State_2_5 {
    shader_program_orange: GLuint,
    shader_program_yellow: GLuint,
    vaos: [GLuint; 2],
    vbos: [GLuint; 2],
}

thread_local! {
    static STATE: RefCell<Option<State_2_5>> = RefCell::new(None);
}

unsafe fn reset_2_5() {
    STATE.with(|state| {
        if let Some(s) = state.borrow_mut().take() {
            gl::DeleteVertexArrays(2, s.vaos.as_ptr());
            gl::DeleteBuffers(2, s.vbos.as_ptr());
            gl::DeleteProgram(s.shader_program_orange);
            gl::DeleteProgram(s.shader_program_yellow);
        }
    });
}

unsafe fn init_2_5() {
    // Compile shaders
    let vertexShader = gl::CreateShader(gl::VERTEX_SHADER);
    let fragmentShaderOrange = gl::CreateShader(gl::FRAGMENT_SHADER);
    let fragmentShaderYellow = gl::CreateShader(gl::FRAGMENT_SHADER);

    let c_str_vert = CString::new(vertexShaderSource.as_bytes()).unwrap();
    gl::ShaderSource(vertexShader, 1, &c_str_vert.as_ptr(), ptr::null());
    gl::CompileShader(vertexShader);

    let c_str_frag_orange = CString::new(fragmentShaderSource.as_bytes()).unwrap();
    gl::ShaderSource(fragmentShaderOrange, 1, &c_str_frag_orange.as_ptr(), ptr::null());
    gl::CompileShader(fragmentShaderOrange);

    let c_str_frag_yellow = CString::new(fragmentShader2Source.as_bytes()).unwrap();
    gl::ShaderSource(fragmentShaderYellow, 1, &c_str_frag_yellow.as_ptr(), ptr::null());
    gl::CompileShader(fragmentShaderYellow);

    // Link first program (orange)
    let shaderProgramOrange = gl::CreateProgram();
    gl::AttachShader(shaderProgramOrange, vertexShader);
    gl::AttachShader(shaderProgramOrange, fragmentShaderOrange);
    gl::LinkProgram(shaderProgramOrange);

    // Link second program (yellow)
    let shaderProgramYellow = gl::CreateProgram();
    gl::AttachShader(shaderProgramYellow, vertexShader);
    gl::AttachShader(shaderProgramYellow, fragmentShaderYellow);
    gl::LinkProgram(shaderProgramYellow);

    // Vertex data
    let firstTriangle: [f32; 9] = [
        -0.9, -0.5, 0.0,
        -0.0, -0.5, 0.0,
        -0.45, 0.5, 0.0,
    ];
    let secondTriangle: [f32; 9] = [
        0.0, -0.5, 0.0,
        0.9, -0.5, 0.0,
        0.45, 0.5, 0.0
    ];

    let (mut VBOs, mut VAOs) = ([0, 0], [0, 0]);
    gl::GenVertexArrays(2, VAOs.as_mut_ptr());
    gl::GenBuffers(2, VBOs.as_mut_ptr());

    // First triangle setup
    gl::BindVertexArray(VAOs[0]);
    gl::BindBuffer(gl::ARRAY_BUFFER, VBOs[0]);
    gl::BufferData(gl::ARRAY_BUFFER,
                   (firstTriangle.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                   &firstTriangle[0] as *const f32 as *const c_void,
                   gl::STATIC_DRAW);
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
    gl::EnableVertexAttribArray(0);

    // Second triangle setup
    gl::BindVertexArray(VAOs[1]);
    gl::BindBuffer(gl::ARRAY_BUFFER, VBOs[1]);
    gl::BufferData(gl::ARRAY_BUFFER,
                   (secondTriangle.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                   &secondTriangle[0] as *const f32 as *const c_void,
                   gl::STATIC_DRAW);
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
    gl::EnableVertexAttribArray(0);

    STATE.with(|state| {
        *state.borrow_mut() = Some(State_2_5 {
            shader_program_orange: shaderProgramOrange,
            shader_program_yellow: shaderProgramYellow,
            vaos: VAOs,
            vbos: VBOs,
        });
    });
}

unsafe fn render_2_5() {
    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);

    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            // Draw first triangle with orange shader
            gl::UseProgram(s.shader_program_orange);
            gl::BindVertexArray(s.vaos[0]);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            
            // Draw second triangle with yellow shader
            gl::UseProgram(s.shader_program_yellow);
            gl::BindVertexArray(s.vaos[1]);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    });
}

#[allow(non_snake_case)]
pub fn main_1_2_5() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe { init_2_5(); }
        }
    });
    unsafe { render_2_5(); }
}
