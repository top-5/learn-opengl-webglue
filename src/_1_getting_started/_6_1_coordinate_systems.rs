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
use cgmath::{Matrix4, vec3, Deg, perspective};
use cgmath::prelude::*;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

struct State_6_1 {
    shader: Shader,
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
    texture1: GLuint,
    texture2: GLuint,
}

thread_local! {
    static STATE: RefCell<Option<State_6_1>> = RefCell::new(None);
}

unsafe fn init_6_1() {
    // build and compile our shader program
    let ourShader = Shader::new(
        "src/_1_getting_started/shaders/6.1.coordinate_systems.vs",
        "src/_1_getting_started/shaders/6.1.coordinate_systems.fs");

    // set up vertex data (and buffer(s)) and configure vertex attributes
    let vertices: [f32; 20] = [
        // positions       // texture coords
         0.5,  0.5, 0.0,   1.0, 1.0, // top right
         0.5, -0.5, 0.0,   1.0, 0.0, // bottom right
        -0.5, -0.5, 0.0,   0.0, 0.0, // bottom left
        -0.5,  0.5, 0.0,   0.0, 1.0  // top left
    ];
    let indices = [
        0, 1, 3,  // first Triangle
        1, 2, 3   // second Triangle
    ];
    let (mut VBO, mut VAO, mut EBO) = (0, 0, 0);
    gl::GenVertexArrays(1, &mut VAO);
    gl::GenBuffers(1, &mut VBO);
    gl::GenBuffers(1, &mut EBO);

    gl::BindVertexArray(VAO);

    gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
    gl::BufferData(gl::ARRAY_BUFFER,
                   (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                   &vertices[0] as *const f32 as *const c_void,
                   gl::STATIC_DRAW);

    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
    gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                   (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                   &indices[0] as *const i32 as *const c_void,
                   gl::STATIC_DRAW);

    let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;
    // position attribute
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    gl::EnableVertexAttribArray(0);
    // texture coord attribute
    gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
    gl::EnableVertexAttribArray(1);

    // load and create textures
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
                   gl::RGBA as i32,  // PNG has alpha
                   img.width() as i32,
                   img.height() as i32,
                   0,
                   gl::RGBA,
                   gl::UNSIGNED_BYTE,
                   &data[0] as *const u8 as *const c_void);
    gl::GenerateMipmap(gl::TEXTURE_2D);

    // tell opengl for each sampler to which texture unit it belongs to
    ourShader.useProgram();
    ourShader.setInt(c_str!("texture1"), 0);
    ourShader.setInt(c_str!("texture2"), 1);

    STATE.with(|state| {
        *state.borrow_mut() = Some(State_6_1 {
            shader: ourShader,
            vao: VAO,
            vbo: VBO,
            ebo: EBO,
            texture1,
            texture2,
        });
    });
}

unsafe fn render_6_1(_time: f64) {
    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // bind textures
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, s.texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, s.texture2);

            // activate shader
            s.shader.useProgram();

            // create transformations
            let model: Matrix4<f32> = Matrix4::from_angle_x(Deg(-55.));
            let view: Matrix4<f32> = Matrix4::from_translation(vec3(0., 0., -3.));
            let projection: Matrix4<f32> = perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            
            // pass matrices to shader
            let modelLoc = gl::GetUniformLocation(s.shader.ID, c_str!("model").as_ptr());
            let viewLoc = gl::GetUniformLocation(s.shader.ID, c_str!("view").as_ptr());
            gl::UniformMatrix4fv(modelLoc, 1, gl::FALSE, model.as_ptr());
            gl::UniformMatrix4fv(viewLoc, 1, gl::FALSE, &view[0][0]);
            s.shader.setMat4(c_str!("projection"), &projection);

            // render container
            gl::BindVertexArray(s.vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
    });
}

unsafe fn reset_6_1() {
    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            gl::DeleteVertexArrays(1, &s.vao);
            gl::DeleteBuffers(1, &s.vbo);
            gl::DeleteBuffers(1, &s.ebo);
            gl::DeleteTextures(1, &s.texture1);
            gl::DeleteTextures(1, &s.texture2);
        }
        *state.borrow_mut() = None;
    });
}

#[allow(non_snake_case)]
pub fn main_1_6_1() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe { init_6_1(); }
        }
    });
    
    let time = 0.0; // Static scene
    unsafe { render_6_1(time); }
}
