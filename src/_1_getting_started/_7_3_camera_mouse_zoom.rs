#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

extern crate gl;
use self::gl::types::*;
use self::gl::glfw::{Key, Action};

use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::ffi::CStr;
use std::cell::RefCell;

use shader::Shader;
use image::GenericImage;
use cgmath::{Matrix4, Vector3, vec3, Deg, perspective, Point3};
use cgmath::prelude::*;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

// camera
const CAMERA_UP: Vector3<f32> = Vector3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

struct State_7_3 {
    shader: Shader,
    vao: GLuint,
    vbo: GLuint,
    texture1: GLuint,
    texture2: GLuint,
    cube_positions: Vec<Vector3<f32>>,
    camera_pos: Point3<f32>,
    camera_front: Vector3<f32>,
    yaw: f32,
    pitch: f32,
    fov: f32,
    first_mouse: bool,
    last_x: f32,
    last_y: f32,
    last_frame: f32,
}

thread_local! {
    static STATE: RefCell<Option<State_7_3>> = RefCell::new(None);
}

unsafe fn init_7_3() {
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&"🔧 1_7_3: INIT CALLED - Starting initialization".into());
    
    // Enable depth testing
    gl::Enable(gl::DEPTH_TEST);

    // Build shader
    let shader = Shader::new(
        "src/_1_getting_started/shaders/7.3.camera.vs",
        "src/_1_getting_started/shaders/7.3.camera.fs"
    );

    // Vertex data for cube
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

    // Cube positions
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

    // Store state
    STATE.with(|state| {
        *state.borrow_mut() = Some(State_7_3 {
            shader,
            vao,
            vbo,
            texture1,
            texture2,
            cube_positions,
            camera_pos: Point3::new(0.0, 0.0, 3.0),
            camera_front: vec3(0.0, 0.0, -1.0),
            yaw: -90.0,
            pitch: 0.0,
            fov: 45.0,
            first_mouse: true,
            last_x: SCR_WIDTH as f32 / 2.0,
            last_y: SCR_HEIGHT as f32 / 2.0,
            last_frame: 0.0,
        });
    });
}

unsafe fn render_7_3(time: f64) {
    STATE.with(|state| {
        if let Some(s) = state.borrow_mut().as_mut() {
            // Calculate delta time
            let current_frame = time as f32;
            let delta_time = current_frame - s.last_frame;
            s.last_frame = current_frame;

            // Process keyboard input
            let camera_speed = 2.5 * delta_time;
            if glfw::key_state(Key::W) == Action::Press {
                s.camera_pos += camera_speed * s.camera_front;
            }
            if glfw::key_state(Key::S) == Action::Press {
                s.camera_pos += -(camera_speed * s.camera_front);
            }
            if glfw::key_state(Key::A) == Action::Press {
                s.camera_pos += -(s.camera_front.cross(CAMERA_UP).normalize() * camera_speed);
            }
            if glfw::key_state(Key::D) == Action::Press {
                s.camera_pos += s.camera_front.cross(CAMERA_UP).normalize() * camera_speed;
            }

            // Mouse look (cursor position handled via WASM bindings)
            let (xpos, ypos) = glfw::get_cursor_pos();
            let (xpos, ypos) = (xpos as f32, ypos as f32);
            
            if s.first_mouse {
                s.last_x = xpos;
                s.last_y = ypos;
                s.first_mouse = false;
            }

            let xoffset = (xpos - s.last_x) * 0.1; // sensitivity
            let yoffset = (s.last_y - ypos) * 0.1; // reversed: y goes bottom to top
            s.last_x = xpos;
            s.last_y = ypos;

            s.yaw += xoffset;
            s.pitch = (s.pitch + yoffset).clamp(-89.0, 89.0);

            // Update camera front vector
            let front = Vector3 {
                x: s.yaw.to_radians().cos() * s.pitch.to_radians().cos(),
                y: s.pitch.to_radians().sin(),
                z: s.yaw.to_radians().sin() * s.pitch.to_radians().cos(),
            };
            s.camera_front = front.normalize();

            // Scroll zoom
            let scroll_delta = glfw::take_scroll() as f32;
            s.fov = (s.fov - scroll_delta).clamp(1.0, 45.0);

            // Render
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // Bind textures
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, s.texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, s.texture2);

            s.shader.useProgram();

            // Projection matrix (can change with FOV)
            let projection: Matrix4<f32> = perspective(Deg(s.fov), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            s.shader.setMat4(c_str!("projection"), &projection);

            // View matrix
            let view: Matrix4<f32> = Matrix4::look_at(s.camera_pos, s.camera_pos + s.camera_front, CAMERA_UP);
            s.shader.setMat4(c_str!("view"), &view);

            // Render 10 cubes
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

unsafe fn reset_7_3() {
    STATE.with(|state| {
        if let Some(s) = state.borrow_mut().take() {
            gl::DeleteVertexArrays(1, &s.vao);
            gl::DeleteBuffers(1, &s.vbo);
            gl::DeleteTextures(1, &s.texture1);
            gl::DeleteTextures(1, &s.texture2);
        }
    });
}

pub fn main_1_7_3() {
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&"🎯 1_7_3: main_1_7_3() called".into());
    
    STATE.with(|state| {
        let is_none = state.borrow().is_none();
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!("🔍 1_7_3: STATE.is_none() = {}", is_none).into());
        
        if is_none {
            #[cfg(target_arch = "wasm32")]
            web_sys::console::log_1(&"🚀 1_7_3: Calling init_7_3()".into());
            unsafe { init_7_3(); }
        }
    });

    let time = unsafe { gl::glfw::get_time() };
    unsafe {
        render_7_3(time);
    }
}


