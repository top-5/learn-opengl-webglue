#![allow(non_upper_case_globals)]
extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;
use self::gl::types::*;

use std::cell::RefCell;
use std::sync::mpsc::Receiver;
use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::path::Path;

use shader::Shader;

extern crate image;
use image::GenericImage;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

struct State_4_1 {
    shader: Shader,
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
    texture: GLuint,
}

thread_local! {
    static STATE: RefCell<Option<State_4_1>> = RefCell::new(None);
}

unsafe fn init_4_1() {
    let shader = Shader::new(
        "src/_1_getting_started/shaders/4.1.texture.vs",
        "src/_1_getting_started/shaders/4.1.texture.fs");

    let vertices: [f32; 32] = [
        // positions       // colors        // texture coords
         0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0, // top right
         0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0, // bottom right
        -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0, // bottom left
        -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0  // top left
    ];
    let indices = [
        0, 1, 3,  // first Triangle
        1, 2, 3   // second Triangle
    ];
    let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
    gl::GenVertexArrays(1, &mut vao);
    gl::GenBuffers(1, &mut vbo);
    gl::GenBuffers(1, &mut ebo);

    gl::BindVertexArray(vao);

    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(gl::ARRAY_BUFFER,
                   (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                   &vertices[0] as *const f32 as *const c_void,
                   gl::STATIC_DRAW);

    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
    gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                   (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                   &indices[0] as *const i32 as *const c_void,
                   gl::STATIC_DRAW);

    let stride = 8 * mem::size_of::<GLfloat>() as GLsizei;
    // position attribute
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    gl::EnableVertexAttribArray(0);
    // color attribute
    gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
    gl::EnableVertexAttribArray(1);
    // texture coord attribute
    gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (6 * mem::size_of::<GLfloat>()) as *const c_void);
    gl::EnableVertexAttribArray(2);

    // load and create a texture
    let mut texture = 0;
    gl::GenTextures(1, &mut texture);
    gl::BindTexture(gl::TEXTURE_2D, texture);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    
    #[cfg(not(target_arch = "wasm32"))]
    let img = image::open(&Path::new("resources/textures/container.jpg")).expect("Failed to load texture");
    #[cfg(target_arch = "wasm32")]
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

    STATE.with(|state| {
        *state.borrow_mut() = Some(State_4_1 { shader, vao, vbo, ebo, texture });
    });
}

unsafe fn render_4_1(_time: f64) {
    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);

    STATE.with(|state| {
        if let Some(s) = state.borrow().as_ref() {
            gl::BindTexture(gl::TEXTURE_2D, s.texture);
            s.shader.useProgram();
            gl::BindVertexArray(s.vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
    });
}

unsafe fn reset_4_1() {
    STATE.with(|state| {
        if let Some(s) = state.borrow_mut().take() {
            gl::DeleteVertexArrays(1, &s.vao);
            gl::DeleteBuffers(1, &s.vbo);
            gl::DeleteBuffers(1, &s.ebo);
            gl::DeleteTextures(1, &s.texture);
        }
    });
}

#[allow(non_snake_case)]
pub fn main_1_4_1() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe { init_4_1(); }
        }
    });
    let time = glfw::get_time();
    unsafe { render_4_1(time); }
}
