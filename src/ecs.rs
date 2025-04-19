pub use self::{
    archetype::Archetype, component_manager::ComponentManager, entity::Entity,
};

mod archetype;
mod component_manager;
mod component_pool;
mod entity;
mod entity_manager;
