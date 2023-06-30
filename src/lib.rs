#![allow(clippy::single_component_path_imports)]

//! [Download](https://lxchn1v1.github.io/DreamEngine/download.html)
//!
//! Dream Engine is a Bevy engine based game engine, made with love by AsyncLoaders and BadDreams Studio
//!
//! ## Example
//!
//! Here is a simple "Hello World" Dream Engine app:
//! ```
//! use dreamengine::prelude::*;
//!
//! fn main() {
//!    App::new()
//!        .add_systems(Update, hello_world_system)
//!        .run();
//! }
//!
//! fn hello_world_system() {
//!    println!("hello world");
//! }
//! ```
//!
//! Don't let the simplicity of the example above fool you. Bevy is a [fully featured game engine](https://bevyengine.org)
//! and it gets more powerful every day!

#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/lxchn1v1/lxchn1v1.github.io/main/public/images/profile.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/lxchn1v1/lxchn1v1.github.io/main/public/images/profile.jpg"
)]

pub use dreamengine_internal::*;

#[cfg(feature = "dynamic_linking")]
#[allow(unused_imports)]
use bevy_dylib;
