#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use std::cell::RefCell;
use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::path::Path;
use std::ffi::CStr;
use gl::types::*;

use cgmath::{Matrix4, Matrix3, Deg, perspective, Point3};
use cgmath::prelude::*;

use image::GenericImage;

use shader::Shader;
use camera::Camera;

const SCR_WIDTH: u32 = 1280;
const SCR_HEIGHT: u32 = 720;

struct State_4_6_2 {
    shader: Shader,
    skyboxShader: Shader,
    cubeVAO: GLuint,
    cubeVBO: GLuint,
    skyboxVAO: GLuint,
    skyboxVBO: GLuint,
    cubemapTexture: GLuint,
}

thread_local! {
    static STATE: RefCell<Option<State_4_6_2>> = RefCell::new(None);
}

unsafe fn reset_4_6_2() {
    STATE.with(|state| {
        if let Some(s) = state.borrow_mut().take() {
            gl::DeleteVertexArrays(1, &s.cubeVAO);
            gl::DeleteVertexArrays(1, &s.skyboxVAO);
            gl::DeleteBuffers(1, &s.cubeVBO);
            gl::DeleteBuffers(1, &s.skyboxVBO);
            gl::DeleteTextures(1, &s.cubemapTexture);
        }
    });
}

unsafe fn init_4_6_2() {
    gl::Enable(gl::DEPTH_TEST);

    let shader = Shader::new(
        "src/_4_advanced_opengl/shaders/6.2.cubemaps.vs",
        "src/_4_advanced_opengl/shaders/6.2.cubemaps.fs");
    let skyboxShader = Shader::new(
        "src/_4_advanced_opengl/shaders/6.2.skybox.vs",
        "src/_4_advanced_opengl/shaders/6.2.skybox.fs");

    // set up vertex data
        let cubeVertices: [f32; 216] = [
            // positions       // normals
            -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,
             0.5, -0.5, -0.5,  0.0,  0.0, -1.0,
             0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
             0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
            -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
            -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,

            -0.5, -0.5,  0.5,  0.0,  0.0, 1.0,
             0.5, -0.5,  0.5,  0.0,  0.0, 1.0,
             0.5,  0.5,  0.5,  0.0,  0.0, 1.0,
             0.5,  0.5,  0.5,  0.0,  0.0, 1.0,
            -0.5,  0.5,  0.5,  0.0,  0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0,  0.0, 1.0,

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
        let skyboxVertices: [f32; 108] = [
            // positions
            -1.0,  1.0, -1.0,
            -1.0, -1.0, -1.0,
             1.0, -1.0, -1.0,
             1.0, -1.0, -1.0,
             1.0,  1.0, -1.0,
            -1.0,  1.0, -1.0,

            -1.0, -1.0,  1.0,
            -1.0, -1.0, -1.0,
            -1.0,  1.0, -1.0,
            -1.0,  1.0, -1.0,
            -1.0,  1.0,  1.0,
            -1.0, -1.0,  1.0,

             1.0, -1.0, -1.0,
             1.0, -1.0,  1.0,
             1.0,  1.0,  1.0,
             1.0,  1.0,  1.0,
             1.0,  1.0, -1.0,
             1.0, -1.0, -1.0,

            -1.0, -1.0,  1.0,
            -1.0,  1.0,  1.0,
             1.0,  1.0,  1.0,
             1.0,  1.0,  1.0,
             1.0, -1.0,  1.0,
            -1.0, -1.0,  1.0,

            -1.0,  1.0, -1.0,
             1.0,  1.0, -1.0,
             1.0,  1.0,  1.0,
             1.0,  1.0,  1.0,
            -1.0,  1.0,  1.0,
            -1.0,  1.0, -1.0,

            -1.0, -1.0, -1.0,
            -1.0, -1.0,  1.0,
             1.0, -1.0, -1.0,
             1.0, -1.0, -1.0,
            -1.0, -1.0,  1.0,
             1.0, -1.0,  1.0
        ];

        // cube VAO
        let (mut cubeVAO, mut cubeVBO) = (0, 0);
        gl::GenVertexArrays(1, &mut cubeVAO);
        gl::GenBuffers(1, &mut cubeVBO);
        gl::BindVertexArray(cubeVAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, cubeVBO);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (cubeVertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &cubeVertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);
        let mut stride = 6 * mem::size_of::<GLfloat>() as GLsizei;
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        // skybox VAO
        let (mut skyboxVAO, mut skyboxVBO) = (0, 0);
        gl::GenVertexArrays(1, &mut skyboxVAO);
        gl::GenBuffers(1, &mut skyboxVBO);
        gl::BindVertexArray(skyboxVAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, skyboxVBO);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (skyboxVertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &skyboxVertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);
        gl::EnableVertexAttribArray(0);
        stride = 3 * mem::size_of::<GLfloat>() as GLsizei;
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());

        // load textures
        // -------------
        let faces = [
            "resources/textures/skybox/right.jpg",
            "resources/textures/skybox/left.jpg",
            "resources/textures/skybox/top.jpg",
            "resources/textures/skybox/bottom.jpg",
            "resources/textures/skybox/back.jpg",
            "resources/textures/skybox/front.jpg"
        ];
        let cubemapTexture = loadCubemap(&faces);

        // shader configuration
        shader.useProgram();
        shader.setInt(c_str!("skybox"), 0);
        skyboxShader.useProgram();
        skyboxShader.setInt(c_str!("skybox"), 0);

        STATE.with(|state| {
            *state.borrow_mut() = Some(State_4_6_2 {
                shader,
                skyboxShader,
                cubeVAO,
                cubeVBO,
                skyboxVAO,
                skyboxVBO,
                cubemapTexture,
            });
        });
}

