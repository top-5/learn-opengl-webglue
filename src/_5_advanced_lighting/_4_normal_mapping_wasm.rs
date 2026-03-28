#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

extern crate gl;
use self::gl::types::*;

use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::ffi::CStr;

use common::loadTexture;
use shader::Shader;

use cgmath::{Matrix4, vec3, Vector3, vec2, Vector2, Deg, perspective, Point3};
use cgmath::prelude::*;

use std::cell::RefCell;

// Global state for WASM compatibility
thread_local! {
    static STATE_5_4: RefCell<Option<State_5_4>> = RefCell::new(None);
}

struct State_5_4 {
    shader: Shader,
    diffuseMap: u32,
    normalMap: u32,
    quadVAO: u32,
    quadVBO: u32,
    camera_pos: Point3<f32>,
    camera_front: Vector3<f32>,
    camera_up: Vector3<f32>,
    camera_zoom: f32,
}

// WASM-compatible initialization function
pub unsafe fn init_5_4() {
    // configure global opengl state
    gl::Enable(gl::DEPTH_TEST);

    // build and compile our shader program
    let shader = Shader::new(
        "shaders/5.normal_mapping.vs",
        "shaders/5.normal_mapping.fs");

    // load textures
    let diffuseMap = loadTexture("resources/textures/brickwall.jpg");
    let normalMap = loadTexture("resources/textures/brickwall_normal.jpg");

    // shader configuration
    shader.useProgram();
    shader.setInt(c_str!("diffuseMap"), 0);
    shader.setInt(c_str!("normalMap"), 1);

    let mut quadVAO = 0;
    let mut quadVBO = 0;

    // Initialize the render quad
    renderQuad(&mut quadVAO, &mut quadVBO);

    let state = State_5_4 {
        shader,
        diffuseMap,
        normalMap,
        quadVAO,
        quadVBO,
        camera_pos: Point3::new(0.0, 0.0, 3.0),
        camera_front: vec3(0.0, 0.0, -1.0),
        camera_up: vec3(0.0, 1.0, 0.0),
        camera_zoom: 45.0,
    };

    STATE_5_4.with(|s| {
        *s.borrow_mut() = Some(state);
    });
}

// WASM-compatible render function
pub unsafe fn render_5_4(time: f64) {
    STATE_5_4.with(|s| {
        if let Some(ref mut state) = *s.borrow_mut() {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // configure view/projection matrices
            let projection: Matrix4<f32> = perspective(Deg(state.camera_zoom), 800.0 / 600.0, 0.1, 100.0);
            
            // Create view matrix from camera state
            let view = Matrix4::look_at(
                state.camera_pos,
                state.camera_pos + state.camera_front.cast().unwrap(),
                state.camera_up
            );

            state.shader.useProgram();
            state.shader.setMat4(c_str!("projection"), &projection);
            state.shader.setMat4(c_str!("view"), &view);

            // lighting info
            let lightPos: Vector3<f32> = vec3(0.5, 1.0, 0.3);

            // render normal-mapped quad
            let model: Matrix4<f32> = Matrix4::from_axis_angle(
                vec3(1.0, 0.0, 1.0).normalize(), 
                Deg(time as f32 * -10.0)
            );
            state.shader.setMat4(c_str!("model"), &model);
            state.shader.setVector3(c_str!("viewPos"), &state.camera_pos.to_vec());
            state.shader.setVector3(c_str!("lightPos"), &lightPos);
            
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, state.diffuseMap);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, state.normalMap);
            
            renderQuad(&mut state.quadVAO, &mut state.quadVBO);

            // render light source (simply re-renders a smaller plane at the light's position for debugging/visualization)
            let light_model = Matrix4::from_translation(lightPos) * Matrix4::from_scale(0.1);
            state.shader.setMat4(c_str!("model"), &light_model);
            renderQuad(&mut state.quadVAO, &mut state.quadVBO);
        }
    });
}

