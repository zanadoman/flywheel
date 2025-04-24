#![allow(clippy::missing_errors_doc, dead_code, missing_docs)]

pub use self::{entity::Entity, manager::Manager, world::World};

mod archetype;
mod component_manager;
mod component_pool;
mod entity;
mod entity_manager;
mod manager;
mod system;
mod world;
