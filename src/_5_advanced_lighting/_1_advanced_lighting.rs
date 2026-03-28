#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

extern crate gl;
use self::gl::types::*;

use std::cell::RefCell;
use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::ffi::CStr;

use common::loadTexture;
use shader::Shader;
use camera::Camera;

use cgmath::{Matrix4, vec3, Deg, perspective};
use cgmath::prelude::*;

const SCR_WIDTH: u32 = 1280;
const SCR_HEIGHT: u32 = 720;

struct State_5_1 {
    shader: Shader,
    planeVAO: u32,
    planeVBO: u32,
    floorTexture: u32,
    blinn: bool,
}

thread_local! {
    static STATE: RefCell<Option<State_5_1>> = RefCell::new(None);
}

unsafe fn reset_5_1() {
    STATE.with(|state| {
        if let Some(s) = state.borrow_mut().take() {
            gl::DeleteVertexArrays(1, &s.planeVAO);
            gl::DeleteBuffers(1, &s.planeVBO);
            gl::DeleteTextures(1, &s.floorTexture);
        }
    });
}

unsafe fn init_5_1() {
    gl::Enable(gl::DEPTH_TEST);
    gl::Enable(gl::BLEND);
    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

    let shader = Shader::new(
        "src/_5_advanced_lighting/shaders/1.advanced_lighting.vs",
        "src/_5_advanced_lighting/shaders/1.advanced_lighting.fs");

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        let planeVertices: [f32; 48] = [
            // positions         // normals      // texcoords
             10.0, -0.5,  10.0,  0.0, 1.0, 0.0,  10.0,  0.0,
            -10.0, -0.5,  10.0,  0.0, 1.0, 0.0,   0.0,  0.0,
            -10.0, -0.5, -10.0,  0.0, 1.0, 0.0,   0.0, 10.0,

             10.0, -0.5,  10.0,  0.0, 1.0, 0.0,  10.0,  0.0,
            -10.0, -0.5, -10.0,  0.0, 1.0, 0.0,   0.0, 10.0,
             10.0, -0.5, -10.0,  0.0, 1.0, 0.0,  10.0, 10.0
        ];
        // plane VAO
        let (mut planeVAO, mut planeVBO) = (0, 0);
        gl::GenVertexArrays(1, &mut planeVAO);
        gl::GenBuffers(1, &mut planeVBO);
        gl::BindVertexArray(planeVAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, planeVBO);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (planeVertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &planeVertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);
        gl::EnableVertexAttribArray(0);
        let stride = 8 * mem::size_of::<GLfloat>() as GLsizei;
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (6 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::BindVertexArray(0);

        let floorTexture = loadTexture("resources/textures/wood.png");

        shader.useProgram();
        shader.setInt(c_str!("texture1"), 0);

        STATE.with(|state| {
            *state.borrow_mut() = Some(State_5_1 {
                shader,
                planeVAO,
                planeVBO,
                floorTexture,
                blinn: true,
            });
        });
}

unsafe fn render_5_1(camera: &Camera) {
    gl::ClearColor(0.1, 0.1, 0.1, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            let lightPos = vec3(0.0, 0.0, 0.0);

            s.shader.useProgram();
            let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            let view = camera.GetViewMatrix();
            s.shader.setMat4(c_str!("projection"), &projection);
            s.shader.setMat4(c_str!("view"), &view);
            s.shader.setVector3(c_str!("viewPos"), &camera.Position.to_vec());
            s.shader.setVector3(c_str!("lightPos"), &lightPos);
            s.shader.setInt(c_str!("blinn"), s.blinn as i32);

            gl::BindVertexArray(s.planeVAO);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, s.floorTexture);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    });
}

pub fn main_5_1() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe { init_5_1(); }
        }
    });
    unsafe {
        render_5_1(&Camera::default());
    }
}
