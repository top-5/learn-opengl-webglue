#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

extern crate gl;
use self::gl::types::*;

use std::cell::RefCell;
use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::ffi::{CStr, CString};

use common::loadTexture;
use shader::Shader;
use camera::Camera;

use cgmath::{Matrix4, vec3, Vector3, Deg, perspective, Point3};
use cgmath::prelude::*;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

struct State_5_6 {
    shader: Shader,
    hdrShader: Shader,
    woodTexture: u32,
    hdrFBO: u32,
    colorBuffer: u32,
    cubeVAO: u32,
    cubeVBO: u32,
    quadVAO: u32,
    quadVBO: u32,
    lightPositions: Vec<Vector3<f32>>,
    lightColors: Vec<Vector3<f32>>,
    hdr: bool,
    exposure: f32,
}

thread_local! {
    static STATE: RefCell<Option<State_5_6>> = RefCell::new(None);
}

pub unsafe fn reset_5_6() {
    STATE.with(|state| {
        if let Some(s) = state.borrow_mut().take() {
            gl::DeleteVertexArrays(1, &s.cubeVAO);
            gl::DeleteBuffers(1, &s.cubeVBO);
            gl::DeleteVertexArrays(1, &s.quadVAO);
            gl::DeleteBuffers(1, &s.quadVBO);
            gl::DeleteTextures(1, &s.woodTexture);
            gl::DeleteTextures(1, &s.colorBuffer);
            gl::DeleteFramebuffers(1, &s.hdrFBO);
        }
    });
}

unsafe fn init_5_6() {
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&"🔧 5_6: INIT CALLED - Starting HDR initialization".into());

    gl::Enable(gl::DEPTH_TEST);

    let shader = Shader::new(
        "src/_5_advanced_lighting/shaders/6.lighting.vs",
        "src/_5_advanced_lighting/shaders/6.lighting.fs");

    let hdrShader = Shader::new(
        "src/_5_advanced_lighting/shaders/6.hdr.vs",
        "src/_5_advanced_lighting/shaders/6.hdr.fs");

    let woodTexture = loadTexture("resources/textures/wood.png");

    // configure floating point framebuffer
    // ------------------------------------
    let mut hdrFBO = 0;
    gl::GenFramebuffers(1, &mut hdrFBO);
    gl::BindFramebuffer(gl::FRAMEBUFFER, hdrFBO);
    
    // create floating point color buffer
    let mut colorBuffer = 0;
    gl::GenTextures(1, &mut colorBuffer);
    gl::BindTexture(gl::TEXTURE_2D, colorBuffer);
    gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA16F as i32,
        SCR_WIDTH as i32, SCR_HEIGHT as i32, 0, gl::RGBA, gl::FLOAT, ptr::null());
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    // create depth buffer (renderbuffer)
    let mut rboDepth = 0;
    gl::GenRenderbuffers(1, &mut rboDepth);
    gl::BindRenderbuffer(gl::RENDERBUFFER, rboDepth);
    gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT, SCR_WIDTH as i32, SCR_HEIGHT as i32);
    
    // attach buffers
    gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, colorBuffer, 0);
    gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, rboDepth);
    if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&"❌ 5_6: Framebuffer not complete!".into());
    }
    gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

    // lighting info
    // -------------
    let mut lightPositions: Vec<Vector3<f32>> = Vec::new();
    lightPositions.push(vec3( 0.0,  0.0, 49.5)); // back light
    lightPositions.push(vec3(-1.4, -1.9, 9.0));
    lightPositions.push(vec3( 0.0, -1.8, 4.0));
    lightPositions.push(vec3( 0.8, -1.7, 6.0));
    
    let mut lightColors: Vec<Vector3<f32>> = Vec::new();
    lightColors.push(vec3(200.0, 200.0, 200.0));
    lightColors.push(vec3(0.1, 0.0, 0.0));
    lightColors.push(vec3(0.0, 0.0, 0.2));
    lightColors.push(vec3(0.0, 0.1, 0.0));

    // shader configuration
    // --------------------
    shader.useProgram();
    shader.setInt(c_str!("diffuseTexture"), 0);
    hdrShader.useProgram();
    hdrShader.setInt(c_str!("hdrBuffer"), 0);

    STATE.with(|state| {
        *state.borrow_mut() = Some(State_5_6 {
            shader,
            hdrShader,
            woodTexture,
            hdrFBO,
            colorBuffer,
            cubeVAO: 0,
            cubeVBO: 0,
            quadVAO: 0,
            quadVBO: 0,
            lightPositions,
            lightColors,
            hdr: true,
            exposure: 1.0,
        });
    });

    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&"✅ 5_6: INIT COMPLETE - HDR initialized successfully".into());
}

