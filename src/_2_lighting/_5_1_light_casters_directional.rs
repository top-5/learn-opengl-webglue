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

use cgmath::{Matrix4, Vector3, vec3, Point3, Deg, perspective};
use cgmath::prelude::*;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

struct State_2_5_1 {
    lighting_shader: Shader,
    cube_vao: GLuint,
    vbo: GLuint,
    diffuse_map: GLuint,
    specular_map: GLuint,
    cube_positions: Vec<Vector3<f32>>,
}

thread_local! {
    static STATE: RefCell<Option<State_2_5_1>> = RefCell::new(None);
}

pub unsafe fn reset_2_5_1() {
    STATE.with(|state| {
        if let Some(s) = state.borrow_mut().take() {
            gl::DeleteVertexArrays(1, &s.cube_vao);
            gl::DeleteBuffers(1, &s.vbo);
            gl::DeleteTextures(1, &s.diffuse_map);
            gl::DeleteTextures(1, &s.specular_map);
        }
    });
}

unsafe fn init_2_5_1() {
    gl::Enable(gl::DEPTH_TEST);

    let lighting_shader = Shader::new(
        "src/_2_lighting/shaders/5.1.light_casters.vs",
        "src/_2_lighting/shaders/5.1.light_casters.fs");

    let vertices: [f32; 288] = [
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

    let cube_positions: Vec<Vector3<f32>> = vec![
        vec3( 0.0,  0.0,  0.0),
        vec3( 2.0,  5.0, -15.0),
        vec3(-1.5, -2.2, -2.5),
        vec3(-3.8, -2.0, -12.3),
        vec3( 2.4, -0.4, -3.5),
        vec3(-1.7,  3.0, -7.5),
        vec3( 1.3, -2.0, -2.5),
        vec3( 1.5,  2.0, -2.5),
        vec3( 1.5,  0.2, -1.5),
        vec3(-1.3,  1.0, -1.5)
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

    let diffuse_map = loadTexture("resources/textures/container2.png");
    let specular_map = loadTexture("resources/textures/container2_specular.png");

    lighting_shader.useProgram();
    lighting_shader.setInt(c_str!("material.diffuse"), 0);
    lighting_shader.setInt(c_str!("material.specular"), 1);

    STATE.with(|state| {
        *state.borrow_mut() = Some(State_2_5_1 {
            lighting_shader,
            cube_vao,
            vbo,
            diffuse_map,
            specular_map,
            cube_positions,
        });
    });
}

unsafe fn render_2_5_1() {
    gl::ClearColor(0.1, 0.1, 0.1, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            s.lighting_shader.useProgram();
            s.lighting_shader.setVector3(c_str!("light.direction"), &vec3(-0.2, -1.0, -0.3));
            s.lighting_shader.setVec3(c_str!("viewPos"), 0.0, 0.0, 3.0);

            s.lighting_shader.setVec3(c_str!("light.ambient"), 0.2, 0.2, 0.2);
            s.lighting_shader.setVec3(c_str!("light.diffuse"), 0.5, 0.5, 0.5);
            s.lighting_shader.setVec3(c_str!("light.specular"), 1.0, 1.0, 1.0);
            s.lighting_shader.setFloat(c_str!("material.shininess"), 32.0);

            let projection: Matrix4<f32> = perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            let view = Matrix4::from_translation(vec3(0.0, 0.0, -3.0));
            s.lighting_shader.setMat4(c_str!("projection"), &projection);
            s.lighting_shader.setMat4(c_str!("view"), &view);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, s.diffuse_map);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, s.specular_map);

            gl::BindVertexArray(s.cube_vao);
            for (i, position) in s.cube_positions.iter().enumerate() {
                let mut model = Matrix4::from_translation(*position);
                let angle = 20.0 * i as f32;
                model = model * Matrix4::from_axis_angle(vec3(1.0, 0.3, 0.5).normalize(), Deg(angle));
                s.lighting_shader.setMat4(c_str!("model"), &model);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }
    });
}

pub fn main_2_5_1() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe { init_2_5_1(); }
        }
    });
    unsafe { render_2_5_1(); }
}