unsafe fn render_4_6_2(camera: &Camera) {
    gl::ClearColor(0.1, 0.1, 0.1, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);

            // Render reflective cube
            s.shader.useProgram();
            let model: Matrix4<f32> = Matrix4::identity();
            let view = camera.GetViewMatrix();
            s.shader.setMat4(c_str!("model"), &model);
            s.shader.setMat4(c_str!("view"), &view);
            s.shader.setMat4(c_str!("projection"), &projection);
            s.shader.setVector3(c_str!("cameraPos"), &camera.Position.to_vec());

            gl::BindVertexArray(s.cubeVAO);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, s.cubemapTexture);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            gl::BindVertexArray(0);

            // Render skybox last
            gl::DepthFunc(gl::LEQUAL);
            s.skyboxShader.useProgram();

            // Remove translation from view matrix
            let skyboxView = Matrix3::from_cols(view.x.truncate(), view.y.truncate(), view.z.truncate());
            let skyboxView4 = Matrix4::from(skyboxView);
            
            s.skyboxShader.setMat4(c_str!("view"), &skyboxView4);
            s.skyboxShader.setMat4(c_str!("projection"), &projection);

            gl::BindVertexArray(s.skyboxVAO);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, s.cubemapTexture);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            gl::BindVertexArray(0);
            gl::DepthFunc(gl::LESS);
        }
    });
}

pub fn main_4_6_2() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe {
                init_4_6_2();
            }
        }
    });

    unsafe {
        render_4_6_2(&Camera::default());
    }
}

unsafe fn loadCubemap(faces: &[&str]) -> u32 {
    let mut textureID = 0;
    gl::GenTextures(1, &mut textureID);
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, textureID);

    for (i, face) in faces.iter().enumerate() {
        #[cfg(target_arch = "wasm32")]
        let img = {
            use gl::resources::load_bytes_sync;
            let bytes = load_bytes_sync(face).expect("Failed to load cubemap bytes");
            image::load_from_memory(&bytes).expect("Cubemap texture failed to load")
        };
        
        #[cfg(not(target_arch = "wasm32"))]
        let img = image::open(&Path::new(face)).expect("Cubemap texture failed to load");

        let data = img.to_rgb().into_raw();
        gl::TexImage2D(
            gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
            0, gl::RGB as i32, img.width() as i32, img.height() as i32,
            0, gl::RGB, gl::UNSIGNED_BYTE,
            &data[0] as *const u8 as *const c_void);
    }

    gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
    gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
    gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);

    textureID
}
