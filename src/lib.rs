#![deny(warnings)]
#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![warn(missing_docs)]

//! # Flywheel Engine
//! `Flywheel` is an open-source, cross-platform 2.5D game engine built on
//! `SDL3`. It is lightweight, dependency-free, and includes `Serde` support for
//! easy serialization. Designed for performance and flexibility, `Flywheel`
//! simplifies game development while leveraging `SDL3`'s power for smooth
//! rendering and input handling.

/// `Flywheel` [geometry] module.
#[forbid(unsafe_code)]
pub mod geometry;
