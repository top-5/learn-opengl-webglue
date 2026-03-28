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

struct State_2_6 {
    shader: Shader,
    lamp_shader: Shader,
    vbo: u32,
    cube_vao: u32,
    light_vao: u32,
    diffuse_map: u32,
    specular_map: u32,
    cube_positions: Vec<Vector3<f32>>,
    point_light_positions: Vec<Vector3<f32>>,
}

thread_local! {
    static STATE: RefCell<Option<State_2_6>> = RefCell::new(None);
}

pub unsafe fn reset_2_6() {
    STATE.with(|state| {
        if let Some(s) = state.borrow_mut().take() {
            gl::DeleteVertexArrays(1, &s.cube_vao);
            gl::DeleteVertexArrays(1, &s.light_vao);
            gl::DeleteBuffers(1, &s.vbo);
        }
    });
}

unsafe fn init_2_6() {
    gl::Enable(gl::DEPTH_TEST);

    let shader = Shader::new(
        "src/_2_lighting/shaders/6.multiple_lights.vs",
        "src/_2_lighting/shaders/6.multiple_lights.fs");
    let lamp_shader = Shader::new(
        "src/_2_lighting/shaders/6.lamp.vs",
        "src/_2_lighting/shaders/6.lamp.fs");

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

    let cube_positions = vec![
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

    let point_light_positions = vec![
        vec3( 0.7,  0.2,  2.0),
        vec3( 2.3, -3.3, -4.0),
        vec3(-4.0,  2.0, -12.0),
        vec3( 0.0,  0.0, -3.0)
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
    let specular_map = loadTexture("resources/textures/container2_specular.png");

    shader.useProgram();
    shader.setInt(c_str!("material.diffuse"), 0);
    shader.setInt(c_str!("material.specular"), 1);

    STATE.with(|state| {
        *state.borrow_mut() = Some(State_2_6 {
            shader,
            lamp_shader,
            vbo,
            cube_vao,
            light_vao,
            diffuse_map,
            specular_map,
            cube_positions,
            point_light_positions,
        });
    });
}

unsafe fn render_2_6(camera: &Camera) {
    STATE.with(|state| {
        if let Some(s) = state.borrow().as_ref() {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            s.shader.useProgram();
            s.shader.setVector3(c_str!("viewPos"), &camera.Position.to_vec());
            s.shader.setFloat(c_str!("material.shininess"), 32.0);

            // Directional light
            s.shader.setVec3(c_str!("dirLight.direction"), -0.2, -1.0, -0.3);
            s.shader.setVec3(c_str!("dirLight.ambient"), 0.05, 0.05, 0.05);
            s.shader.setVec3(c_str!("dirLight.diffuse"), 0.4, 0.4, 0.4);
            s.shader.setVec3(c_str!("dirLight.specular"), 0.5, 0.5, 0.5);

            // Point lights
            for i in 0..4 {
                use std::ffi::CString;
                let pos_name = CString::new(format!("pointLights[{}].position", i)).unwrap();
                let amb_name = CString::new(format!("pointLights[{}].ambient", i)).unwrap();
                let diff_name = CString::new(format!("pointLights[{}].diffuse", i)).unwrap();
                let spec_name = CString::new(format!("pointLights[{}].specular", i)).unwrap();
                let const_name = CString::new(format!("pointLights[{}].constant", i)).unwrap();
                let lin_name = CString::new(format!("pointLights[{}].linear", i)).unwrap();
                let quad_name = CString::new(format!("pointLights[{}].quadratic", i)).unwrap();
                
                s.shader.setVector3(&pos_name, &s.point_light_positions[i]);
                s.shader.setVec3(&amb_name, 0.05, 0.05, 0.05);
                s.shader.setVec3(&diff_name, 0.8, 0.8, 0.8);
                s.shader.setVec3(&spec_name, 1.0, 1.0, 1.0);
                s.shader.setFloat(&const_name, 1.0);
                s.shader.setFloat(&lin_name, 0.09);
                s.shader.setFloat(&quad_name, 0.032);
            }

            // Spotlight
            s.shader.setVec3(c_str!("spotLight.position"), camera.Position.x, camera.Position.y, camera.Position.z);
            s.shader.setVector3(c_str!("spotLight.direction"), &camera.Front);
            s.shader.setVec3(c_str!("spotLight.ambient"), 0.0, 0.0, 0.0);
            s.shader.setVec3(c_str!("spotLight.diffuse"), 1.0, 1.0, 1.0);
            s.shader.setVec3(c_str!("spotLight.specular"), 1.0, 1.0, 1.0);
            s.shader.setFloat(c_str!("spotLight.constant"), 1.0);
            s.shader.setFloat(c_str!("spotLight.linear"), 0.09);
            s.shader.setFloat(c_str!("spotLight.quadratic"), 0.032);
            s.shader.setFloat(c_str!("spotLight.cutOff"), 12.5f32.to_radians().cos());
            s.shader.setFloat(c_str!("spotLight.outerCutOff"), 15.0f32.to_radians().cos());

            let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            let view = camera.GetViewMatrix();
            s.shader.setMat4(c_str!("projection"), &projection);
            s.shader.setMat4(c_str!("view"), &view);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, s.diffuse_map);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, s.specular_map);

            gl::BindVertexArray(s.cube_vao);
            for (i, position) in s.cube_positions.iter().enumerate() {
                let mut model: Matrix4<f32> = Matrix4::from_translation(*position);
                let angle = 20.0 * i as f32;
                model = model * Matrix4::from_axis_angle(vec3(1.0, 0.3, 0.5).normalize(), Deg(angle));
                s.shader.setMat4(c_str!("model"), &model);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }

            // Draw lamp cubes
            s.lamp_shader.useProgram();
            s.lamp_shader.setMat4(c_str!("projection"), &projection);
            s.lamp_shader.setMat4(c_str!("view"), &view);

            gl::BindVertexArray(s.light_vao);
            for position in &s.point_light_positions {
                let mut model = Matrix4::from_translation(*position);
                model = model * Matrix4::from_scale(0.2);
                s.lamp_shader.setMat4(c_str!("model"), &model);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }
    });
}

pub fn main_2_6() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe { init_2_6(); }
        }
    });
    unsafe { render_2_6(&Camera::default()); }
}

