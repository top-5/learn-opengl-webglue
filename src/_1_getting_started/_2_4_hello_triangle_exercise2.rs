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

struct State_2_4 {
    shader_program: GLuint,
    vaos: [GLuint; 2],
    vbos: [GLuint; 2],
}

thread_local! {
    static STATE: RefCell<Option<State_2_4>> = RefCell::new(None);
}

unsafe fn reset_2_4() {
    STATE.with(|state| {
        if let Some(s) = state.borrow_mut().take() {
            gl::DeleteVertexArrays(2, s.vaos.as_ptr());
            gl::DeleteBuffers(2, s.vbos.as_ptr());
            gl::DeleteProgram(s.shader_program);
        }
    });
}

unsafe fn init_2_4() {
    // Compile vertex shader
    let vertexShader = gl::CreateShader(gl::VERTEX_SHADER);
    let c_str_vert = CString::new(vertexShaderSource.as_bytes()).unwrap();
    gl::ShaderSource(vertexShader, 1, &c_str_vert.as_ptr(), ptr::null());
    gl::CompileShader(vertexShader);

    let mut success = gl::FALSE as GLint;
    let mut infoLog = Vec::with_capacity(512);
    infoLog.set_len(512 - 1);
    gl::GetShaderiv(vertexShader, gl::COMPILE_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetShaderInfoLog(vertexShader, 512, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
        println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", String::from_utf8_lossy(&infoLog));
    }

    // Compile fragment shader
    let fragmentShader = gl::CreateShader(gl::FRAGMENT_SHADER);
    let c_str_frag = CString::new(fragmentShaderSource.as_bytes()).unwrap();
    gl::ShaderSource(fragmentShader, 1, &c_str_frag.as_ptr(), ptr::null());
    gl::CompileShader(fragmentShader);
    gl::GetShaderiv(fragmentShader, gl::COMPILE_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetShaderInfoLog(fragmentShader, 512, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
        println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", String::from_utf8_lossy(&infoLog));
    }

    // Link shader program
    let shaderProgram = gl::CreateProgram();
    gl::AttachShader(shaderProgram, vertexShader);
    gl::AttachShader(shaderProgram, fragmentShader);
    gl::LinkProgram(shaderProgram);
    gl::GetProgramiv(shaderProgram, gl::LINK_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetProgramInfoLog(shaderProgram, 512, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
        println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", String::from_utf8_lossy(&infoLog));
    }
    gl::DeleteShader(vertexShader);
    gl::DeleteShader(fragmentShader);

    // Vertex data for two triangles
    let firstTriangle: [f32; 9] = [
        -0.9, -0.5, 0.0,  // left
        -0.0, -0.5, 0.0,  // right
        -0.45, 0.5, 0.0,  // top
    ];
    let secondTriangle: [f32; 9] = [
        0.0, -0.5, 0.0,  // left
        0.9, -0.5, 0.0,  // right
        0.45, 0.5, 0.0   // top
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
        *state.borrow_mut() = Some(State_2_4 {
            shader_program: shaderProgram,
            vaos: VAOs,
            vbos: VBOs,
        });
    });
}

unsafe fn render_2_4() {
    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);

    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            gl::UseProgram(s.shader_program);
            // Draw first triangle
            gl::BindVertexArray(s.vaos[0]);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            // Draw second triangle
            gl::BindVertexArray(s.vaos[1]);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    });
}

#[allow(non_snake_case)]
pub fn main_1_2_4() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe { init_2_4(); }
        }
    });
    unsafe { render_2_4(); }
}
