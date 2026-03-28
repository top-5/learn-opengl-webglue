#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use std::cell::RefCell;

extern crate glfw;

extern crate gl;
use self::gl::types::*;

use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::ffi::CStr;

use common::loadTexture;
use shader::Shader;
use camera::Camera;

use cgmath::{Matrix4, vec3, Deg, perspective};
use cgmath::prelude::*;

// settings
const SCR_WIDTH: u32 = 1280;
const SCR_HEIGHT: u32 = 720;

struct State_4_2 {
    shader: Shader,
    shaderSingleColor: Shader,
    cubeVAO: GLuint,
    cubeVBO: GLuint,
    planeVAO: GLuint,
    planeVBO: GLuint,
    cubeTexture: GLuint,
    floorTexture: GLuint,
}

thread_local! {
    static STATE: RefCell<Option<State_4_2>> = RefCell::new(None);
}

unsafe fn reset_4_2() {
    STATE.with(|state| {
        if let Some(s) = state.borrow_mut().take() {
            gl::DeleteVertexArrays(1, &s.cubeVAO);
            gl::DeleteVertexArrays(1, &s.planeVAO);
            gl::DeleteBuffers(1, &s.cubeVBO);
            gl::DeleteBuffers(1, &s.planeVBO);
            gl::DeleteTextures(1, &s.cubeTexture);
            gl::DeleteTextures(1, &s.floorTexture);
        }
    });
}

