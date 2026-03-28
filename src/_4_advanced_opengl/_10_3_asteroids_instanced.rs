#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use std::cell::RefCell;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::ffi::CStr;

extern crate rand;
use self::rand::Rng;

use shader::Shader;
use camera::Camera;
use model::Model;

use cgmath::{Matrix4, vec3, Vector4, Deg, perspective};
use cgmath::prelude::*;

const SCR_WIDTH: u32 = 1280;
const SCR_HEIGHT: u32 = 720;

struct State_4_10_3 {
    asteroidShader: Shader,
    planetShader: Shader,
    rock: Model,
    planet: Model,
    amount: usize,
    buffer: u32,
}

thread_local! {
    static STATE: RefCell<Option<State_4_10_3>> = RefCell::new(None);
}

unsafe fn reset_4_10_3() {
    STATE.with(|state| {
        if let Some(s) = state.borrow_mut().take() {
            gl::DeleteBuffers(1, &s.buffer);
        }
    });
}

unsafe fn init_4_10_3() {
    gl::Enable(gl::DEPTH_TEST);

    let asteroidShader = Shader::new(
        "src/_4_advanced_opengl/shaders/10.3.asteroids.vs",
        "src/_4_advanced_opengl/shaders/10.3.asteroids.fs");
    let planetShader = Shader::new(
        "src/_4_advanced_opengl/shaders/10.3.planet.vs",
        "src/_4_advanced_opengl/shaders/10.3.planet.fs");

    let rock = Model::new("resources/objects/rock/rock.obj");
    let planet = Model::new("resources/objects/planet/planet.obj");

    // generate a large list of semi-random model transformation matrices
    let amount = 100_000;
    let mut modelMatrices: Vec<Matrix4<f32>> = Vec::with_capacity(amount);
    let mut rng = rand::thread_rng();
    let radius = 150.0;
    let offset: f32 = 25.0;
    for i in 0..amount {
        let angle = (i as f32 / amount as f32) * 360.0;
        let disp_range = (2.0 * offset * 100.0) as u32;
        let mut displacement = (rng.gen::<u32>() % disp_range) as f32 / 100.0 - offset;
        let x = angle.sin() * radius + displacement;
        displacement = (rng.gen::<u32>() % disp_range) as f32 / 100.0 - offset;
        let y = displacement * 0.4;
        displacement = (rng.gen::<u32>() % disp_range) as f32 / 100.0 - offset;
        let z = angle.cos() * radius + displacement;
        let mut model = Matrix4::<f32>::from_translation(vec3(x, y, z));

        let scale = (rng.gen::<u32>() % 20) as f32 / 100.0 + 0.05;
        model = model * Matrix4::from_scale(scale);

        let rotAngle = (rng.gen::<u32>() % 360) as f32;
        model = model * Matrix4::from_axis_angle(vec3(0.4, 0.6, 0.8).normalize(), Deg(rotAngle));

        modelMatrices.push(model);
    }

    // configure instanced array
    let mut buffer = 0;
    gl::GenBuffers(1, &mut buffer);
    gl::BindBuffer(gl::ARRAY_BUFFER, buffer);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (amount * mem::size_of::<Matrix4<f32>>()) as isize,
        &modelMatrices[0] as *const Matrix4<f32> as *const c_void,
        gl::STATIC_DRAW);

    // set transformation matrices as an instance vertex attribute (with divisor 1)
    let size_mat4 = mem::size_of::<Matrix4<f32>>() as i32;
    let size_vec4 = mem::size_of::<Vector4<f32>>() as i32;
    for mesh in &rock.meshes {
        let VAO = mesh.VAO;
        gl::BindVertexArray(VAO);
        // set attribute pointers for matrix (4 times vec4)
        gl::EnableVertexAttribArray(3);
        gl::VertexAttribPointer(3, 4, gl::FLOAT, gl::FALSE, size_mat4, ptr::null());
        gl::EnableVertexAttribArray(4);
        gl::VertexAttribPointer(4, 4, gl::FLOAT, gl::FALSE, size_mat4, size_vec4 as *const c_void);
        gl::EnableVertexAttribArray(5);
        gl::VertexAttribPointer(5, 4, gl::FLOAT, gl::FALSE, size_mat4, (2 * size_vec4) as *const c_void);
        gl::EnableVertexAttribArray(6);
        gl::VertexAttribPointer(6, 4, gl::FLOAT, gl::FALSE, size_mat4, (3 * size_vec4) as *const c_void);

        gl::VertexAttribDivisor(3, 1);
        gl::VertexAttribDivisor(4, 1);
        gl::VertexAttribDivisor(5, 1);
        gl::VertexAttribDivisor(6, 1);

        gl::BindVertexArray(0);
    }

    STATE.with(|state| {
        *state.borrow_mut() = Some(State_4_10_3 {
            asteroidShader,
            planetShader,
            rock,
            planet,
            amount,
            buffer,
        });
    });
}

unsafe fn render_4_10_3(camera: &Camera) {
    gl::ClearColor(0.1, 0.1, 0.1, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            let projection: Matrix4<f32> = perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 1000.0);
            let view = camera.GetViewMatrix();

            s.asteroidShader.useProgram();
            s.asteroidShader.setMat4(c_str!("projection"), &projection);
            s.asteroidShader.setMat4(c_str!("view"), &view);
            s.planetShader.useProgram();
            s.planetShader.setMat4(c_str!("projection"), &projection);
            s.planetShader.setMat4(c_str!("view"), &view);

            // draw planet
            let mut model = Matrix4::<f32>::from_translation(vec3(0.0, -3.0, 0.0));
            model = model * Matrix4::from_scale(4.0);
            s.planetShader.setMat4(c_str!("model"), &model);
            s.planet.Draw(&s.planetShader);

            // draw meteorites
            s.asteroidShader.useProgram();
            s.asteroidShader.setInt(c_str!("texture_diffuse1"), 0);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, s.rock.textures_loaded[0].id);

            for mesh in &s.rock.meshes {
                gl::BindVertexArray(mesh.VAO);
                gl::DrawElementsInstanced(gl::TRIANGLES, mesh.indices.len() as i32, gl::UNSIGNED_INT, ptr::null(), s.amount as i32);
                gl::BindVertexArray(0);
            }
        }
    });
}

pub fn main_4_10_3() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe { init_4_10_3(); }
        }
    });
    unsafe {
        render_4_10_3(&Camera::default());
    }
}
