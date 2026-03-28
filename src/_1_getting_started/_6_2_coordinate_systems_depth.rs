#![allow(non_upper_case_globals)]

use gl::types::*;

use std::cell::RefCell;
use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::ffi::CStr;

use shader::Shader;
use image;
use image::GenericImage;
use cgmath::{Matrix4, vec3, Deg, Rad, perspective};
use cgmath::prelude::*;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

struct State_6_2 {
    shader: Shader,
    vao: GLuint,
    vbo: GLuint,
    texture1: GLuint,
    texture2: GLuint,
}

thread_local! {
    static STATE: RefCell<Option<State_6_2>> = RefCell::new(None);
}

unsafe fn init_6_2() {
    // Enable depth testing
    gl::Enable(gl::DEPTH_TEST);

    // build and compile shader program
    let ourShader = Shader::new(
        "src/_1_getting_started/shaders/6.2.coordinate_systems.vs",
        "src/_1_getting_started/shaders/6.2.coordinate_systems.fs");

    // Cube vertex data (position + texcoords)
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

    // Load textures
    let (mut texture1, mut texture2) = (0, 0);
    
    // texture 1 - container.jpg
    gl::GenTextures(1, &mut texture1);
    gl::BindTexture(gl::TEXTURE_2D, texture1);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    
    let img = {
        use gl::resources::load_bytes_sync;
        let bytes = load_bytes_sync("resources/textures/container.jpg").expect("Failed to load texture");
        image::load_from_memory(&bytes).expect("Failed to decode texture")
    };
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
    
    // texture 2 - awesomeface.png
    gl::GenTextures(1, &mut texture2);
    gl::BindTexture(gl::TEXTURE_2D, texture2);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    
    let img = {
        use gl::resources::load_bytes_sync;
        let bytes = load_bytes_sync("resources/textures/awesomeface.png").expect("Failed to load texture");
        image::load_from_memory(&bytes).expect("Failed to decode texture")
    };
    let img = img.flipv();
    let data = img.raw_pixels();
    gl::TexImage2D(gl::TEXTURE_2D,
                   0,
                   gl::RGBA as i32,
                   img.width() as i32,
                   img.height() as i32,
                   0,
                   gl::RGBA,
                   gl::UNSIGNED_BYTE,
                   &data[0] as *const u8 as *const c_void);
    gl::GenerateMipmap(gl::TEXTURE_2D);

    // Set texture units
    ourShader.useProgram();
    ourShader.setInt(c_str!("texture1"), 0);
    ourShader.setInt(c_str!("texture2"), 1);

    STATE.with(|state| {
        *state.borrow_mut() = Some(State_6_2 {
            shader: ourShader,
            vao: VAO,
            vbo: VBO,
            texture1,
            texture2,
        });
    });
}

unsafe fn render_6_2(time: f64) {
    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // Bind textures
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, s.texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, s.texture2);

            // Activate shader
            s.shader.useProgram();

            // Create transformations - rotating cube
            let model: Matrix4<f32> = Matrix4::from_axis_angle(
                vec3(0.5, 1.0, 0.0).normalize(),
                Rad(time as f32)
            );
            let view: Matrix4<f32> = Matrix4::from_translation(vec3(0., 0., -3.));
            let projection: Matrix4<f32> = perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            
            // Pass matrices to shader
            let modelLoc = gl::GetUniformLocation(s.shader.ID, c_str!("model").as_ptr());
            let viewLoc = gl::GetUniformLocation(s.shader.ID, c_str!("view").as_ptr());
            gl::UniformMatrix4fv(modelLoc, 1, gl::FALSE, model.as_ptr());
            gl::UniformMatrix4fv(viewLoc, 1, gl::FALSE, &view[0][0]);
            s.shader.setMat4(c_str!("projection"), &projection);

            // Render cube
            gl::BindVertexArray(s.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    });
}

unsafe fn reset_6_2() {
    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            gl::DeleteVertexArrays(1, &s.vao);
            gl::DeleteBuffers(1, &s.vbo);
            gl::DeleteTextures(1, &s.texture1);
            gl::DeleteTextures(1, &s.texture2);
        }
        *state.borrow_mut() = None;
    });
}

#[allow(non_snake_case)]
pub fn main_1_6_2() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe { init_6_2(); }
        }
    });
    
    // Use time for rotation animation
    let time = unsafe { gl::glfw::get_time() };
    unsafe { render_6_2(time); }
}
