use crate::{component_manager, ecs::World, types::Entity, ComponentManager, ComponentRegistry};

/// Provides a friendlier api for creating an entity and hooking up its components
pub struct EntityBuilder<'a>
where
// T: ComponentRegistry<'static> + Sync + 'static,
{
    ecs: &'a mut World,
    entity: Option<Entity>,
}

impl<'a> EntityBuilder<'a>
where
// T: ComponentRegistry<'static> + Sync + 'static,
{
    /// Create a new instance of `EntityBuilder`
    pub fn new(ecs: &'a mut World) -> Self {
        let entity = Some(ecs.entity_manager.create());
        Self { ecs, entity }
    }

    /// Add a component to the Entity. Failing to call `done()` before the builder is dropped
    /// should cause the Entity to be destroyed
    /// # Panic
    /// Panics if the component type has not been registered
    pub fn with<U>(self, value: U) -> Self
    where
        U: std::fmt::Debug + Sync + Send + 'static,
    {
        let mut registry = self.ecs.component_registry.lock().unwrap();
        registry
            .get_mut::<component_manager::Manager<U>>()
            .unwrap_or_else(|| panic!("Component type not registered for {:?}", value))
            .create(self.entity.unwrap(), value);

        self
    }

    /// Consume builder and return Entity
    pub fn done(mut self) -> Entity {
        let e = self.entity.take().unwrap();
        self.ecs.entity_list.push(e);
        e
    }
}

impl<'a> Drop for EntityBuilder<'a>
where
// T: ComponentRegistry<'static> + Sync + 'static,
{
    fn drop(&mut self) {
        if let Some(e) = self.entity {
            self.ecs.entity_manager.destroy(&e);
        }
    }
}
