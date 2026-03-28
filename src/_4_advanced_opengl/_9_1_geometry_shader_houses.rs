#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use std::cell::RefCell;
use std::ptr;
use std::mem;
use std::os::raw::c_void;

extern crate glfw;

extern crate gl;
use self::gl::types::*;

use shader::Shader;
use camera::Camera;

// settings
const SCR_WIDTH: u32 = 1280;
const SCR_HEIGHT: u32 = 720;

struct State_4_9_1 {
    shader: Shader,
    vao: GLuint,
}

thread_local! {
    static STATE: RefCell<Option<State_4_9_1>> = RefCell::new(None);
}

unsafe fn reset_4_9_1() {
    STATE.with(|state| {
        if let Some(s) = state.borrow_mut().take() {
            gl::DeleteVertexArrays(1, &s.vao);
        }
    });
}

unsafe fn init_4_9_1() {
        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);

        // build and compile shaders
        // -------------------------
        let shader = Shader::with_geometry_shader(
            "src/_4_advanced_opengl/shaders/9.1.geometry_shader.vs",
            "src/_4_advanced_opengl/shaders/9.1.geometry_shader.fs",
            "src/_4_advanced_opengl/shaders/9.1.geometry_shader.gs"
        );

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        let points: [f32; 20] = [
            -0.5,  0.5, 1.0, 0.0, 0.0, // top-left
             0.5,  0.5, 0.0, 1.0, 0.0, // top-right
             0.5, -0.5, 0.0, 0.0, 1.0, // bottom-right
            -0.5, -0.5, 1.0, 1.0, 0.0  // bottom-left
        ];
        // cube VAO
        let (mut VAO, mut VBO) = (0, 0);
        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);
        gl::BindVertexArray(VAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (points.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &points[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);
        let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (2 * mem::size_of::<GLfloat>()) as *const f32 as *const c_void);
        gl::BindVertexArray(0);

    STATE.with(|state| {
        *state.borrow_mut() = Some(State_4_9_1 {
            shader,
            vao: VAO,
        });
    });
}

unsafe fn render_4_9_1(_camera: &Camera) {
    gl::ClearColor(0.1, 0.1, 0.1, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            s.shader.useProgram();
            gl::BindVertexArray(s.vao);
            gl::DrawArrays(gl::POINTS, 0, 4);
        }
    });
}

pub fn main_4_9_1() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe { init_4_9_1(); }
        }
    });
    unsafe { render_4_9_1(&Camera::default()); }
}
