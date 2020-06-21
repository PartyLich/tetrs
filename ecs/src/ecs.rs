use std::sync;

use entity_man::{entity_manager::EntityManagerU32, EntityManager};

use crate::{
    component_manager::{self, ComponentManager},
    component_registry::{ComponentRegistry, Registry},
    error,
    types::Entity,
};

mod entity_builder;
pub use entity_builder::*;

pub type Result<T> = std::result::Result<T, error::Error>;

pub struct World {
    pub component_registry: &'static sync::Mutex<Registry>,
    pub entity_manager: Box<dyn EntityManager>,

    pub entity_list: Vec<Entity>,
}

impl World {
    /// Create a new instance
    pub fn new() -> Self {
        lazy_static! {
            pub static ref COMPONENT_REGISTRY: sync::Mutex<Registry> =
                sync::Mutex::new(Registry::new());
        }

        Self {
            entity_manager: Box::new(EntityManagerU32::new()),
            component_registry: &COMPONENT_REGISTRY,
            entity_list: Vec::new(),
        }
    }

    // remove dead entities from entity_list
    pub fn prune_dead(&mut self) {
        let mut corpses = Vec::new();
        // identify the dead
        for (i, e) in self.entity_list.iter().enumerate() {
            if !self.entity_manager.alive(&e) {
                corpses.push(i);
            }
        }

        // remove the dead
        for (i, corpse) in (0..).zip(corpses) {
            self.entity_list.remove(corpse - i);
        }
    }

    // pub fn read_only<T>(&mut self) -> &T
    // where
    //     T: Sync + Send + 'static,
    // {
    //     let registry = self.component_registry.lock().unwrap();
    //     // self.component_registry
    //     // .lock()
    //     // .unwrap()
    //     registry
    //         .get::<T>()
    //         // .ok_or(tetrs::Error::MissingComponent("Mesh"))
    //         .unwrap()
    // }

    /// Build an entity via `EntityBuilder`
    pub fn build_entity(&mut self) -> EntityBuilder {
        EntityBuilder::new(self)
    }

    /// Create an Entity with no components
    pub fn create_entity(&mut self) -> Entity {
        let e = self.entity_manager.create();
        self.entity_list.push(e);
        e
    }

    /// Destroy an Entity
    pub fn destroy_entity(&mut self, e: &Entity) {
        if self.entity_manager.alive(e) {
            self.entity_manager.destroy(e);
            // self.entity_list.retain(|ent| ent != e);
        }
    }

    /// Add a component to an Entity, initialized to the component's default value
    /// # Panic
    /// Panics if the component type has not been registered
    pub fn add_component_default<T>(&mut self, e: Entity) -> Result<Entity>
    where
        T: Default + Sync + Send + 'static,
    {
        add_component_default::<T>(self, e)
    }

    /// Add a component to an Entity, providing the initial value
    /// # Panic
    /// Panics if the component type has not been registered
    pub fn add_component<T>(&mut self, e: Entity, value: T) -> Result<Entity>
    where
        T: Sync + Send + 'static,
    {
        add_component::<T>(self, e, value)
    }

    /// Remove a component from the provided Entity
    /// # Panic
    /// Panics if the component type has not been registered
    pub fn remove_component<U>(&mut self, e: Entity) -> Result<Entity>
    where
        U: std::fmt::Debug + Sync + Send + 'static,
    {
        remove_component::<U>(self, e)
    }

    /// Set the value for component `T` and entity e
    pub fn set_component<T>(&mut self, e: Entity, value: T)
    where
        T: Sync + Send + 'static,
    {
        set_component::<T>(self, e, value)
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

/// Add a component to an Entity, initialized to the component's default value
/// # Panic
/// Panics if the component type has not been registered
fn add_component_default<T>(ecs: &mut World, e: Entity) -> Result<Entity>
where
    T: Default + Sync + Send + 'static,
{
    let mut registry = ecs.component_registry.lock().unwrap();
    registry
        .get_mut::<component_manager::Manager<T>>()
        .unwrap_or_else(|| panic!("Component type not registered for {:?}", ()))
        .create(e, T::default());

    Ok(e)
}

/// Add a component to an Entity, providing the initial value
/// # Panic
/// Panics if the component type has not been registered
fn add_component<T>(ecs: &mut World, e: Entity, value: T) -> Result<Entity>
where
    T: Sync + Send + 'static,
{
    let mut registry = ecs.component_registry.lock().unwrap();
    registry
        .get_mut::<component_manager::Manager<T>>()
        .unwrap_or_else(|| panic!("Component type not registered for {:?}", ()))
        .create(e, value);

    Ok(e)
}

/// Remove a component from the provided Entity
/// # Panic
/// Panics if the component type has not been registered
fn remove_component<U>(ecs: &mut World, e: Entity) -> Result<Entity>
where
    U: std::fmt::Debug + Sync + Send + 'static,
{
    let mut registry = ecs.component_registry.lock().unwrap();
    let comp_man = registry
        .get_mut::<component_manager::Manager<U>>()
        .unwrap_or_else(|| panic!("Component type not registered for {:?}", ()));
    let i = comp_man.lookup(&e);
    if i.is_none() {
        return Ok(e);
    }
    comp_man.destroy(i.unwrap());

    Ok(e)
}

/// Set the value for component `T` and entity e
fn set_component<T>(ecs: &mut World, e: Entity, value: T)
where
    T: Sync + Send + 'static,
{
    let mut registry = ecs.component_registry.lock().unwrap();
    registry.get_component_mut::<T>(&e).map(|m| *m = value);
}
