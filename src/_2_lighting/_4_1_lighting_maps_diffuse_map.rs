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

use common::{process_events, loadTexture};
use shader::Shader;
use camera::Camera;

use cgmath::{Matrix4, vec3, Point3, Deg, perspective, Vector3};
use cgmath::prelude::*;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

struct State_2_4_1 {
    lighting_shader: Shader,
    lamp_shader: Shader,
    cube_vao: GLuint,
    light_vao: GLuint,
    vbo: GLuint,
    diffuse_map: GLuint,
    light_pos: Vector3<f32>,
}

thread_local! {
    static STATE: RefCell<Option<State_2_4_1>> = RefCell::new(None);
}

pub unsafe fn reset_2_4_1() {
    STATE.with(|state| {
        if let Some(s) = state.borrow_mut().take() {
            gl::DeleteVertexArrays(1, &s.cube_vao);
            gl::DeleteVertexArrays(1, &s.light_vao);
            gl::DeleteBuffers(1, &s.vbo);
            gl::DeleteTextures(1, &s.diffuse_map);
        }
    });
}

unsafe fn init_2_4_1() {
    gl::Enable(gl::DEPTH_TEST);

    let lighting_shader = Shader::new(
        "src/_2_lighting/shaders/4.1.lighting_maps.vs",
        "src/_2_lighting/shaders/4.1.lighting_maps.fs");
    let lamp_shader = Shader::new(
        "src/_2_lighting/shaders/4.1.lamp.vs",
        "src/_2_lighting/shaders/4.1.lamp.fs");

    let vertices: [f32; 288] = [
        // positions       // normals        // texture coords
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,
         0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  0.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
        -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  1.0,
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,

        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,
         0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
        -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  1.0,
        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,

        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,
        -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,  1.0,  1.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
        -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,  0.0,  0.0,
        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,

         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,
         0.5,  0.5, -0.5,  1.0,  0.0,  0.0,  1.0,  1.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
         0.5, -0.5,  0.5,  1.0,  0.0,  0.0,  0.0,  0.0,
         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,

        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,
         0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  1.0,  1.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
        -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  0.0,  0.0,
        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,

        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0,
         0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  1.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
        -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  0.0,  0.0,
        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0
    ];

    let (mut vbo, mut cube_vao) = (0, 0);
    gl::GenVertexArrays(1, &mut cube_vao);
    gl::GenBuffers(1, &mut vbo);

    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(gl::ARRAY_BUFFER,
                   (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                   &vertices[0] as *const f32 as *const c_void,
                   gl::STATIC_DRAW);

    gl::BindVertexArray(cube_vao);
    let stride = 8 * mem::size_of::<GLfloat>() as GLsizei;
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
    gl::EnableVertexAttribArray(1);
    gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (6 * mem::size_of::<GLfloat>()) as *const c_void);
    gl::EnableVertexAttribArray(2);

    let mut light_vao = 0;
    gl::GenVertexArrays(1, &mut light_vao);
    gl::BindVertexArray(light_vao);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    gl::EnableVertexAttribArray(0);

    let diffuse_map = loadTexture("resources/textures/container2.png");

    lighting_shader.useProgram();
    lighting_shader.setInt(c_str!("material.diffuse"), 0);

    STATE.with(|state| {
        *state.borrow_mut() = Some(State_2_4_1 {
            lighting_shader,
            lamp_shader,
            cube_vao,
            light_vao,
            vbo,
            diffuse_map,
            light_pos: vec3(1.2, 1.0, 2.0),
        });
    });
}

unsafe fn render_2_4_1() {
    gl::ClearColor(0.1, 0.1, 0.1, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            s.lighting_shader.useProgram();
            s.lighting_shader.setVector3(c_str!("light.position"), &s.light_pos);
            s.lighting_shader.setVec3(c_str!("viewPos"), 0.0, 0.0, 3.0);

            s.lighting_shader.setVec3(c_str!("light.ambient"), 0.2, 0.2, 0.2);
            s.lighting_shader.setVec3(c_str!("light.diffuse"), 0.5, 0.5, 0.5);
            s.lighting_shader.setVec3(c_str!("light.specular"), 1.0, 1.0, 1.0);

            s.lighting_shader.setVec3(c_str!("material.specular"), 0.5, 0.5, 0.5);
            s.lighting_shader.setFloat(c_str!("material.shininess"), 64.0);

            let projection: Matrix4<f32> = perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            let view = Matrix4::from_translation(vec3(0.0, 0.0, -3.0));
            s.lighting_shader.setMat4(c_str!("projection"), &projection);
            s.lighting_shader.setMat4(c_str!("view"), &view);

            let model = Matrix4::<f32>::identity();
            s.lighting_shader.setMat4(c_str!("model"), &model);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, s.diffuse_map);

            gl::BindVertexArray(s.cube_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            s.lamp_shader.useProgram();
            s.lamp_shader.setMat4(c_str!("projection"), &projection);
            s.lamp_shader.setMat4(c_str!("view"), &view);
            let mut lamp_model = Matrix4::from_translation(s.light_pos);
            lamp_model = lamp_model * Matrix4::from_scale(0.2);
            s.lamp_shader.setMat4(c_str!("model"), &lamp_model);

            gl::BindVertexArray(s.light_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    });
}

pub fn main_2_4_1() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe { init_2_4_1(); }
        }
    });
    unsafe { render_2_4_1(); }
}
