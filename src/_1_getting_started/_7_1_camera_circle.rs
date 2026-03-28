#![allow(non_upper_case_globals)]
extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;
use self::gl::types::*;

use std::sync::mpsc::Receiver;
use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::path::Path;
use std::ffi::CStr;

use shader::Shader;

use image;
use image::GenericImage;

use cgmath::{Matrix4, Vector3, vec3,  Deg, perspective, Point3};
use cgmath::prelude::*;

// State storage for init/render split
use std::cell::RefCell;
thread_local! {
    static STATE: RefCell<Option<State_7_1>> = RefCell::new(None);
}

struct State_7_1 {
    shader: Shader,
    vao: u32,
    vbo: u32,
    texture1: u32,
    texture2: u32,
    cube_positions: Vec<Vector3<f32>>,
}

pub fn reset_7_1() {
    STATE.with(|state| {
        if let Some(s) = state.borrow_mut().take() {
            unsafe {
                gl::DeleteVertexArrays(1, &s.vao);
                gl::DeleteBuffers(1, &s.vbo);
                gl::DeleteTextures(1, &s.texture1);
                gl::DeleteTextures(1, &s.texture2);
            }
        }
    });
}

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

unsafe fn init_7_1() {
    // configure global opengl state
    gl::Enable(gl::DEPTH_TEST);

    // build and compile our shader program
    let ourShader = Shader::new(
        "src/_1_getting_started/shaders/7.1.camera.vs",
        "src/_1_getting_started/shaders/7.1.camera.fs");

    // set up vertex data
    let vertices: [f32; 180] = [
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

    // world space positions of cubes
    let cubePositions: Vec<Vector3<f32>> = vec![
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

    let (mut VBO, mut VAO) = (0, 0);
    gl::GenVertexArrays(1, &mut VAO);
    gl::GenBuffers(1, &mut VBO);

    gl::BindVertexArray(VAO);

    gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
    gl::BufferData(gl::ARRAY_BUFFER,
                   (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                   &vertices[0] as *const f32 as *const c_void,
                   gl::STATIC_DRAW);

    let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
    gl::EnableVertexAttribArray(1);

    // load textures
    let mut texture1 = 0;
    gl::GenTextures(1, &mut texture1);
    gl::BindTexture(gl::TEXTURE_2D, texture1);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    let bytes = include_bytes!("../../resources/textures/container.jpg");
    let img = image::load_from_memory(bytes).expect("Failed to decode texture").flipv();
    let data = img.raw_pixels();
    gl::TexImage2D(gl::TEXTURE_2D,
                   0,
                   gl::RGB as i32,
                   img.width() as i32,
                   img.height() as i32,
                   0,
                   gl::RGB,
                   gl::UNSIGNED_BYTE,
                   &data[0] as *const u8 as *const c_void);
    gl::GenerateMipmap(gl::TEXTURE_2D);

    let mut texture2 = 0;
    gl::GenTextures(1, &mut texture2);
    gl::BindTexture(gl::TEXTURE_2D, texture2);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    let bytes = include_bytes!("../../resources/textures/awesomeface.png");
    let img = image::load_from_memory(bytes).expect("Failed to decode texture").flipv();
    let data = img.raw_pixels();
    gl::TexImage2D(gl::TEXTURE_2D,
                   0,
                   gl::RGB as i32,
                   img.width() as i32,
                   img.height() as i32,
                   0,
                   gl::RGBA,
                   gl::UNSIGNED_BYTE,
                   &data[0] as *const u8 as *const c_void);
    gl::GenerateMipmap(gl::TEXTURE_2D);

    // shader setup
    ourShader.useProgram();
    ourShader.setInt(c_str!("texture1"), 0);
    ourShader.setInt(c_str!("texture2"), 1);

    let projection: Matrix4<f32> = perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
    ourShader.setMat4(c_str!("projection"), &projection);

    // Store state
    STATE.with(|state| {
        *state.borrow_mut() = Some(State_7_1 {
            shader: ourShader,
            vao: VAO,
            vbo: VBO,
            texture1,
            texture2,
            cube_positions: cubePositions,
        });
    });
}

unsafe fn render_7_1(time: f64) {
    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            // bind textures
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, s.texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, s.texture2);

            // activate shader
            s.shader.useProgram();

            // camera/view transformation
            let radius: f32 = 10.0;
            let camX = time.sin() as f32 * radius;
            let camZ = time.cos() as f32 * radius;
            let view: Matrix4<f32> =
                Matrix4::look_at(Point3::new(camX, 0.0, camZ), Point3::new(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0));
            s.shader.setMat4(c_str!("view"), &view);

            // render boxes
            gl::BindVertexArray(s.vao);
            for (i, position) in s.cube_positions.iter().enumerate() {
                let mut model: Matrix4<f32> = Matrix4::from_translation(*position);
                let angle = 20.0 * i as f32;
                model = model * Matrix4::from_axis_angle(vec3(1.0, 0.3, 0.5).normalize(), Deg(angle));
                s.shader.setMat4(c_str!("model"), &model);

                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }
    });
}

#[allow(non_snake_case)]
pub fn main_1_7_1() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe { init_7_1(); }
        }
    });
    
    let time = unsafe { gl::glfw::get_time() };
    unsafe { render_7_1(time); }
}