unsafe fn render_5_6(camera: &Camera) {
    STATE.with(|state| {
        if let Some(ref mut s) = *state.borrow_mut() {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // 1. render scene into floating point framebuffer
            // -----------------------------------------------
            gl::BindFramebuffer(gl::FRAMEBUFFER, s.hdrFBO);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            
            let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            let view = camera.GetViewMatrix();
            
            s.shader.useProgram();
            s.shader.setMat4(c_str!("projection"), &projection);
            s.shader.setMat4(c_str!("view"), &view);
            
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, s.woodTexture);
            
            // set lighting uniforms
            for (i, lightPos) in s.lightPositions.iter().enumerate() {
                let name = CString::new(format!("lights[{}].Position", i)).unwrap();
                s.shader.setVector3(&name, lightPos);
                let name = CString::new(format!("lights[{}].Color", i)).unwrap();
                s.shader.setVector3(&name, &s.lightColors[i]);
            }
            s.shader.setVector3(c_str!("viewPos"), &camera.Position.to_vec());
            
            // render tunnel (simplified for performance)
            let mut model: Matrix4<f32> = Matrix4::from_translation(vec3(0.0, 0.0, 25.0));
            model = model * Matrix4::from_nonuniform_scale(2.5, 2.5, 27.5);
            s.shader.setMat4(c_str!("model"), &model);
            s.shader.setBool(c_str!("inverse_normals"), true);
            renderCube(&mut s.cubeVAO, &mut s.cubeVBO);
            
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            // 2. render floating point color buffer to 2D quad and tonemap HDR colors
            // -----------------------------------------------------------------------
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            s.hdrShader.useProgram();
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, s.colorBuffer);
            s.hdrShader.setBool(c_str!("hdr"), s.hdr);
            s.hdrShader.setFloat(c_str!("exposure"), s.exposure);
            renderQuad(&mut s.quadVAO, &mut s.quadVBO);
        }
    });
}

pub fn main_5_6() {
    static INIT_DONE: std::sync::Once = std::sync::Once::new();
    INIT_DONE.call_once(|| {
        unsafe { init_5_6(); }
    });
    
    unsafe { render_5_6(&Camera::default()); }
}

