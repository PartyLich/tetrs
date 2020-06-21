#[macro_use]
extern crate lazy_static;

pub mod component_manager;
pub mod component_registry;
mod ecs;
pub mod error;
pub mod types;

pub use component_manager::ComponentManager;
pub use component_registry::ComponentRegistry;
pub use entity_man::EntityManager;

pub use crate::ecs::*;
