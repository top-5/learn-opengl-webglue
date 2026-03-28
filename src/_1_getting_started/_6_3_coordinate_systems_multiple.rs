#![allow(non_upper_case_globals)]
extern crate gl;
use self::gl::types::*;

use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::cell::RefCell;
use std::ffi::CStr;

use shader::Shader;
use image::GenericImage;
use cgmath::{Matrix4, Vector3, vec3, Deg, Rad, perspective};
use cgmath::prelude::*;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

struct State_6_3 {
    shader: Shader,
    vao: GLuint,
    vbo: GLuint,
    texture1: GLuint,
    texture2: GLuint,
    cube_positions: [Vector3<f32>; 10],
}

thread_local! {
    static STATE: RefCell<Option<State_6_3>> = RefCell::new(None);
}

unsafe fn init_6_3() -> State_6_3 {
    // Enable depth testing
    gl::Enable(gl::DEPTH_TEST);

    // Build and compile shader program
    let shader = Shader::new(
        "src/_1_getting_started/shaders/6.3.coordinate_systems.vs",
        "src/_1_getting_started/shaders/6.3.coordinate_systems.fs"
    );

    // Vertex data for cube (180 floats = 36 vertices * 5 components)
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

    // World space positions for 10 cubes
    let cube_positions: [Vector3<f32>; 10] = [
        vec3(0.0, 0.0, 0.0),
        vec3(2.0, 5.0, -15.0),
        vec3(-1.5, -2.2, -2.5),
        vec3(-3.8, -2.0, -12.3),
        vec3(2.4, -0.4, -3.5),
        vec3(-1.7, 3.0, -7.5),
        vec3(1.3, -2.0, -2.5),
        vec3(1.5, 2.0, -2.5),
        vec3(1.5, 0.2, -1.5),
        vec3(-1.3, 1.0, -1.5)
    ];

    let (mut vbo, mut vao) = (0, 0);
    gl::GenVertexArrays(1, &mut vao);
    gl::GenBuffers(1, &mut vbo);

    gl::BindVertexArray(vao);

    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
        &vertices[0] as *const f32 as *const c_void,
        gl::STATIC_DRAW
    );

    let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;
    // Position attribute
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    gl::EnableVertexAttribArray(0);
    // Texture coord attribute
    gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
    gl::EnableVertexAttribArray(1);

    // Load textures
    let mut texture1 = 0;
    gl::GenTextures(1, &mut texture1);
    gl::BindTexture(gl::TEXTURE_2D, texture1);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    #[cfg(target_arch = "wasm32")]
    let img = {
        use gl::resources::load_bytes_sync;
        let bytes = load_bytes_sync("resources/textures/container.jpg").expect("Failed to load texture");
        image::load_from_memory(&bytes).expect("Failed to decode texture")
    };
    #[cfg(not(target_arch = "wasm32"))]
    let img = image::open("resources/textures/container.jpg").expect("Failed to load texture");

    let data = img.raw_pixels();
    gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGB as i32,
        img.width() as i32,
        img.height() as i32,
        0,
        gl::RGB,
        gl::UNSIGNED_BYTE,
        &data[0] as *const u8 as *const c_void
    );
    gl::GenerateMipmap(gl::TEXTURE_2D);

    // Texture 2
    let mut texture2 = 0;
    gl::GenTextures(1, &mut texture2);
    gl::BindTexture(gl::TEXTURE_2D, texture2);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    #[cfg(target_arch = "wasm32")]
    let img = {
        use gl::resources::load_bytes_sync;
        let bytes = load_bytes_sync("resources/textures/awesomeface.png").expect("Failed to load texture");
        image::load_from_memory(&bytes).expect("Failed to decode texture")
    };
    #[cfg(not(target_arch = "wasm32"))]
    let img = image::open("resources/textures/awesomeface.png").expect("Failed to load texture");

    let img = img.flipv();
    let data = img.raw_pixels();
    gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGBA as i32,
        img.width() as i32,
        img.height() as i32,
        0,
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        &data[0] as *const u8 as *const c_void
    );
    gl::GenerateMipmap(gl::TEXTURE_2D);

    // Set texture units
    shader.useProgram();
    shader.setInt(c_str!("texture1"), 0);
    shader.setInt(c_str!("texture2"), 1);

    State_6_3 {
        shader,
        vao,
        vbo,
        texture1,
        texture2,
        cube_positions,
    }
}

unsafe fn render_6_3(time: f64) {
    STATE.with(|state| {
        if let Some(s) = state.borrow().as_ref() {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // Bind textures
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, s.texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, s.texture2);

            s.shader.useProgram();

            // View and projection matrices (same for all cubes)
            let view: Matrix4<f32> = Matrix4::from_translation(vec3(0.0, 0.0, -3.0));
            let projection: Matrix4<f32> = perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            
            let view_loc = gl::GetUniformLocation(s.shader.ID, c_str!("view").as_ptr());
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, view.as_ptr());
            s.shader.setMat4(c_str!("projection"), &projection);

            // Render 10 cubes with different positions and rotations
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

unsafe fn reset_6_3() {
    STATE.with(|state| {
        if let Some(s) = state.borrow_mut().take() {
            gl::DeleteVertexArrays(1, &s.vao);
            gl::DeleteBuffers(1, &s.vbo);
            gl::DeleteTextures(1, &s.texture1);
            gl::DeleteTextures(1, &s.texture2);
        }
    });
}

#[allow(non_snake_case)]
pub fn main_1_6_3() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            *state.borrow_mut() = Some(unsafe { init_6_3() });
        }
    });

    unsafe {
        let time = gl::glfw::get_time();
        render_6_3(time);
    }
}

