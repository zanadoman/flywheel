#![deny(warnings)]
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic, missing_docs)]

//! # Flywheel Engine
//! `Flywheel` is an open-source, cross-platform 2.5D game engine built on
//! `SDL3`. It is lightweight, dependency-free, and includes `Serde` support for
//! easy serialization. Designed for performance and flexibility, `Flywheel`
//! simplifies game development while leveraging `SDL3`'s power for smooth
//! rendering and input handling.

pub use self::geometry::{
    Angle, Matrix, Position, Rectangle, Scale, Vector, into_degs, into_rads,
};

/// `Flywheel` [geometry] module.
#[forbid(unsafe_code)]
pub mod geometry;
