#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use std::ffi::CStr;
use std::cell::RefCell;

extern crate glfw;

extern crate gl;

use cgmath::{Matrix4, Deg, vec3, perspective};

use shader::Shader;
use camera::Camera;
use model::Model;

// settings
const SCR_WIDTH: u32 = 1280;
const SCR_HEIGHT: u32 = 720;

struct State_4_9_3 {
    shader: Shader,
    normalShader: Shader,
    nanoSuit: Model,
}

thread_local! {
    static STATE: RefCell<Option<State_4_9_3>> = RefCell::new(None);
}

unsafe fn reset_4_9_3() {
    STATE.with(|state| {
        if let Some(_s) = state.borrow_mut().take() {
            // Model cleanup handled by Drop
        }
    });
}

unsafe fn init_4_9_3() {
    // configure global opengl state
    gl::Enable(gl::DEPTH_TEST);

    // build and compile shaders
    let shader = Shader::new(
        "src/_4_advanced_opengl/shaders/9.3.default.vs",
        "src/_4_advanced_opengl/shaders/9.3.default.fs",
    );
    let normalShader = Shader::with_geometry_shader(
        "src/_4_advanced_opengl/shaders/9.3.normal_visualization.vs",
        "src/_4_advanced_opengl/shaders/9.3.normal_visualization.fs",
        "src/_4_advanced_opengl/shaders/9.3.normal_visualization.gs"
    );

    // load models
    let nanoSuit = Model::new("resources/objects/nanosuit/nanosuit.obj");

    STATE.with(|state| {
        *state.borrow_mut() = Some(State_4_9_3 {
            shader,
            normalShader,
            nanoSuit,
        });
    });
}

unsafe fn render_4_9_3(camera: &Camera) {
    gl::ClearColor(0.1, 0.1, 0.1, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            // configure transformation matrices
            let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            let view = camera.GetViewMatrix();
            let mut model = Matrix4::<f32>::from_translation(vec3(0.0, -1.75, 0.0));
            model = model * Matrix4::from_scale(0.2);
            
            s.shader.useProgram();
            s.shader.setMat4(c_str!("projection"), &projection);
            s.shader.setMat4(c_str!("view"), &view);
            s.shader.setMat4(c_str!("model"), &model);

            // draw model as usual
            s.nanoSuit.Draw(&s.shader);

            // then draw model with normal visualizing geometry shader
            s.normalShader.useProgram();
            s.normalShader.setMat4(c_str!("projection"), &projection);
            s.normalShader.setMat4(c_str!("view"), &view);
            s.normalShader.setMat4(c_str!("model"), &model);

            s.nanoSuit.Draw(&s.normalShader);
        }
    });
}

pub fn main_4_9_3() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe { init_4_9_3(); }
        }
    });
    unsafe { render_4_9_3(&Camera::default()); }
}
