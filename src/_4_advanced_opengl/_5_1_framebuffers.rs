#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use std::cell::RefCell;
use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::ffi::CStr;
use gl::types::*;

use common::loadTexture;
use shader::Shader;
use camera::Camera;

use cgmath::{Matrix4, vec3, Deg, perspective};
use cgmath::prelude::*;

const SCR_WIDTH: u32 = 1280;
const SCR_HEIGHT: u32 = 720;

struct State_4_5_1 {
    shader: Shader,
    screenShader: Shader,
    cubeVAO: GLuint,
    cubeVBO: GLuint,
    planeVAO: GLuint,
    planeVBO: GLuint,
    quadVAO: GLuint,
    quadVBO: GLuint,
    cubeTexture: GLuint,
    floorTexture: GLuint,
    framebuffer: GLuint,
    textureColorbuffer: GLuint,
    rbo: GLuint,
}

thread_local! {
    static STATE: RefCell<Option<State_4_5_1>> = RefCell::new(None);
}

unsafe fn reset_4_5_1() {
    STATE.with(|state| {
        if let Some(s) = state.borrow_mut().take() {
            gl::DeleteVertexArrays(1, &s.cubeVAO);
            gl::DeleteVertexArrays(1, &s.planeVAO);
            gl::DeleteVertexArrays(1, &s.quadVAO);
            gl::DeleteBuffers(1, &s.cubeVBO);
            gl::DeleteBuffers(1, &s.planeVBO);
            gl::DeleteBuffers(1, &s.quadVBO);
            gl::DeleteTextures(1, &s.cubeTexture);
            gl::DeleteTextures(1, &s.floorTexture);
            gl::DeleteFramebuffers(1, &s.framebuffer);
            gl::DeleteTextures(1, &s.textureColorbuffer);
            gl::DeleteRenderbuffers(1, &s.rbo);
        }
    });
}

unsafe fn init_4_5_1() {
    gl::Enable(gl::DEPTH_TEST);

    let shader = Shader::new(
        "src/_4_advanced_opengl/shaders/5.1.framebuffers.vs",
        "src/_4_advanced_opengl/shaders/5.1.framebuffers.fs");
    let screenShader = Shader::new(
        "src/_4_advanced_opengl/shaders/5.1.framebuffers_screen.vs",
        "src/_4_advanced_opengl/shaders/5.1.framebuffers_screen.fs");

    // set up vertex data
        let cubeVertices: [f32; 180] = [
             // positions       // texture Coords
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
        let planeVertices: [f32; 30] = [
            // positions       // texture Coords (note we set these higher than 1 (together with GL_REPEAT as texture wrapping mode). this will cause the floor texture to repeat)
             5.0, -0.5,  5.0,  2.0, 0.0,
            -5.0, -0.5,  5.0,  0.0, 0.0,
            -5.0, -0.5, -5.0,  0.0, 2.0,

             5.0, -0.5,  5.0,  2.0, 0.0,
            -5.0, -0.5, -5.0,  0.0, 2.0,
             5.0, -0.5, -5.0,  2.0, 2.0
        ];
        let quadVertices: [f32; 24] = [
            // positions  // texCoords
            -1.0,  1.0,  0.0, 1.0,
            -1.0, -1.0,  0.0, 0.0,
             1.0, -1.0,  1.0, 0.0,
            -1.0,  1.0,  0.0, 1.0,
             1.0, -1.0,  1.0, 0.0,
             1.0,  1.0,  1.0, 1.0
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
        let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::BindVertexArray(0);
        // plane VAO
        let (mut planeVAO, mut planeVBO) = (0, 0);
        gl::GenVertexArrays(1, &mut planeVAO);
        gl::GenBuffers(1, &mut planeVBO);
        gl::BindVertexArray(planeVAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, planeVBO);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (planeVertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &planeVertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        // screen quad VAO
        let (mut quadVAO, mut quadVBO) = (0, 0);
        gl::GenVertexArrays(1, &mut quadVAO);
        gl::GenBuffers(1, &mut quadVBO);
        gl::BindVertexArray(quadVBO);
        gl::BindBuffer(gl::ARRAY_BUFFER, quadVBO);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (quadVertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &quadVertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);
        gl::EnableVertexAttribArray(0);
        let stride = 4 * mem::size_of::<GLfloat>() as GLsizei;
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (2 * mem::size_of::<GLfloat>()) as *const c_void);

        // load textures
        let cubeTexture = loadTexture("resources/textures/container.jpg");
        let floorTexture = loadTexture("resources/textures/metal.png");

        // shader configuration
        shader.useProgram();
        shader.setInt(c_str!("texture1"), 0);
        screenShader.useProgram();
        screenShader.setInt(c_str!("screenTexture"), 0);

        // framebuffer configuration
        let mut framebuffer = 0;
        gl::GenFramebuffers(1, &mut framebuffer);
        gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);

        // create a color attachment texture
        let mut textureColorbuffer = 0;
        gl::GenTextures(1, &mut textureColorbuffer);
        gl::BindTexture(gl::TEXTURE_2D, textureColorbuffer);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, SCR_WIDTH as i32, SCR_HEIGHT as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, ptr::null());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, textureColorbuffer, 0);

        // create a renderbuffer object for depth and stencil attachment
        let mut rbo = 0;
        gl::GenRenderbuffers(1, &mut rbo);
        gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
        gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, SCR_WIDTH as i32, SCR_HEIGHT as i32);
        gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, rbo);

        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            println!("ERROR::FRAMEBUFFER:: Framebuffer is not complete!");
        }
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

        STATE.with(|state| {
            *state.borrow_mut() = Some(State_4_5_1 {
                shader,
                screenShader,
                cubeVAO,
                cubeVBO,
                planeVAO,
                planeVBO,
                quadVAO,
                quadVBO,
                cubeTexture,
                floorTexture,
                framebuffer,
                textureColorbuffer,
                rbo,
            });
        });
}

unsafe fn render_4_5_1(camera: &Camera) {
    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            // First pass: render to framebuffer
            gl::BindFramebuffer(gl::FRAMEBUFFER, s.framebuffer);
            gl::Enable(gl::DEPTH_TEST);
            
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            s.shader.useProgram();
            let view = camera.GetViewMatrix();
            let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            s.shader.setMat4(c_str!("view"), &view);
            s.shader.setMat4(c_str!("projection"), &projection);

            // cubes
            gl::BindVertexArray(s.cubeVAO);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, s.cubeTexture);
            let mut model = Matrix4::from_translation(vec3(-1.0, 0.0, -1.0));
            s.shader.setMat4(c_str!("model"), &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            model = Matrix4::from_translation(vec3(2.0, 0.0, 0.0));
            s.shader.setMat4(c_str!("model"), &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            // floor
            gl::BindVertexArray(s.planeVAO);
            gl::BindTexture(gl::TEXTURE_2D, s.floorTexture);
            s.shader.setMat4(c_str!("model"), &Matrix4::identity());
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);

            // Second pass: render to screen with post-processing
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::Disable(gl::DEPTH_TEST);
            
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            s.screenShader.useProgram();
            gl::BindVertexArray(s.quadVAO);
            gl::BindTexture(gl::TEXTURE_2D, s.textureColorbuffer);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    });
}

pub fn main_4_5_1() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe {
                init_4_5_1();
            }
        }
    });

    unsafe {
        render_4_5_1(&Camera::default());
    }
}
