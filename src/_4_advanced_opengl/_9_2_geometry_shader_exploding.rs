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

struct State_4_9_2 {
    shader: Shader,
    nanoSuit: Model,
    time: f32,
}

thread_local! {
    static STATE: RefCell<Option<State_4_9_2>> = RefCell::new(None);
}

unsafe fn reset_4_9_2() {
    STATE.with(|state| {
        if let Some(_s) = state.borrow_mut().take() {
            // Model cleanup handled by Drop
        }
    });
}

unsafe fn init_4_9_2() {
    // configure global opengl state
    gl::Enable(gl::DEPTH_TEST);

    // build and compile shaders
    let shader = Shader::with_geometry_shader(
        "src/_4_advanced_opengl/shaders/9.2.geometry_shader.vs",
        "src/_4_advanced_opengl/shaders/9.2.geometry_shader.fs",
        "src/_4_advanced_opengl/shaders/9.2.geometry_shader.gs"
    );

    // load models
    let nanoSuit = Model::new("resources/objects/nanosuit/nanosuit.obj");

    STATE.with(|state| {
        *state.borrow_mut() = Some(State_4_9_2 {
            shader,
            nanoSuit,
            time: 0.0,
        });
    });
}

unsafe fn render_4_9_2(camera: &Camera) {
    gl::ClearColor(0.1, 0.1, 0.1, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    STATE.with(|state| {
        if let Some(ref mut s) = *state.borrow_mut() {
            // Update time
            s.time += 0.016; // ~60fps

            // configure transformation matrices
            let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            let view = camera.GetViewMatrix();
            let mut model = Matrix4::<f32>::from_translation(vec3(0.0, -1.75, 0.0));
            model = model * Matrix4::from_scale(0.2);
            
            s.shader.useProgram();
            s.shader.setMat4(c_str!("projection"), &projection);
            s.shader.setMat4(c_str!("view"), &view);
            s.shader.setMat4(c_str!("model"), &model);

            // add time component to geometry shader
            s.shader.setFloat(c_str!("time"), s.time);

            // draw model
            s.nanoSuit.Draw(&s.shader);
        }
    });
}

pub fn main_4_9_2() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe { init_4_9_2(); }
        }
    });
    unsafe { render_4_9_2(&Camera::default()); }
}
