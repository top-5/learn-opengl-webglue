#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use std::cell::RefCell;
use std::ptr;
use std::mem;
use std::os::raw::c_void;
use gl::types::*;

use cgmath::Vector2;

extern crate num;
use self::num::range_step;

use shader::Shader;
use camera::Camera;

const SCR_WIDTH: u32 = 1280;
const SCR_HEIGHT: u32 = 720;

struct State_4_10_1 {
    shader: Shader,
    quadVAO: GLuint,
    quadVBO: GLuint,
    instanceVBO: GLuint,
}

thread_local! {
    static STATE: RefCell<Option<State_4_10_1>> = RefCell::new(None);
}

unsafe fn reset_4_10_1() {
    STATE.with(|state| {
        if let Some(s) = state.borrow_mut().take() {
            gl::DeleteVertexArrays(1, &s.quadVAO);
            gl::DeleteBuffers(1, &s.quadVBO);
            gl::DeleteBuffers(1, &s.instanceVBO);
        }
    });
}

unsafe fn init_4_10_1() {
    gl::Enable(gl::DEPTH_TEST);

    let shader = Shader::new(
        "src/_4_advanced_opengl/shaders/10.1.instancing.vs",
        "src/_4_advanced_opengl/shaders/10.1.instancing.fs",
    );

    // generate 100 quad translations
        let mut translations = vec![];
        let offset = 0.1;
        for y in range_step(-10, 10, 2) {
            for x in range_step(-10, 10, 2) {
                translations.push(
                    Vector2 {
                        x: x as i32 as f32 / 10.0 + offset,
                        y: y as i32 as f32 / 10.0 + offset
                    }
                )
            }
        }

        let mut instanceVBO = 0;
        gl::GenBuffers(1, &mut instanceVBO);
        gl::BindBuffer(gl::ARRAY_BUFFER, instanceVBO);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            mem::size_of::<Vector2<f32>>() as isize * 100 ,
            &translations[0] as *const Vector2<f32> as *const c_void,
            gl::STATIC_DRAW);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        let quadVertices: [f32; 30] = [
            // positions   // colors
            -0.05,  0.05,  1.0, 0.0, 0.0,
             0.05, -0.05,  0.0, 1.0, 0.0,
            -0.05, -0.05,  0.0, 0.0, 1.0,

            -0.05,  0.05,  1.0, 0.0, 0.0,
             0.05, -0.05,  0.0, 1.0, 0.0,
             0.05,  0.05,  0.0, 1.0, 1.0
        ];
        let (mut quadVAO, mut quadVBO) = (0, 0);
        gl::GenVertexArrays(1, &mut quadVAO);
        gl::GenBuffers(1, &mut quadVBO);
        gl::BindVertexArray(quadVAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, quadVBO);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (quadVertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &quadVertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);
        gl::EnableVertexAttribArray(0);
        let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (2 * mem::size_of::<GLfloat>()) as *const c_void);
        // also set instance data
        gl::EnableVertexAttribArray(2);
        gl::BindBuffer(gl::ARRAY_BUFFER, instanceVBO); // this attribute comes from a different vertex buffer
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, (2 * mem::size_of::<GLfloat>()) as i32, ptr::null());
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::VertexAttribDivisor(2, 1); // tell OpenGL this is an instanced vertex attribute.

        STATE.with(|state| {
            *state.borrow_mut() = Some(State_4_10_1 {
                shader,
                quadVAO,
                quadVBO,
                instanceVBO,
            });
        });
}

unsafe fn render_4_10_1(_camera: &Camera) {
    gl::ClearColor(0.1, 0.1, 0.1, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            s.shader.useProgram();
            gl::BindVertexArray(s.quadVAO);
            gl::DrawArraysInstanced(gl::TRIANGLES, 0, 6, 100);
            gl::BindVertexArray(0);
        }
    });
}

pub fn main_4_10_1() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe {
                init_4_10_1();
            }
        }
    });

    unsafe {
        render_4_10_1(&Camera::default());
    }
}
