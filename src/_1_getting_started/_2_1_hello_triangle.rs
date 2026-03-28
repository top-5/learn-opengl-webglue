#![allow(non_upper_case_globals)]
extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;
use self::gl::types::*;

use std::sync::mpsc::Receiver;
use std::ffi::CString;
use std::ptr;
use std::str;
use std::mem;
use std::os::raw::c_void;
use std::cell::RefCell;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

const vertexShaderSource: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const fragmentShaderSource: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
       FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

// State for the triangle example
struct State_2_1 {
    shader_program: GLuint,
    vao: GLuint,
    vbo: GLuint,
}

thread_local! {
    static STATE: RefCell<Option<State_2_1>> = RefCell::new(None);
}

/// Reset state (force re-initialization)
pub unsafe fn reset_2_1() {
    STATE.with(|state| {
        if let Some(s) = state.borrow_mut().take() {
            gl::DeleteVertexArrays(1, &s.vao);
            gl::DeleteBuffers(1, &s.vbo);
            gl::DeleteProgram(s.shader_program);
        }
    });
}

unsafe fn init_2_1() {
    // build and compile our shader program
    // ------------------------------------
    // vertex shader
    let vertexShader = gl::CreateShader(gl::VERTEX_SHADER);
    let c_str_vert = CString::new(vertexShaderSource.as_bytes()).unwrap();
    gl::ShaderSource(vertexShader, 1, &c_str_vert.as_ptr(), ptr::null());
    gl::CompileShader(vertexShader);

    // check for shader compile errors
    let mut success = gl::FALSE as GLint;
    let mut infoLog = Vec::with_capacity(512);
    infoLog.set_len(512 - 1); // subtract 1 to skip the trailing null character
    gl::GetShaderiv(vertexShader, gl::COMPILE_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetShaderInfoLog(vertexShader, 512, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
        println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", String::from_utf8_lossy(&infoLog));
    }

    // fragment shader
    let fragmentShader = gl::CreateShader(gl::FRAGMENT_SHADER);
    let c_str_frag = CString::new(fragmentShaderSource.as_bytes()).unwrap();
    gl::ShaderSource(fragmentShader, 1, &c_str_frag.as_ptr(), ptr::null());
    gl::CompileShader(fragmentShader);
    // check for shader compile errors
    gl::GetShaderiv(fragmentShader, gl::COMPILE_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetShaderInfoLog(fragmentShader, 512, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
        println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", String::from_utf8_lossy(&infoLog));
    }

    // link shaders
    let shaderProgram = gl::CreateProgram();
    gl::AttachShader(shaderProgram, vertexShader);
    gl::AttachShader(shaderProgram, fragmentShader);
    gl::LinkProgram(shaderProgram);
    // check for linking errors
    gl::GetProgramiv(shaderProgram, gl::LINK_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetProgramInfoLog(shaderProgram, 512, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
        println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", String::from_utf8_lossy(&infoLog));
    }
    gl::DeleteShader(vertexShader);
    gl::DeleteShader(fragmentShader);

    // set up vertex data (and buffer(s)) and configure vertex attributes
    // ------------------------------------------------------------------
    // HINT: type annotation is crucial since default for float literals is f64
    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0, // left
         0.5, -0.5, 0.0, // right
         0.0,  0.5, 0.0  // top
    ];
    let (mut VBO, mut VAO) = (0, 0);
    gl::GenVertexArrays(1, &mut VAO);
    gl::GenBuffers(1, &mut VBO);
    // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
    gl::BindVertexArray(VAO);

    gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
    gl::BufferData(gl::ARRAY_BUFFER,
                   (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                   &vertices[0] as *const f32 as *const c_void,
                   gl::STATIC_DRAW);

    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
    gl::EnableVertexAttribArray(0);

    // note that this is allowed, the call to gl::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);

    // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
    // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
    gl::BindVertexArray(0);

    // uncomment this call to draw in wireframe polygons.
    // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

    STATE.with(|state| {
        *state.borrow_mut() = Some(State_2_1 {
            shader_program: shaderProgram,
            vao: VAO,
            vbo: VBO,
        });
    });
}

unsafe fn render_2_1() {
    STATE.with(|state| {
        if let Some(ref s) = *state.borrow() {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // draw our first triangle
            gl::UseProgram(s.shader_program);
            gl::BindVertexArray(s.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    });
}

#[allow(non_snake_case)]
pub fn main_1_2_1() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe { init_2_1(); }
        }
    });
    unsafe { render_2_1(); }
}

// NOTE: not the same version as in common.rs!
fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}
