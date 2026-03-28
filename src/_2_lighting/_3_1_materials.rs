#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
extern crate glfw;

extern crate gl;
use self::gl::types::*;

use std::cell::RefCell;
use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::ffi::CStr;

use shader::Shader;

use cgmath::{Matrix4, Vector3, vec3, Point3, Deg, perspective};
use cgmath::prelude::*;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

struct State_2_3_1 {
    lighting_shader: Shader,
    lamp_shader: Shader,
    cube_vao: GLuint,
    light_vao: GLuint,
    vbo: GLuint,
    light_pos: Vector3<f32>,
}

thread_local! {
    static STATE: RefCell<Option<State_2_3_1>> = RefCell::new(None);
}

pub unsafe fn reset_2_3_1() {
    STATE.with(|state| {
        if let Some(s) = state.borrow_mut().take() {
            gl::DeleteVertexArrays(1, &s.cube_vao);
            gl::DeleteVertexArrays(1, &s.light_vao);
            gl::DeleteBuffers(1, &s.vbo);
        }
    });
}

unsafe fn init_2_3_1() {
    gl::Enable(gl::DEPTH_TEST);

    let lighting_shader = Shader::new(
        "src/_2_lighting/shaders/3.1.materials.vs",
        "src/_2_lighting/shaders/3.1.materials.fs");
    let lamp_shader = Shader::new(
        "src/_2_lighting/shaders/3.1.lamp.vs",
        "src/_2_lighting/shaders/3.1.lamp.fs");

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
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
    gl::EnableVertexAttribArray(1);

    let mut lightVAO = 0;
    gl::GenVertexArrays(1, &mut lightVAO);
    gl::BindVertexArray(lightVAO);
    gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    gl::EnableVertexAttribArray(0);

    let light_pos = vec3(1.2, 1.0, 2.0);

    STATE.with(|state| {
        *state.borrow_mut() = Some(State_2_3_1 {
            lighting_shader,
            lamp_shader,
            cube_vao: cubeVAO,
            light_vao: lightVAO,
            vbo: VBO,
            light_pos,
        });
    });
}

unsafe fn render_2_3_1(time: f32) {
    gl::ClearColor(0.1, 0.1, 0.1, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            s.lighting_shader.useProgram();
            s.lighting_shader.setVector3(c_str!("light.position"), &s.light_pos);
            
            let view_pos = Point3::new(0.0, 0.0, 3.0);
            s.lighting_shader.setVector3(c_str!("viewPos"), &view_pos.to_vec());

            // Animated light color
            let lightColor = Vector3 {
                x: (time * 2.0).sin(),
                y: (time * 0.7).sin(),
                z: (time * 1.3).sin(),
            };
            let diffuseColor = lightColor * 0.5;
            let ambientColor = diffuseColor * 0.2;
            s.lighting_shader.setVector3(c_str!("light.ambient"), &ambientColor);
            s.lighting_shader.setVector3(c_str!("light.diffuse"), &diffuseColor);
            s.lighting_shader.setVec3(c_str!("light.specular"), 1.0, 1.0, 1.0);

            // Material properties
            s.lighting_shader.setVec3(c_str!("material.ambient"), 1.0, 0.5, 0.31);
            s.lighting_shader.setVec3(c_str!("material.diffuse"), 1.0, 0.5, 0.31);
            s.lighting_shader.setVec3(c_str!("material.specular"), 0.5, 0.5, 0.5);
            s.lighting_shader.setFloat(c_str!("material.shininess"), 32.0);

            let projection: Matrix4<f32> = perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            let view = Matrix4::from_translation(vec3(0.0, 0.0, -3.0));
            s.lighting_shader.setMat4(c_str!("projection"), &projection);
            s.lighting_shader.setMat4(c_str!("view"), &view);

            let model = Matrix4::<f32>::identity();
            s.lighting_shader.setMat4(c_str!("model"), &model);

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

pub fn main_2_3_1() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe { init_2_3_1(); }
        }
    });
    
    // Simple static time for demo (could be enhanced with actual time tracking)
    let time = 0.0;
    
    unsafe { render_2_3_1(time); }
}
