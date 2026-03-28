#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use std::cell::RefCell;
use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::ffi::CStr;

use shader::Shader;
use camera::Camera;

use cgmath::{Matrix4, Deg, perspective};
use cgmath::prelude::*;

const SCR_WIDTH: u32 = 1280;
const SCR_HEIGHT: u32 = 720;

struct State_4_11 {
    shader: Shader,
    screenShader: Shader,
    cubeVAO: u32,
    cubeVBO: u32,
    quadVAO: u32,
    quadVBO: u32,
    framebuffer: u32,
    textureColorBufferMultiSampled: u32,
    rbo: u32,
    intermediateFBO: u32,
    screenTexture: u32,
}

thread_local! {
    static STATE: RefCell<Option<State_4_11>> = RefCell::new(None);
}

unsafe fn reset_4_11() {
    STATE.with(|state| {
        if let Some(s) = state.borrow_mut().take() {
            gl::DeleteVertexArrays(1, &s.cubeVAO);
            gl::DeleteBuffers(1, &s.cubeVBO);
            gl::DeleteVertexArrays(1, &s.quadVAO);
            gl::DeleteBuffers(1, &s.quadVBO);
            gl::DeleteFramebuffers(1, &s.framebuffer);
            gl::DeleteFramebuffers(1, &s.intermediateFBO);
            gl::DeleteTextures(1, &s.textureColorBufferMultiSampled);
            gl::DeleteTextures(1, &s.screenTexture);
            gl::DeleteRenderbuffers(1, &s.rbo);
        }
    });
}

