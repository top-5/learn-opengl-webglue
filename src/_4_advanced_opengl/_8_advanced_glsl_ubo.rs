#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use std::cell::RefCell;
use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::ffi::CStr;
use gl::types::*;

use cgmath::{Matrix4, vec3, Deg, perspective};
use cgmath::prelude::*;

use shader::Shader;
use camera::Camera;

const SCR_WIDTH: u32 = 1280;
const SCR_HEIGHT: u32 = 720;

struct State_4_8 {
    shaderRed: Shader,
    shaderGreen: Shader,
    shaderBlue: Shader,
    shaderYellow: Shader,
    cubeVAO: GLuint,
    cubeVBO: GLuint,
    uboMatrices: GLuint,
}

thread_local! {
    static STATE: RefCell<Option<State_4_8>> = RefCell::new(None);
}

unsafe fn reset_4_8() {
    STATE.with(|state| {
        if let Some(s) = state.borrow_mut().take() {
            gl::DeleteVertexArrays(1, &s.cubeVAO);
            gl::DeleteBuffers(1, &s.cubeVBO);
            gl::DeleteBuffers(1, &s.uboMatrices);
        }
    });
}

unsafe fn init_4_8() {
    gl::Enable(gl::DEPTH_TEST);

    let shaderRed = Shader::new("src/_4_advanced_opengl/shaders/8.advanced_glsl.vs", "src/_4_advanced_opengl/shaders/8.red.fs");
    let shaderGreen = Shader::new("src/_4_advanced_opengl/shaders/8.advanced_glsl.vs", "src/_4_advanced_opengl/shaders/8.green.fs");
    let shaderBlue = Shader::new("src/_4_advanced_opengl/shaders/8.advanced_glsl.vs", "src/_4_advanced_opengl/shaders/8.blue.fs");
    let shaderYellow = Shader::new("src/_4_advanced_opengl/shaders/8.advanced_glsl.vs", "src/_4_advanced_opengl/shaders/8.yellow.fs");

    // set up vertex data
        let cubeVertices: [f32; 108] = [
            // positions
            -0.5, -0.5, -0.5,
             0.5, -0.5, -0.5,
             0.5,  0.5, -0.5,
             0.5,  0.5, -0.5,
            -0.5,  0.5, -0.5,
            -0.5, -0.5, -0.5,

            -0.5, -0.5,  0.5,
             0.5, -0.5,  0.5,
             0.5,  0.5,  0.5,
             0.5,  0.5,  0.5,
            -0.5,  0.5,  0.5,
            -0.5, -0.5,  0.5,

            -0.5,  0.5,  0.5,
            -0.5,  0.5, -0.5,
            -0.5, -0.5, -0.5,
            -0.5, -0.5, -0.5,
            -0.5, -0.5,  0.5,
            -0.5,  0.5,  0.5,

             0.5,  0.5,  0.5,
             0.5,  0.5, -0.5,
             0.5, -0.5, -0.5,
             0.5, -0.5, -0.5,
             0.5, -0.5,  0.5,
             0.5,  0.5,  0.5,

            -0.5, -0.5, -0.5,
             0.5, -0.5, -0.5,
             0.5, -0.5,  0.5,
             0.5, -0.5,  0.5,
            -0.5, -0.5,  0.5,
            -0.5, -0.5, -0.5,

            -0.5,  0.5, -0.5,
             0.5,  0.5, -0.5,
             0.5,  0.5,  0.5,
             0.5,  0.5,  0.5,
            -0.5,  0.5,  0.5,
            -0.5,  0.5, -0.5,
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
        let stride = 3 * mem::size_of::<GLfloat>() as GLsizei;
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());

        // configure uniform buffer object
        let uniformBlockIndexRed = gl::GetUniformBlockIndex(shaderRed.ID, c_str!("Matrices").as_ptr());
        let uniformBlockIndexGreen = gl::GetUniformBlockIndex(shaderGreen.ID, c_str!("Matrices").as_ptr());
        let uniformBlockIndexBlue = gl::GetUniformBlockIndex(shaderBlue.ID, c_str!("Matrices").as_ptr());
        let uniformBlockIndexYellow = gl::GetUniformBlockIndex(shaderYellow.ID, c_str!("Matrices").as_ptr());

        gl::UniformBlockBinding(shaderRed.ID, uniformBlockIndexRed, 0);
        gl::UniformBlockBinding(shaderGreen.ID, uniformBlockIndexGreen, 0);
        gl::UniformBlockBinding(shaderBlue.ID, uniformBlockIndexBlue, 0);
        gl::UniformBlockBinding(shaderYellow.ID, uniformBlockIndexYellow, 0);

        let mut uboMatrices = 0;
        gl::GenBuffers(1, &mut uboMatrices);
        gl::BindBuffer(gl::UNIFORM_BUFFER, uboMatrices);
        gl::BufferData(gl::UNIFORM_BUFFER, 2 * mem::size_of::<Matrix4<f32>>() as isize, ptr::null(), gl::STATIC_DRAW);
        gl::BindBufferRange(gl::UNIFORM_BUFFER, 0, uboMatrices, 0, 2 * mem::size_of::<Matrix4<f32>>() as isize);

        // store projection matrix once
        let projection: Matrix4<f32> = perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
        gl::BindBuffer(gl::UNIFORM_BUFFER, uboMatrices);
        gl::BufferSubData(gl::UNIFORM_BUFFER, 0, mem::size_of::<Matrix4<f32>>() as isize, projection.as_ptr() as *const c_void);
        gl::BindBuffer(gl::UNIFORM_BUFFER, 0);

        STATE.with(|state| {
            *state.borrow_mut() = Some(State_4_8 {
                shaderRed,
                shaderGreen,
                shaderBlue,
                shaderYellow,
                cubeVAO,
                cubeVBO,
                uboMatrices,
            });
        });
}

unsafe fn render_4_8(camera: &Camera) {
    gl::ClearColor(0.1, 0.1, 0.1, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            // Update view matrix in UBO
            let view = camera.GetViewMatrix();
            gl::BindBuffer(gl::UNIFORM_BUFFER, s.uboMatrices);
            let size = mem::size_of::<Matrix4<f32>>() as isize;
            gl::BufferSubData(gl::UNIFORM_BUFFER, size, size, view.as_ptr() as *const c_void);
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);

            gl::BindVertexArray(s.cubeVAO);

            // RED - top-left
            s.shaderRed.useProgram();
            let mut model = Matrix4::from_translation(vec3(-0.75, 0.75, 0.0));
            s.shaderRed.setMat4(c_str!("model"), &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            // GREEN - top-right
            s.shaderGreen.useProgram();
            model = Matrix4::from_translation(vec3(0.75, 0.75, 0.0));
            s.shaderGreen.setMat4(c_str!("model"), &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            // YELLOW - bottom-left
            s.shaderYellow.useProgram();
            model = Matrix4::from_translation(vec3(-0.75, -0.75, 0.0));
            s.shaderYellow.setMat4(c_str!("model"), &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            // BLUE - bottom-right
            s.shaderBlue.useProgram();
            model = Matrix4::from_translation(vec3(0.75, -0.75, 0.0));
            s.shaderBlue.setMat4(c_str!("model"), &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    });
}

pub fn main_4_8() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe {
                init_4_8();
            }
        }
    });

    unsafe {
        render_4_8(&Camera::default());
    }
}
