#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use std::cell::RefCell;

extern crate glfw;

extern crate gl;
use self::gl::types::*;

use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::path::Path;
use std::ffi::CStr;

use common::loadTexture;
use shader::Shader;
use camera::Camera;

use cgmath::{Matrix4, vec3, Vector3, Deg, perspective};
use cgmath::prelude::*;

// settings
const SCR_WIDTH: u32 = 1280;
const SCR_HEIGHT: u32 = 720;

struct State_4_3_1 {
    shader: Shader,
    cubeVAO: GLuint,
    cubeVBO: GLuint,
    planeVAO: GLuint,
    planeVBO: GLuint,
    transparentVAO: GLuint,
    transparentVBO: GLuint,
    cubeTexture: GLuint,
    floorTexture: GLuint,
    transparentTexture: GLuint,
    vegetation: [Vector3<f32>; 5],
}

thread_local! {
    static STATE: RefCell<Option<State_4_3_1>> = RefCell::new(None);
}

unsafe fn reset_4_3_1() {
    STATE.with(|state| {
        if let Some(s) = state.borrow_mut().take() {
            gl::DeleteVertexArrays(1, &s.cubeVAO);
            gl::DeleteVertexArrays(1, &s.planeVAO);
            gl::DeleteVertexArrays(1, &s.transparentVAO);
            gl::DeleteBuffers(1, &s.cubeVBO);
            gl::DeleteBuffers(1, &s.planeVBO);
            gl::DeleteBuffers(1, &s.transparentVBO);
            gl::DeleteTextures(1, &s.cubeTexture);
            gl::DeleteTextures(1, &s.floorTexture);
            gl::DeleteTextures(1, &s.transparentTexture);
        }
    });
}

unsafe fn init_4_3_1() {
    // configure global opengl state
    gl::Enable(gl::DEPTH_TEST);

    // build and compile our shader program
    let shader = Shader::new(
        "src/_4_advanced_opengl/shaders/3.1.blending.vs",
        "src/_4_advanced_opengl/shaders/3.1.blending.fs");

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
            // positions       // texture Coords (note we set these higher than 1 (together with GL_REPEAT as texture wrapping mode). this will cause the floor texture to repeat)
             5.0, -0.5,  5.0,  2.0, 0.0,
            -5.0, -0.5,  5.0,  0.0, 0.0,
            -5.0, -0.5, -5.0,  0.0, 2.0,

             5.0, -0.5,  5.0,  2.0, 0.0,
            -5.0, -0.5, -5.0,  0.0, 2.0,
             5.0, -0.5, -5.0,  2.0, 2.0
        ];
        let transparentVertices: [f32; 30] = [
            // positions      // texture Coords (swapped y coordinates because texture is flipped upside down)
            0.0,  0.5,  0.0,  0.0,  0.0,
            0.0, -0.5,  0.0,  0.0,  1.0,
            1.0, -0.5,  0.0,  1.0,  1.0,

            0.0,  0.5,  0.0,  0.0,  0.0,
            1.0, -0.5,  0.0,  1.0,  1.0,
            1.0,  0.5,  0.0,  1.0,  0.0
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
        // transparent VAO
        let (mut transparentVAO, mut transparentVBO) = (0, 0);
        gl::GenVertexArrays(1, &mut transparentVAO);
        gl::GenBuffers(1, &mut transparentVBO);
        gl::BindVertexArray(transparentVAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, transparentVBO);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (transparentVertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &transparentVertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::BindVertexArray(0);

        // load textures
        let cubeTexture = loadTexture("resources/textures/marble.jpg");
        let floorTexture = loadTexture("resources/textures/metal.png");
        let transparentTexture = loadTexture("resources/textures/grass.png");

        // transparent vegetation locations
        let vegetation = [
            vec3(-1.5, 0.0, -0.48),
            vec3( 1.5, 0.0, 0.51),
            vec3( 0.0, 0.0, 0.7),
            vec3(-0.3, 0.0, -2.3),
            vec3 (0.5, 0.0, -0.6)
        ];

        // shader configuration
        shader.useProgram();
        shader.setInt(c_str!("texture1"), 0);

        STATE.with(|state| {
            *state.borrow_mut() = Some(State_4_3_1 {
                shader,
                cubeVAO,
                cubeVBO,
                planeVAO,
                planeVBO,
                transparentVAO,
                transparentVBO,
                cubeTexture,
                floorTexture,
                transparentTexture,
                vegetation,
            });
        });
}

unsafe fn render_4_3_1(camera: &Camera) {
    gl::ClearColor(0.1, 0.1, 0.1, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            s.shader.useProgram();
            let view = camera.GetViewMatrix();
            let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            s.shader.setMat4(c_str!("view"), &view);
            s.shader.setMat4(c_str!("projection"), &projection);

            // cubes
            gl::BindVertexArray(s.cubeVAO);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, s.cubeTexture);
            let mut model = Matrix4::from_translation(vec3(-1.0, 0.0, -1.0));
            s.shader.setMat4(c_str!("model"), &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            model = Matrix4::from_translation(vec3(2.0, 0.0, 0.0));
            s.shader.setMat4(c_str!("model"), &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            // floor
            gl::BindVertexArray(s.planeVAO);
            gl::BindTexture(gl::TEXTURE_2D, s.floorTexture);
            s.shader.setMat4(c_str!("model"), &Matrix4::identity());
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
            // vegetation
            gl::BindVertexArray(s.transparentVAO);
            gl::BindTexture(gl::TEXTURE_2D, s.transparentTexture);
            for v in &s.vegetation {
                let model = Matrix4::from_translation(*v);
                s.shader.setMat4(c_str!("model"), &model);
                gl::DrawArrays(gl::TRIANGLES, 0, 6);
            }
        }
    });
}

pub fn main_4_3_1() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe { init_4_3_1(); }
        }
    });
    unsafe { render_4_3_1(&Camera::default()); }
}
