#![cfg_attr(feature = "cargo-clippy", allow(cast_lossless))]

extern crate gl;
extern crate image;
extern crate cgmath;
extern crate tobj;

pub mod common;
pub mod shader;
pub mod macros;
pub mod camera;
pub mod mesh;
pub mod model;
pub mod utils;

#[cfg(feature = "chapter-1")]
pub mod _1_getting_started;
#[cfg(feature = "chapter-2")]
pub mod _2_lighting;
#[cfg(feature = "chapter-3")]
pub mod _3_model_loading;
#[cfg(feature = "chapter-4")]
pub mod _4_advanced_opengl;
#[cfg(feature = "chapter-5")]
pub mod _5_advanced_lighting;
#[cfg(feature = "chapter-6")]
pub mod _6_pbr;
#[cfg(feature = "chapter-7")]
pub mod _7_in_practice;
