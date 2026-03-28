#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use std::cell::RefCell;
use std::ffi::CStr;

extern crate rand;
use self::rand::Rng;

use shader::Shader;
use camera::Camera;
use model::Model;

use cgmath::{Matrix4, vec3, Deg, perspective};
use cgmath::prelude::*;

const SCR_WIDTH: u32 = 1280;
const SCR_HEIGHT: u32 = 720;

struct State_4_10_2 {
    shader: Shader,
    rock: Model,
    planet: Model,
    modelMatrices: Vec<Matrix4<f32>>,
}

thread_local! {
    static STATE: RefCell<Option<State_4_10_2>> = RefCell::new(None);
}

unsafe fn reset_4_10_2() {
    STATE.with(|state| {
        state.borrow_mut().take();
    });
}

unsafe fn init_4_10_2() {
    gl::Enable(gl::DEPTH_TEST);

    let shader = Shader::new(
        "src/_4_advanced_opengl/shaders/10.2.instancing.vs",
        "src/_4_advanced_opengl/shaders/10.2.instancing.fs");

    let rock = Model::new("resources/objects/rock/rock.obj");
    let planet = Model::new("resources/objects/planet/planet.obj");

    // generate random asteroid transformations
        let amount = 1000;
        let mut modelMatrices: Vec<Matrix4<f32>> = Vec::with_capacity(amount);
        let mut rng = rand::thread_rng();
        let radius = 50.0;
        let offset: f32 = 2.5;
        for i in 0..amount {
            let angle = (i as f32 / amount as f32) * 360.0;
            let disp_range = (2.0 * offset * 100.0) as i32;
            let mut displacement = (rng.gen::<i32>().rem_euclid(disp_range)) as f32 / 100.0 - offset;
            let x = angle.sin() * radius + displacement;
            displacement = (rng.gen::<i32>().rem_euclid(disp_range)) as f32 / 100.0 - offset;
            let y = displacement * 0.4; // keep height of asteroid field smaller compared to width of x and z
            displacement = (rng.gen::<i32>().rem_euclid(disp_range)) as f32 / 100.0 - offset;
            let z = angle.cos() * radius + displacement;
            let mut model = Matrix4::<f32>::from_translation(vec3(x, y, z));

            // 2. scale: Scale between 0.05 and 0.25
            let scale = (rng.gen::<i32>().rem_euclid(20)) as f32 / 100.0 + 0.05;
            model = model * Matrix4::from_scale(scale);

            // 3. rotation: add random rotation around a (semi)randomly picked rotation axis vector
            let rotAngle = (rng.gen::<i32>().rem_euclid(360)) as f32;
            model = model * Matrix4::from_axis_angle(vec3(0.4, 0.6, 0.8).normalize(), Deg(rotAngle));

            // 4. now add to list of matrices
            modelMatrices.push(model);
        }

        STATE.with(|state| {
            *state.borrow_mut() = Some(State_4_10_2 {
                shader,
                rock,
                planet,
                modelMatrices,
            });
        });
}

unsafe fn render_4_10_2(camera: &Camera) {
    gl::ClearColor(0.1, 0.1, 0.1, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 1000.0);
            let view = camera.GetViewMatrix();
            
            s.shader.useProgram();
            s.shader.setMat4(c_str!("projection"), &projection);
            s.shader.setMat4(c_str!("view"), &view);

            // draw planet
            let mut model = Matrix4::<f32>::from_translation(vec3(0.0, -3.0, 0.0));
            model = model * Matrix4::from_scale(4.0);
            s.shader.setMat4(c_str!("model"), &model);
            s.planet.Draw(&s.shader);

            // draw asteroids
            for model_matrix in &s.modelMatrices {
                s.shader.setMat4(c_str!("model"), model_matrix);
                s.rock.Draw(&s.shader);
            }
        }
    });
}

pub fn main_4_10_2() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe {
                init_4_10_2();
            }
        }
    });

    unsafe {
        render_4_10_2(&Camera::default());
    }
}
