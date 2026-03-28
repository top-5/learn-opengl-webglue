#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

extern crate gl;
use self::gl::types::*;

use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::cell::RefCell;
use std::ffi::CStr;

use shader::Shader;
use camera::Camera;

use cgmath::{Matrix4, Vector3, vec3, Deg, perspective, Point3};
use cgmath::prelude::*;
use image::GenericImage;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

struct State_7_4 {
    shader: Shader,
    vao: GLuint,
    vbo: GLuint,
    texture1: GLuint,
    texture2: GLuint,
    cube_positions: Vec<Vector3<f32>>,
    camera: Camera,
    first_mouse: bool,
    last_x: f32,
    last_y: f32,
    last_frame: f32,
}

thread_local! {
    static STATE: RefCell<Option<State_7_4>> = RefCell::new(None);
}

unsafe fn init_7_4() {
    // configure global opengl state
    gl::Enable(gl::DEPTH_TEST);

    // build and compile our shader program
    let shader = Shader::new(
        "src/_1_getting_started/shaders/7.4.camera.vs",
        "src/_1_getting_started/shaders/7.4.camera.fs");

    // set up vertex data (and buffer(s)) and configure vertex attributes
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
    
    // world space positions of our cubes
    let cube_positions: Vec<Vector3<f32>> = vec![
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
    gl::BufferData(gl::ARRAY_BUFFER,
                   (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                   &vertices[0] as *const f32 as *const c_void,
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
    
    // texture 1
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
    let img = img.to_rgba();
    let width = img.width() as i32;
    let height = img.height() as i32;
    let data = img.into_raw();
    gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, width, height,
                   0, gl::RGBA, gl::UNSIGNED_BYTE, &data[0] as *const u8 as *const c_void);
    gl::GenerateMipmap(gl::TEXTURE_2D);
    
    // texture 2
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
    let img = img.flipv().to_rgba();
    let width = img.width() as i32;
    let height = img.height() as i32;
    let data = img.into_raw();
    gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, width, height,
                   0, gl::RGBA, gl::UNSIGNED_BYTE, &data[0] as *const u8 as *const c_void);
    gl::GenerateMipmap(gl::TEXTURE_2D);

    // tell opengl for each sampler to which texture unit it belongs to
    shader.useProgram();
    shader.setInt(c_str!("texture1"), 0);
    shader.setInt(c_str!("texture2"), 1);

    // Initialize camera
    let camera = Camera {
        Position: Point3::new(0.0, 0.0, 3.0),
        ..Camera::default()
    };

    // Store state
    STATE.with(|state| {
        *state.borrow_mut() = Some(State_7_4 {
            shader,
            vao,
            vbo,
            texture1,
            texture2,
            cube_positions,
            camera,
            first_mouse: true,
            last_x: SCR_WIDTH as f32 / 2.0,
            last_y: SCR_HEIGHT as f32 / 2.0,
            last_frame: 0.0,
        });
    });
}

unsafe fn render_7_4(time: f64) {
    STATE.with(|state| {
        if let Some(s) = state.borrow_mut().as_mut() {
            // Calculate delta time
            let current_frame = time as f32;
            let delta_time = current_frame - s.last_frame;
            s.last_frame = current_frame;

            // Process input
            use gl::glfw::{self, Key, Action};
            use camera::Camera_Movement::*;
            
            if glfw::key_state(Key::W) == Action::Press {
                s.camera.ProcessKeyboard(FORWARD, delta_time);
            }
            if glfw::key_state(Key::S) == Action::Press {
                s.camera.ProcessKeyboard(BACKWARD, delta_time);
            }
            if glfw::key_state(Key::A) == Action::Press {
                s.camera.ProcessKeyboard(LEFT, delta_time);
            }
            if glfw::key_state(Key::D) == Action::Press {
                s.camera.ProcessKeyboard(RIGHT, delta_time);
            }

            // Process mouse movement
            let (xpos, ypos) = glfw::get_cursor_pos();
            {
                if s.first_mouse {
                    s.last_x = xpos as f32;
                    s.last_y = ypos as f32;
                    s.first_mouse = false;
                }

                let xoffset = xpos as f32 - s.last_x;
                let yoffset = s.last_y - ypos as f32;

                s.last_x = xpos as f32;
                s.last_y = ypos as f32;

                s.camera.ProcessMouseMovement(xoffset, yoffset, true);
            }

            // Process mouse scroll
            let yoffset = glfw::take_scroll();
            if yoffset != 0.0 {
                s.camera.ProcessMouseScroll(yoffset as f32);
            }

            // Render
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // Bind textures
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, s.texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, s.texture2);

            // Activate shader
            s.shader.useProgram();

            // Pass projection matrix to shader
            let projection: Matrix4<f32> = perspective(Deg(s.camera.Zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            s.shader.setMat4(c_str!("projection"), &projection);

            // Camera/view transformation
            let view = s.camera.GetViewMatrix();
            s.shader.setMat4(c_str!("view"), &view);

            // Render boxes
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

pub unsafe fn reset_7_4() {
    STATE.with(|state| {
        if let Some(s) = state.borrow_mut().take() {
            gl::DeleteVertexArrays(1, &s.vao);
            gl::DeleteBuffers(1, &s.vbo);
            gl::DeleteTextures(1, &s.texture1);
            gl::DeleteTextures(1, &s.texture2);
        }
    });
}

pub fn main_1_7_4() {
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&"🎯 1_7_4: main_1_7_4() called".into());
    
    STATE.with(|state| {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!("🔍 1_7_4: STATE.is_none() = {}", state.borrow().is_none()).into());
        
        if state.borrow().is_none() {
            unsafe { init_7_4(); }
        }
    });
    
    unsafe { render_7_4(gl::glfw::get_time()); }
}
