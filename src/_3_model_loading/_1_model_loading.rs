#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
extern crate gl;

use std::cell::RefCell;
use std::ffi::CStr;

use shader::Shader;
use camera::Camera;
use model::Model;

use cgmath::{Matrix4, vec3, Deg, perspective};

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

struct State_3_1 {
    shader: Shader,
    model: Model,
}

thread_local! {
    static STATE: RefCell<Option<State_3_1>> = RefCell::new(None);
}

unsafe fn reset_3_1() {
    STATE.with(|state| {
        *state.borrow_mut() = None;
    });
}

unsafe fn init_3_1() {
    gl::Enable(gl::DEPTH_TEST);

    let shader = Shader::new(
        "src/_3_model_loading/shaders/1.model_loading.vs",
        "src/_3_model_loading/shaders/1.model_loading.fs");

    let model = Model::new("resources/objects/nanosuit/nanosuit.obj");

    STATE.with(|state| {
        *state.borrow_mut() = Some(State_3_1 {
            shader,
            model,
        });
    });
}

unsafe fn render_3_1(camera: &Camera) {
    gl::ClearColor(0.1, 0.1, 0.1, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            s.shader.useProgram();

            let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            let view = camera.GetViewMatrix();
            s.shader.setMat4(c_str!("projection"), &projection);
            s.shader.setMat4(c_str!("view"), &view);

            let mut model = Matrix4::<f32>::from_translation(vec3(0.0, -0.4, 0.0));
            model = model * Matrix4::from_scale(0.18);
            s.shader.setMat4(c_str!("model"), &model);
            s.model.Draw(&s.shader);
        }
    });
}

pub fn main_3_1() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe { init_3_1(); }
        }
    });
    
    // Create a camera positioned to view the full model including head
    let camera = Camera {
        Position: cgmath::Point3::new(0.0, 0.5, 4.0),  // Further back and centered
        ..Camera::default()
    };
    
    unsafe { render_3_1(&camera); }
}
