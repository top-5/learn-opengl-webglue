#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use std::cell::RefCell;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::ffi::CStr;
use camera::Camera;
use cgmath::{Matrix4, vec3, Vector3, Deg, perspective, EuclideanSpace};
use shader::Shader;
use image::GenericImage;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

struct State_5_2 {
    shader: Shader,
    planeVAO: u32,
    planeVBO: u32,
    floorTexture: u32,
    floorTextureGammaCorrected: u32,
    gammaEnabled: bool,
}

thread_local! {
    static STATE: RefCell<Option<State_5_2>> = RefCell::new(None);
}

unsafe fn reset_5_2() {
    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            gl::DeleteVertexArrays(1, &s.planeVAO);
            gl::DeleteBuffers(1, &s.planeVBO);
            gl::DeleteTextures(1, &s.floorTexture);
            gl::DeleteTextures(1, &s.floorTextureGammaCorrected);
        }
        *state.borrow_mut() = None;
    });
}

unsafe fn init_5_2() {
    gl::Enable(gl::DEPTH_TEST);
    gl::Enable(gl::BLEND);
    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

    let shader = Shader::new(
        "src/_5_advanced_lighting/shaders/2.gamma_correction.vs",
        "src/_5_advanced_lighting/shaders/2.gamma_correction.fs");

    let planeVertices: [f32; 48] = [
         10.0, -0.5,  10.0,  0.0, 1.0, 0.0,  10.0,  0.0,
        -10.0, -0.5,  10.0,  0.0, 1.0, 0.0,   0.0,  0.0,
        -10.0, -0.5, -10.0,  0.0, 1.0, 0.0,   0.0, 10.0,
         10.0, -0.5,  10.0,  0.0, 1.0, 0.0,  10.0,  0.0,
        -10.0, -0.5, -10.0,  0.0, 1.0, 0.0,   0.0, 10.0,
         10.0, -0.5, -10.0,  0.0, 1.0, 0.0,  10.0, 10.0
    ];

    let (mut planeVAO, mut planeVBO) = (0, 0);
    gl::GenVertexArrays(1, &mut planeVAO);
    gl::GenBuffers(1, &mut planeVBO);
    gl::BindVertexArray(planeVAO);
    gl::BindBuffer(gl::ARRAY_BUFFER, planeVBO);
    gl::BufferData(gl::ARRAY_BUFFER,
                   (planeVertices.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                   &planeVertices[0] as *const f32 as *const c_void,
                   gl::STATIC_DRAW);
    gl::EnableVertexAttribArray(0);
    let stride = 8 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei;
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    gl::EnableVertexAttribArray(1);
    gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<gl::types::GLfloat>()) as *const c_void);
    gl::EnableVertexAttribArray(2);
    gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (6 * mem::size_of::<gl::types::GLfloat>()) as *const c_void);
    gl::BindVertexArray(0);

    let floorTexture = loadTexture("resources/textures/wood.png", false);
    let floorTextureGammaCorrected = loadTexture("resources/textures/wood.png", true);

    shader.useProgram();
    shader.setInt(c_str!("floorTexture"), 0);

    STATE.with(|state| {
        *state.borrow_mut() = Some(State_5_2 {
            shader,
            planeVAO,
            planeVBO,
            floorTexture,
            floorTextureGammaCorrected,
            gammaEnabled: false,
        });
    });
}

unsafe fn render_5_2(camera: &Camera) {
    gl::ClearColor(0.1, 0.1, 0.1, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    let lightPositions: [Vector3<f32>; 4] = [
        vec3(-3.0, 0.0, 0.0),
        vec3(-1.0, 0.0, 0.0),
        vec3( 1.0, 0.0, 0.0),
        vec3( 3.0, 0.0, 0.0)
    ];
    let lightColors: [Vector3<f32>; 4] = [
        vec3(0.25, 0.25, 0.25),
        vec3(0.50, 0.50, 0.50),
        vec3(0.75, 0.75, 0.75),
        vec3(1.00, 1.00, 1.00)
    ];

    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            s.shader.useProgram();
            let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            let view = camera.GetViewMatrix();
            s.shader.setMat4(c_str!("projection"), &projection);
            s.shader.setMat4(c_str!("view"), &view);
            s.shader.setVector3(c_str!("viewPos"), &camera.Position.to_vec());

            s.shader.setVector3(c_str!("lights[0].Position"), &lightPositions[0]);
            s.shader.setVector3(c_str!("lights[0].Color"), &lightColors[0]);
            s.shader.setVector3(c_str!("lights[1].Position"), &lightPositions[1]);
            s.shader.setVector3(c_str!("lights[1].Color"), &lightColors[1]);
            s.shader.setVector3(c_str!("lights[2].Position"), &lightPositions[2]);
            s.shader.setVector3(c_str!("lights[2].Color"), &lightColors[2]);
            s.shader.setVector3(c_str!("lights[3].Position"), &lightPositions[3]);
            s.shader.setVector3(c_str!("lights[3].Color"), &lightColors[3]);
            s.shader.setInt(c_str!("gamma"), s.gammaEnabled as i32);

            gl::BindVertexArray(s.planeVAO);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, if s.gammaEnabled { s.floorTextureGammaCorrected } else { s.floorTexture });
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    });
}

pub fn main_5_2() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe { init_5_2(); }
        }
    });
    unsafe {
        render_5_2(&Camera::default());
    }
}

unsafe fn loadTexture(path: &str, gammaCorrection: bool) -> u32 {
    use std::path::Path;

    let mut textureID = 0;
    gl::GenTextures(1, &mut textureID);

    #[cfg(target_arch = "wasm32")]
    let img = {
        let bytes = gl::resources::load_bytes_sync(path).expect("Failed to load texture");
        image::load_from_memory(&bytes).expect("Texture failed to load")
    };
    
    #[cfg(not(target_arch = "wasm32"))]
    let img = image::open(&Path::new(path)).expect("Texture failed to load");

    let (width, height) = img.dimensions();

    let color_type = img.color();
    let (internal_format, data_format) = match color_type {
        image::ColorType::Gray(_) => (gl::RED, gl::RED),
        image::ColorType::RGB(_) => {
            if gammaCorrection {
                (gl::SRGB, gl::RGB)
            } else {
                (gl::RGB, gl::RGB)
            }
        },
        image::ColorType::RGBA(_) => {
            if gammaCorrection {
                (gl::SRGB_ALPHA, gl::RGBA)
            } else {
                (gl::RGBA, gl::RGBA)
            }
        },
        _ => panic!("Unsupported color type"),
    };

    let data = img.to_rgb().into_raw();

    gl::BindTexture(gl::TEXTURE_2D, textureID);
    gl::TexImage2D(gl::TEXTURE_2D, 0, internal_format as i32, width as i32, height as i32,
                   0, data_format, gl::UNSIGNED_BYTE, &data[0] as *const u8 as *const c_void);
    gl::GenerateMipmap(gl::TEXTURE_2D);

    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    textureID
}