// renderCube() renders a 1x1 3D cube in NDC.
// -------------------------------------------------
unsafe fn renderCube(cubeVAO: &mut u32, cubeVBO: &mut u32) {
    if *cubeVAO == 0 {
        let vertices: [f32; 288] = [
            // back face
            -1.0, -1.0, -1.0,  0.0,  0.0, -1.0, 0.0, 0.0, // bottom-left
             1.0,  1.0, -1.0,  0.0,  0.0, -1.0, 1.0, 1.0, // top-right
             1.0, -1.0, -1.0,  0.0,  0.0, -1.0, 1.0, 0.0, // bottom-right
             1.0,  1.0, -1.0,  0.0,  0.0, -1.0, 1.0, 1.0, // top-right
            -1.0, -1.0, -1.0,  0.0,  0.0, -1.0, 0.0, 0.0, // bottom-left
            -1.0,  1.0, -1.0,  0.0,  0.0, -1.0, 0.0, 1.0, // top-left
            // front face
            -1.0, -1.0,  1.0,  0.0,  0.0,  1.0, 0.0, 0.0, // bottom-left
             1.0, -1.0,  1.0,  0.0,  0.0,  1.0, 1.0, 0.0, // bottom-right
             1.0,  1.0,  1.0,  0.0,  0.0,  1.0, 1.0, 1.0, // top-right
             1.0,  1.0,  1.0,  0.0,  0.0,  1.0, 1.0, 1.0, // top-right
            -1.0,  1.0,  1.0,  0.0,  0.0,  1.0, 0.0, 1.0, // top-left
            -1.0, -1.0,  1.0,  0.0,  0.0,  1.0, 0.0, 0.0, // bottom-left
            // left face
            -1.0,  1.0,  1.0, -1.0,  0.0,  0.0, 1.0, 0.0, // top-right
            -1.0,  1.0, -1.0, -1.0,  0.0,  0.0, 1.0, 1.0, // top-left
            -1.0, -1.0, -1.0, -1.0,  0.0,  0.0, 0.0, 1.0, // bottom-left
            -1.0, -1.0, -1.0, -1.0,  0.0,  0.0, 0.0, 1.0, // bottom-left
            -1.0, -1.0,  1.0, -1.0,  0.0,  0.0, 0.0, 0.0, // bottom-right
            -1.0,  1.0,  1.0, -1.0,  0.0,  0.0, 1.0, 0.0, // top-right
            // right face
             1.0,  1.0,  1.0,  1.0,  0.0,  0.0, 1.0, 0.0, // top-left
             1.0, -1.0, -1.0,  1.0,  0.0,  0.0, 0.0, 1.0, // bottom-right
             1.0,  1.0, -1.0,  1.0,  0.0,  0.0, 1.0, 1.0, // top-right
             1.0, -1.0, -1.0,  1.0,  0.0,  0.0, 0.0, 1.0, // bottom-right
             1.0,  1.0,  1.0,  1.0,  0.0,  0.0, 1.0, 0.0, // top-left
             1.0, -1.0,  1.0,  1.0,  0.0,  0.0, 0.0, 0.0, // bottom-left
            // bottom face
            -1.0, -1.0, -1.0,  0.0, -1.0,  0.0, 0.0, 1.0, // top-right
             1.0, -1.0, -1.0,  0.0, -1.0,  0.0, 1.0, 1.0, // top-left
             1.0, -1.0,  1.0,  0.0, -1.0,  0.0, 1.0, 0.0, // bottom-left
             1.0, -1.0,  1.0,  0.0, -1.0,  0.0, 1.0, 0.0, // bottom-left
            -1.0, -1.0,  1.0,  0.0, -1.0,  0.0, 0.0, 0.0, // bottom-right
            -1.0, -1.0, -1.0,  0.0, -1.0,  0.0, 0.0, 1.0, // top-right
            // top face
            -1.0,  1.0, -1.0,  0.0,  1.0,  0.0, 0.0, 1.0, // top-left
             1.0,  1.0, 1.0,  0.0,  1.0,  0.0, 1.0, 0.0, // bottom-right
             1.0,  1.0, -1.0,  0.0,  1.0,  0.0, 1.0, 1.0, // top-right
             1.0,  1.0,  1.0,  0.0,  1.0,  0.0, 1.0, 0.0, // bottom-right
            -1.0,  1.0, -1.0,  0.0,  1.0,  0.0, 0.0, 1.0, // top-left
            -1.0,  1.0,  1.0,  0.0,  1.0,  0.0, 0.0, 0.0  // bottom-left
        ];
        
        gl::GenVertexArrays(1, cubeVAO);
        gl::GenBuffers(1, cubeVBO);
        
        gl::BindBuffer(gl::ARRAY_BUFFER, *cubeVBO);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &vertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);
        
        gl::BindVertexArray(*cubeVAO);
        gl::EnableVertexAttribArray(0);
        let stride = 8 * mem::size_of::<GLfloat>() as GLsizei;
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (6 * mem::size_of::<GLfloat>()) as *const c_void);
    }
    
    gl::BindVertexArray(*cubeVAO);
    gl::DrawArrays(gl::TRIANGLES, 0, 36);
    gl::BindVertexArray(0);
}

// renderQuad() renders a 1x1 XY quad in NDC
// -----------------------------------------
unsafe fn renderQuad(quadVAO: &mut u32, quadVBO: &mut u32) {
    if *quadVAO == 0 {
        let quadVertices: [f32; 20] = [
            // positions        // texture Coords
            -1.0,  1.0, 0.0,    0.0, 1.0,
            -1.0, -1.0, 0.0,    0.0, 0.0,
             1.0,  1.0, 0.0,    1.0, 1.0,
             1.0, -1.0, 0.0,    1.0, 0.0,
        ];
        
        gl::GenVertexArrays(1, quadVAO);
        gl::GenBuffers(1, quadVBO);
        
        gl::BindVertexArray(*quadVAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, *quadVBO);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (quadVertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &quadVertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);
        
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 5 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 5 * mem::size_of::<GLfloat>() as GLsizei, (3 * mem::size_of::<GLfloat>()) as *const c_void);
    }
    
    gl::BindVertexArray(*quadVAO);
    gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
    gl::BindVertexArray(0);
}