// renders a 1x1 quad in NDC with manually calculated tangent vectors
// ------------------------------------------------------------------
unsafe fn renderQuad(quadVAO: &mut u32, quadVBO: &mut u32) {
    if *quadVAO == 0 {
        // positions
        let pos1: Vector3<f32> = vec3(-1.0,  1.0, 0.0);
        let pos2: Vector3<f32> = vec3(-1.0, -1.0, 0.0);
        let pos3: Vector3<f32> = vec3( 1.0, -1.0, 0.0);
        let pos4: Vector3<f32> = vec3( 1.0,  1.0, 0.0);
        // texture coordinates
        let uv1: Vector2<f32> = vec2(0.0, 1.0);
        let uv2: Vector2<f32> = vec2(0.0, 0.0);
        let uv3: Vector2<f32> = vec2(1.0, 0.0);
        let uv4: Vector2<f32> = vec2(1.0, 1.0);
        // normal vector
        let nm: Vector3<f32> = vec3(0.0, 0.0, 1.0);

        // calculate tangent/bitangent vectors of both triangles
        let mut tangent1: Vector3<f32> = vec3(0.0, 0.0, 0.0);
        let mut bitangent1: Vector3<f32> = vec3(0.0, 0.0, 0.0);
        let mut tangent2: Vector3<f32> = vec3(0.0, 0.0, 0.0);
        let mut bitangent2: Vector3<f32> = vec3(0.0, 0.0, 0.0);
        // triangle 1
        // ----------
        let mut edge1 = pos2 - pos1;
        let mut edge2 = pos3 - pos1;
        let mut deltaUV1 = uv2 - uv1;
        let mut deltaUV2 = uv3 - uv1;

        let mut f = 1.0 / (deltaUV1.x * deltaUV2.y - deltaUV2.x * deltaUV1.y);

        tangent1.x = f * (deltaUV2.y * edge1.x - deltaUV1.y * edge2.x);
        tangent1.y = f * (deltaUV2.y * edge1.y - deltaUV1.y * edge2.y);
        tangent1.z = f * (deltaUV2.y * edge1.z - deltaUV1.y * edge2.z);
        tangent1 = tangent1.normalize();

        bitangent1.x = f * (-deltaUV2.x * edge1.x + deltaUV1.x * edge2.x);
        bitangent1.y = f * (-deltaUV2.x * edge1.y + deltaUV1.x * edge2.y);
        bitangent1.z = f * (-deltaUV2.x * edge1.z + deltaUV1.x * edge2.z);
        bitangent1 = bitangent1.normalize();

        // triangle 2
        // ----------
        edge1 = pos3 - pos1;
        edge2 = pos4 - pos1;
        deltaUV1 = uv3 - uv1;
        deltaUV2 = uv4 - uv1;

        f = 1.0 / (deltaUV1.x * deltaUV2.y - deltaUV2.x * deltaUV1.y);

        tangent2.x = f * (deltaUV2.y * edge1.x - deltaUV1.y * edge2.x);
        tangent2.y = f * (deltaUV2.y * edge1.y - deltaUV1.y * edge2.y);
        tangent2.z = f * (deltaUV2.y * edge1.z - deltaUV1.y * edge2.z);
        tangent2 = tangent2.normalize();

        bitangent2.x = f * (-deltaUV2.x * edge1.x + deltaUV1.x * edge2.x);
        bitangent2.y = f * (-deltaUV2.x * edge1.y + deltaUV1.x * edge2.y);
        bitangent2.z = f * (-deltaUV2.x * edge1.z + deltaUV1.x * edge2.z);
        bitangent2 = bitangent2.normalize();

        let quadVertices: [f32; 84] = [
            // positions            // normal         // texcoords  // tangent                          // bitangent
            pos1.x, pos1.y, pos1.z, nm.x, nm.y, nm.z, uv1.x, uv1.y, tangent1.x, tangent1.y, tangent1.z, bitangent1.x, bitangent1.y, bitangent1.z,
            pos2.x, pos2.y, pos2.z, nm.x, nm.y, nm.z, uv2.x, uv2.y, tangent1.x, tangent1.y, tangent1.z, bitangent1.x, bitangent1.y, bitangent1.z,
            pos3.x, pos3.y, pos3.z, nm.x, nm.y, nm.z, uv3.x, uv3.y, tangent1.x, tangent1.y, tangent1.z, bitangent1.x, bitangent1.y, bitangent1.z,

            pos1.x, pos1.y, pos1.z, nm.x, nm.y, nm.z, uv1.x, uv1.y, tangent2.x, tangent2.y, tangent2.z, bitangent2.x, bitangent2.y, bitangent2.z,
            pos3.x, pos3.y, pos3.z, nm.x, nm.y, nm.z, uv3.x, uv3.y, tangent2.x, tangent2.y, tangent2.z, bitangent2.x, bitangent2.y, bitangent2.z,
            pos4.x, pos4.y, pos4.z, nm.x, nm.y, nm.z, uv4.x, uv4.y, tangent2.x, tangent2.y, tangent2.z, bitangent2.x, bitangent2.y, bitangent2.z
        ];

        // configure plane VAO
        gl::GenVertexArrays(1, quadVAO);
        gl::GenBuffers(1, quadVBO);
        gl::BindVertexArray(*quadVAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, *quadVBO);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (quadVertices.len() * mem::size_of::<f32>()) as isize,
            &quadVertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW);
        let stride = 14 * mem::size_of::<GLfloat>() as GLsizei;
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (6 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(3);
        gl::VertexAttribPointer(3, 3, gl::FLOAT, gl::FALSE, stride, (8 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(4);
        gl::VertexAttribPointer(4, 3, gl::FLOAT, gl::FALSE, stride, (11 * mem::size_of::<GLfloat>()) as *const c_void);
    }

    gl::BindVertexArray(*quadVAO);
    gl::DrawArrays(gl::TRIANGLES, 0, 6);
    gl::BindVertexArray(0);
}

// WASM entry point
pub fn main_5_4() {
    unsafe {
        init_5_4();
    }
}