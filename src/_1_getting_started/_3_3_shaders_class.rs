#![allow(non_upper_case_globals)]
extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;
use self::gl::types::*;

use std::cell::RefCell;
use std::sync::mpsc::Receiver;
use std::ptr;
use std::mem;
use std::os::raw::c_void;

use shader::Shader;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

struct State_3_3 {
    shader: Shader,
    vao: GLuint,
    vbo: GLuint,
}

thread_local! {
    static STATE: RefCell<Option<State_3_3>> = RefCell::new(None);
}

unsafe fn reset_3_3() {
    STATE.with(|state| {
        if let Some(s) = state.borrow_mut().take() {
            gl::DeleteVertexArrays(1, &s.vao);
            gl::DeleteBuffers(1, &s.vbo);
        }
    });
}

unsafe fn init_3_3() {
    let ourShader = Shader::new(
        "src/_1_getting_started/shaders/3.3.shader.vs",
        "src/_1_getting_started/shaders/3.3.shader.fs"
    );

    // Vertex data with positions and colors
    let vertices: [f32; 18] = [
        // positions         // colors
        0.5, -0.5, 0.0,  1.0, 0.0, 0.0,  // bottom right
       -0.5, -0.5, 0.0,  0.0, 1.0, 0.0,  // bottom left
        0.0,  0.5, 0.0,  0.0, 0.0, 1.0   // top
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

    let stride = 6 * mem::size_of::<GLfloat>() as GLsizei;
    // Position attribute
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    gl::EnableVertexAttribArray(0);
    // Color attribute
    gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
    gl::EnableVertexAttribArray(1);

    STATE.with(|state| {
        *state.borrow_mut() = Some(State_3_3 {
            shader: ourShader,
            vao: VAO,
            vbo: VBO,
        });
    });
}

unsafe fn render_3_3() {
    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);

    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            s.shader.useProgram();
            gl::BindVertexArray(s.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    });
}

#[allow(non_snake_case)]
pub fn main_1_3_3() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe { init_3_3(); }
        }
    });
    unsafe { render_3_3(); }
}