unsafe fn init_4_11() {
    gl::Enable(gl::DEPTH_TEST);

    let shader = Shader::new(
        "src/_4_advanced_opengl/shaders/11.anti_aliasing.vs",
        "src/_4_advanced_opengl/shaders/11.anti_aliasing.fs");
    let screenShader = Shader::new(
        "src/_4_advanced_opengl/shaders/11.aa_post.vs",
        "src/_4_advanced_opengl/shaders/11.aa_post.fs");

    let cubeVertices: [f32; 108] = [
        -0.5, -0.5, -0.5,  0.5, -0.5, -0.5,  0.5,  0.5, -0.5,
         0.5,  0.5, -0.5, -0.5,  0.5, -0.5, -0.5, -0.5, -0.5,
        -0.5, -0.5,  0.5,  0.5, -0.5,  0.5,  0.5,  0.5,  0.5,
         0.5,  0.5,  0.5, -0.5,  0.5,  0.5, -0.5, -0.5,  0.5,
        -0.5,  0.5,  0.5, -0.5,  0.5, -0.5, -0.5, -0.5, -0.5,
        -0.5, -0.5, -0.5, -0.5, -0.5,  0.5, -0.5,  0.5,  0.5,
         0.5,  0.5,  0.5,  0.5,  0.5, -0.5,  0.5, -0.5, -0.5,
         0.5, -0.5, -0.5,  0.5, -0.5,  0.5,  0.5,  0.5,  0.5,
        -0.5, -0.5, -0.5,  0.5, -0.5, -0.5,  0.5, -0.5,  0.5,
         0.5, -0.5,  0.5, -0.5, -0.5,  0.5, -0.5, -0.5, -0.5,
        -0.5,  0.5, -0.5,  0.5,  0.5, -0.5,  0.5,  0.5,  0.5,
         0.5,  0.5,  0.5, -0.5,  0.5,  0.5, -0.5,  0.5, -0.5,
    ];
    let quadVertices: [f32; 24] = [
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
                   (cubeVertices.len() * mem::size_of::<f32>()) as isize,
                   &cubeVertices[0] as *const f32 as *const c_void,
                   gl::STATIC_DRAW);
    let stride = 3 * mem::size_of::<f32>() as i32;
    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());

    // quad VAO
    let (mut quadVAO, mut quadVBO) = (0, 0);
    gl::GenVertexArrays(1, &mut quadVAO);
    gl::GenBuffers(1, &mut quadVBO);
    gl::BindVertexArray(quadVAO);
    gl::BindBuffer(gl::ARRAY_BUFFER, quadVBO);
    gl::BufferData(gl::ARRAY_BUFFER,
                   (quadVertices.len() * mem::size_of::<f32>()) as isize,
                   &quadVertices[0] as *const f32 as *const c_void,
                   gl::STATIC_DRAW);
    gl::EnableVertexAttribArray(0);
    let stride = 4 * mem::size_of::<f32>() as i32;
    gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, ptr::null());
    gl::EnableVertexAttribArray(1);
    gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (2 * mem::size_of::<f32>()) as *const c_void);

    // MSAA framebuffer
    let mut framebuffer = 0;
    gl::GenFramebuffers(1, &mut framebuffer);
    gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);

    let mut textureColorBufferMultiSampled = 0;
    gl::GenTextures(1, &mut textureColorBufferMultiSampled);
    gl::BindTexture(gl::TEXTURE_2D_MULTISAMPLE, textureColorBufferMultiSampled);
    gl::TexImage2DMultisample(gl::TEXTURE_2D_MULTISAMPLE, 4, gl::RGB, SCR_WIDTH as i32, SCR_HEIGHT as i32, gl::TRUE);
    gl::BindTexture(gl::TEXTURE_2D_MULTISAMPLE, 0);
    gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D_MULTISAMPLE, textureColorBufferMultiSampled, 0);

    let mut rbo = 0;
    gl::GenRenderbuffers(1, &mut rbo);
    gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
    gl::RenderbufferStorageMultisample(gl::RENDERBUFFER, 4, gl::DEPTH24_STENCIL8, SCR_WIDTH as i32, SCR_HEIGHT as i32);
    gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
    gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, rbo);

    if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::error_1(&"ERROR::FRAMEBUFFER:: Framebuffer is not complete!".into());
    }
    gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

    // Intermediate FBO (for blit target)
    let mut intermediateFBO = 0;
    gl::GenFramebuffers(1, &mut intermediateFBO);
    gl::BindFramebuffer(gl::FRAMEBUFFER, intermediateFBO);

    let mut screenTexture = 0;
    gl::GenTextures(1, &mut screenTexture);
    gl::BindTexture(gl::TEXTURE_2D, screenTexture);
    gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, SCR_WIDTH as i32, SCR_HEIGHT as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, ptr::null());
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, screenTexture, 0);

    if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::error_1(&"ERROR::FRAMEBUFFER:: Intermediate framebuffer is not complete!".into());
    }
    gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

    screenShader.useProgram();
    screenShader.setInt(c_str!("screenTexture"), 0);

    STATE.with(|state| {
        *state.borrow_mut() = Some(State_4_11 {
            shader,
            screenShader,
            cubeVAO,
            cubeVBO,
            quadVAO,
            quadVBO,
            framebuffer,
            textureColorBufferMultiSampled,
            rbo,
            intermediateFBO,
            screenTexture,
        });
    });
}

unsafe fn render_4_11(camera: &Camera) {
    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            // 1. draw scene in multisampled FBO
            gl::BindFramebuffer(gl::FRAMEBUFFER, s.framebuffer);
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);

            s.shader.useProgram();
            let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            s.shader.setMat4(c_str!("projection"), &projection);
            s.shader.setMat4(c_str!("view"), &camera.GetViewMatrix());
            s.shader.setMat4(c_str!("model"), &Matrix4::identity());

            gl::BindVertexArray(s.cubeVAO);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            // 2. blit multisampled buffer to intermediate FBO
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, s.framebuffer);
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, s.intermediateFBO);
            gl::BlitFramebuffer(0, 0, SCR_WIDTH as i32, SCR_HEIGHT as i32,
                               0, 0, SCR_WIDTH as i32, SCR_HEIGHT as i32,
                               gl::COLOR_BUFFER_BIT, gl::NEAREST);

            // 3. render quad with resolved texture
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Disable(gl::DEPTH_TEST);

            s.screenShader.useProgram();
            gl::BindVertexArray(s.quadVAO);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, s.screenTexture);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    });
}

pub fn main_4_11() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe { init_4_11(); }
        }
    });
    unsafe {
        render_4_11(&Camera::default());
    }
}
