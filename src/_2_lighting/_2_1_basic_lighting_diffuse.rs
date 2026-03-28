#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
extern crate glfw;
use self::glfw::Context;

extern crate gl;
use self::gl::types::*;

use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::ffi::CStr;
use std::cell::RefCell;

use common::process_events;
use shader::Shader;
use camera::Camera;

use cgmath::{Matrix4, vec3, Point3, Deg, perspective, Vector3};
use cgmath::prelude::*;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

struct State_2_2_1 {
    lighting_shader: Shader,
    lamp_shader: Shader,
    cube_vao: GLuint,
    light_vao: GLuint,
    vbo: GLuint,
    light_pos: Vector3<f32>,
}

thread_local! {
    static STATE: RefCell<Option<State_2_2_1>> = RefCell::new(None);
}

pub unsafe fn reset_2_2_1() {
    STATE.with(|state| {
        if let Some(s) = state.borrow_mut().take() {
            gl::DeleteVertexArrays(1, &s.cube_vao);
            gl::DeleteVertexArrays(1, &s.light_vao);
            gl::DeleteBuffers(1, &s.vbo);
        }
    });
}

unsafe fn init_2_2_1() {
    // configure global opengl state
    // -----------------------------
    gl::Enable(gl::DEPTH_TEST);

    // configure global opengl state
    // -----------------------------
    gl::Enable(gl::DEPTH_TEST);

    // build and compile our shader program
    // ------------------------------------
    let lightingShader = Shader::new(
        "src/_2_lighting/shaders/2.1.basic_lighting.vs",
        "src/_2_lighting/shaders/2.1.basic_lighting.fs");
    let lampShader = Shader::new(
        "src/_2_lighting/shaders/2.1.lamp.vs",
        "src/_2_lighting/shaders/2.1.lamp.fs");

    // set up vertex data (and buffer(s)) and configure vertex attributes
    // ------------------------------------------------------------------
    let vertices: [f32; 216] = [
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,
         0.5, -0.5, -0.5,  0.0,  0.0, -1.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
        -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,

        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,
         0.5, -0.5,  0.5,  0.0,  0.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
        -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,

        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,
        -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
        -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,
        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,

         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,
         0.5,  0.5, -0.5,  1.0,  0.0,  0.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
         0.5, -0.5,  0.5,  1.0,  0.0,  0.0,
         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,

        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
         0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
        -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,

        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
         0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
        -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0
    ];
    // first, configure the cube's VAO (and VBO)
    let (mut VBO, mut cubeVAO) = (0, 0);
    gl::GenVertexArrays(1, &mut cubeVAO);
    gl::GenBuffers(1, &mut VBO);

    gl::BindVertexArray(cubeVAO);

    gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
    gl::BufferData(gl::ARRAY_BUFFER,
                   (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                   &vertices[0] as *const f32 as *const c_void,
                   gl::STATIC_DRAW);

    let stride = 6 * mem::size_of::<GLfloat>() as GLsizei;
    // position attribute
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    gl::EnableVertexAttribArray(0);
    // normal attribute
    gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
    gl::EnableVertexAttribArray(1);


    // second, configure the light's VAO (VBO stays the same; the vertices are the same for the light object which is also a 3D cube)
    let mut lightVAO = 0;
    gl::GenVertexArrays(1, &mut lightVAO);
    gl::BindVertexArray(lightVAO);

    gl::BindBuffer(gl::ARRAY_BUFFER, VBO);

    // note that we update the lamp's position attribute's stride to reflect the updated buffer data
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    gl::EnableVertexAttribArray(0);

    STATE.with(|state| {
        *state.borrow_mut() = Some(State_2_2_1 {
            lighting_shader: lightingShader,
            lamp_shader: lampShader,
            cube_vao: cubeVAO,
            light_vao: lightVAO,
            vbo: VBO,
            light_pos: vec3(1.2, 1.0, 2.0),
        });
    });
}

unsafe fn render_2_2_1() {
    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            // be sure to activate shader when setting uniforms/drawing objects
            s.lighting_shader.useProgram();
            s.lighting_shader.setVec3(c_str!("objectColor"), 1.0, 0.5, 0.31);
            s.lighting_shader.setVec3(c_str!("lightColor"), 1.0, 1.0, 1.0);
            s.lighting_shader.setVector3(c_str!("lightPos"), &s.light_pos);

            // view/projection transformations
            let projection: Matrix4<f32> = perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            let view = Matrix4::from_translation(vec3(0.0, 0.0, -3.0));
            s.lighting_shader.setMat4(c_str!("projection"), &projection);
            s.lighting_shader.setMat4(c_str!("view"), &view);

            // world transformation
            let model = Matrix4::<f32>::identity();
            s.lighting_shader.setMat4(c_str!("model"), &model);

            // render the cube
            gl::BindVertexArray(s.cube_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            // also draw the lamp object
            s.lamp_shader.useProgram();
            s.lamp_shader.setMat4(c_str!("projection"), &projection);
            s.lamp_shader.setMat4(c_str!("view"), &view);
            let mut model = Matrix4::from_translation(s.light_pos);
            model = model * Matrix4::from_scale(0.2);  // a smaller cube
            s.lamp_shader.setMat4(c_str!("model"), &model);

            gl::BindVertexArray(s.light_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    });
}

#[allow(non_snake_case)]
pub fn main_2_2_1() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe { init_2_2_1(); }
        }
    });
    unsafe { render_2_2_1(); }
}
