extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;

use std::cell::RefCell;
use std::sync::mpsc::Receiver;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

struct State_1_2 {
    initialized: bool,
}

thread_local! {
    static STATE: RefCell<Option<State_1_2>> = RefCell::new(None);
}

unsafe fn reset_1_2() {
    STATE.with(|state| {
        state.borrow_mut().take();
    });
}

unsafe fn init_1_2() {
    STATE.with(|state| {
        *state.borrow_mut() = Some(State_1_2 {
            initialized: true,
        });
    });
}

unsafe fn render_1_2() {
    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);
}

pub fn main_1_1_2() {
    STATE.with(|state| {
        if state.borrow().is_none() {
            unsafe { init_1_2(); }
        }
    });
    unsafe { render_1_2(); }
}