unsafe fn init_4_2() {
    // configure global opengl state
    gl::Enable(gl::DEPTH_TEST);
    gl::DepthFunc(gl::LESS);
    gl::Enable(gl::STENCIL_TEST);
    gl::StencilFunc(gl::NOTEQUAL, 1, 0xFF);
    gl::StencilOp(gl::KEEP, gl::KEEP, gl::REPLACE);

    // build and compile our shader program
    let shader = Shader::new(
        "src/_4_advanced_opengl/shaders/2.stencil_testing.vs",
        "src/_4_advanced_opengl/shaders/2.stencil_testing.fs");
    let shaderSingleColor = Shader::new(
        "src/_4_advanced_opengl/shaders/2.stencil_testing.vs",
        "src/_4_advanced_opengl/shaders/2.stencil_single_color.fs");

    // set up vertex data (and buffer(s)) and configure vertex attributes
    let cubeVertices: [f32; 180] = [
         // positions       // texture Coords
         -0.5, -0.5, -0.5,  0.0, 0.0,
          0.5, -0.5, -0.5,  1.0, 0.0,
          0.5,  0.5, -0.5,  1.0, 1.0,
          0.5,  0.5, -0.5,  1.0, 1.0,
         -0.5,  0.5, -0.5,  0.0, 1.0,
         -0.5, -0.5, -0.5,  0.0, 0.0,

         -0.5, -0.5,  0.5,  0.0, 0.0,
          0.5, -0.5,  0.5,  1.0, 0.0,
          0.5,  0.5,  0.5,  1.0, 1.0,
          0.5,  0.5,  0.5,  1.0, 1.0,
         -0.5,  0.5,  0.5,  0.0, 1.0,
         -0.5, -0.5,  0.5,  0.0, 0.0,

         -0.5,  0.5,  0.5,  1.0, 0.0,
         -0.5,  0.5, -0.5,  1.0, 1.0,
         -0.5, -0.5, -0.5,  0.0, 1.0,
         -0.5, -0.5, -0.5,  0.0, 1.0,
         -0.5, -0.5,  0.5,  0.0, 0.0,
         -0.5,  0.5,  0.5,  1.0, 0.0,

          0.5,  0.5,  0.5,  1.0, 0.0,
          0.5,  0.5, -0.5,  1.0, 1.0,
          0.5, -0.5, -0.5,  0.0, 1.0,
          0.5, -0.5, -0.5,  0.0, 1.0,
          0.5, -0.5,  0.5,  0.0, 0.0,
          0.5,  0.5,  0.5,  1.0, 0.0,

         -0.5, -0.5, -0.5,  0.0, 1.0,
          0.5, -0.5, -0.5,  1.0, 1.0,
          0.5, -0.5,  0.5,  1.0, 0.0,
          0.5, -0.5,  0.5,  1.0, 0.0,
         -0.5, -0.5,  0.5,  0.0, 0.0,
         -0.5, -0.5, -0.5,  0.0, 1.0,

         -0.5,  0.5, -0.5,  0.0, 1.0,
          0.5,  0.5, -0.5,  1.0, 1.0,
          0.5,  0.5,  0.5,  1.0, 0.0,
          0.5,  0.5,  0.5,  1.0, 0.0,
         -0.5,  0.5,  0.5,  0.0, 0.0,
         -0.5,  0.5, -0.5,  0.0, 1.0
    ];
    let planeVertices: [f32; 30] = [
        // positions       // texture Coords
         5.0, -0.5,  5.0,  2.0, 0.0,
        -5.0, -0.5,  5.0,  0.0, 0.0,
        -5.0, -0.5, -5.0,  0.0, 2.0,

         5.0, -0.5,  5.0,  2.0, 0.0,
        -5.0, -0.5, -5.0,  0.0, 2.0,
         5.0, -0.5, -5.0,  2.0, 2.0
    ];
    // cube VAO
    let (mut cubeVAO, mut cubeVBO) = (0, 0);
    gl::GenVertexArrays(1, &mut cubeVAO);
    gl::GenBuffers(1, &mut cubeVBO);
    gl::BindVertexArray(cubeVAO);
    gl::BindBuffer(gl::ARRAY_BUFFER, cubeVBO);
    gl::BufferData(gl::ARRAY_BUFFER,
                   (cubeVertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                   &cubeVertices[0] as *const f32 as *const c_void,
                   gl::STATIC_DRAW);
    let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;
    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    gl::EnableVertexAttribArray(1);
    gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
    gl::BindVertexArray(0);
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
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    gl::EnableVertexAttribArray(1);
    gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
    gl::BindVertexArray(0);

    // load textures
    let cubeTexture = loadTexture("resources/textures/marble.jpg");
    let floorTexture = loadTexture("resources/textures/metal.png");

    // shader configuration
    shader.useProgram();
    shader.setInt(c_str!("texture1"), 0);

    STATE.with(|state| {
        *state.borrow_mut() = Some(State_4_2 {
            shader,
            shaderSingleColor,
            cubeVAO,
            cubeVBO,
            planeVAO,
            planeVBO,
            cubeTexture,
            floorTexture,
        });
    });
}

unsafe fn render_4_2(camera: &Camera) {
    gl::ClearColor(0.1, 0.1, 0.1, 1.0);
    gl::Clear(
        gl::COLOR_BUFFER_BIT |
        gl::DEPTH_BUFFER_BIT |
        gl::STENCIL_BUFFER_BIT);

    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            // set uniforms
            let view = camera.GetViewMatrix();
            let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            
            s.shaderSingleColor.useProgram();
            s.shaderSingleColor.setMat4(c_str!("view"), &view);
            s.shaderSingleColor.setMat4(c_str!("projection"), &projection);

            s.shader.useProgram();
            s.shader.setMat4(c_str!("view"), &view);
            s.shader.setMat4(c_str!("projection"), &projection);

            // draw floor as normal, but don't write to stencil buffer
            gl::StencilMask(0x00);
            gl::BindVertexArray(s.planeVAO);
            gl::BindTexture(gl::TEXTURE_2D, s.floorTexture);
            s.shader.setMat4(c_str!("model"), &Matrix4::identity());
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
            
            // 1st. render pass, draw objects as normal, writing to the stencil buffer
            gl::StencilFunc(gl::ALWAYS, 1, 0xFF);
            gl::StencilMask(0xFF);
            gl::BindVertexArray(s.cubeVAO);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, s.cubeTexture);
            let mut model = Matrix4::from_translation(vec3(-1.0, 0.0, -1.0));
            s.shader.setMat4(c_str!("model"), &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            model = Matrix4::from_translation(vec3(2.0, 0.0, 0.0));
            s.shader.setMat4(c_str!("model"), &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            // 2nd. render pass: draw slightly scaled versions (outline effect)
            gl::StencilFunc(gl::NOTEQUAL, 1, 0xFF);
            gl::StencilMask(0x00);
            gl::Disable(gl::DEPTH_TEST);
            s.shaderSingleColor.useProgram();
            let scale = 1.1;
            gl::BindVertexArray(s.cubeVAO);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, s.cubeTexture);
            model = Matrix4::from_translation(vec3(-1.0, 0.0, -1.0));
            model = model * Matrix4::from_scale(scale);
            s.shaderSingleColor.setMat4(c_str!("model"), &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            model = Matrix4::from_translation(vec3(2.0, 0.0, 0.0));
            model = model * Matrix4::from_scale(scale);
            s.shaderSingleColor.setMat4(c_str!("model"), &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            gl::BindVertexArray(0);
            gl::StencilMask(0xFF);
            gl::Enable(gl::DEPTH_TEST);
        }
    });
}

pub fn main_4_2() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe { init_4_2(); }
        }
    });
    unsafe { render_4_2(&Camera::default()); }
}
