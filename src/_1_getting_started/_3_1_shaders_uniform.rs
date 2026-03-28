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
    uniform vec4 ourColor;
    void main() {
       FragColor = ourColor;
    }
"#;

struct State_3_1 {
    shader_program: GLuint,
    vao: GLuint,
    vbo: GLuint,
    vertex_color_location: GLint,
}

thread_local! {
    static STATE: RefCell<Option<State_3_1>> = RefCell::new(None);
}

unsafe fn reset_3_1() {
    STATE.with(|state| {
        if let Some(s) = state.borrow_mut().take() {
            gl::DeleteVertexArrays(1, &s.vao);
            gl::DeleteBuffers(1, &s.vbo);
            gl::DeleteProgram(s.shader_program);
        }
    });
}

unsafe fn init_3_1() {
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

    // Vertex data
    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0, // left
         0.5, -0.5, 0.0, // right
         0.0,  0.5, 0.0  // top
    ];

    let (mut VBO, mut VAO) = (0, 0);
    gl::GenVertexArrays(1, &mut VAO);
    gl::GenBuffers(1, &mut VBO);
    gl::BindVertexArray(VAO);

    gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
    gl::BufferData(gl::ARRAY_BUFFER,
                   (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                   &vertices[0] as *const f32 as *const c_void,
                   gl::STATIC_DRAW);

    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
    gl::EnableVertexAttribArray(0);

    // Get uniform location
    let ourColor = CString::new("ourColor").unwrap();
    let vertexColorLocation = gl::GetUniformLocation(shaderProgram, ourColor.as_ptr());

    STATE.with(|state| {
        *state.borrow_mut() = Some(State_3_1 {
            shader_program: shaderProgram,
            vao: VAO,
            vbo: VBO,
            vertex_color_location: vertexColorLocation,
        });
    });
}

unsafe fn render_3_1(time: f64) {
    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);

    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            gl::UseProgram(s.shader_program);
            
            // Update uniform - pulsing green color based on time
            let timeValue = time as f32;
            let greenValue = timeValue.sin() / 2.0 + 0.5;
            gl::Uniform4f(s.vertex_color_location, 0.0, greenValue, 0.0, 1.0);

            gl::BindVertexArray(s.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    });
}

#[allow(non_snake_case)]
pub fn main_1_3_1() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe { init_3_1(); }
        }
    });
    let time = glfw::get_time();
    unsafe { render_3_1(time); }
}
