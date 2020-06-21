//! component registry
//!
use crate::component_manager::{ComponentManager, Manager};
use entity_man::Entity;

pub trait ComponentRegistry<'a> {
    /// Registers a component type
    fn register<T>(&mut self)
    // fn register<T, U>(&mut self)
    where
        T: Default + Sync + Send + 'static;
    // T: ComponentManager<U> + Default + Sync + Send + 'static;

    fn register2<T>(&mut self)
    where
        T: Sync + Send + 'static;

    /// get a reference to the ComponentManager for type T
    fn get<T>(&self) -> Option<&T>
    where
        T: Sync + Send + 'static;

    /// get a mutable reference to the ComponentManager for type T
    fn get_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Sync + Send + 'static;

    /// get a reference to the component of type U for this Entity
    fn get_component<T, U>(&self, e: &Entity) -> Option<&U>
    where
        T: ComponentManager<U> + Sync + Send + 'static;

    /// get a reference to the component of type U for this Entity
    fn get_component2<T>(&self, e: &Entity) -> Option<&T>
    where
        // T: Default + Sync + Send + 'static;
        T: Sync + Send + 'static;

    /// get a mutable reference to the component of type U for this Entity
    // fn get_component_mut<T, U>(&mut self, e: &Entity) -> Option<&mut U>
    // where
    //     T: ComponentManager<U> + Sync + Send + 'static;

    /// get a mutable reference to the component of type U for this Entity
    fn get_component_mut<T>(&mut self, e: &Entity) -> Option<&mut T>
    where
        // T: Default + Sync + Send + 'static;
        T: Sync + Send + 'static;
}

type Repo = anymap::Map<dyn anymap::any::Any + Sync + Send>;

pub struct Registry {
    repo: Repo,
}

impl<'a> Registry {
    pub fn new() -> Self {
        Self { repo: Repo::new() }
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ComponentRegistry<'a> for Registry {
    fn register<T>(&mut self)
    where
        T: Default + Sync + Send + 'static,
    {
        let component = T::default();
        self.repo.insert(component);
    }

    fn register2<T>(&mut self)
    where
        T: Sync + Send + 'static,
    {
        let component = Manager::<T>::default();
        self.repo.insert(component);
    }

    fn get<T>(&self) -> Option<&T>
    where
        T: Sync + Send + 'static,
    {
        self.repo.get::<T>()
    }

    fn get_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Sync + Send + 'static,
    {
        self.repo.get_mut::<T>()
    }

    fn get_component<T, U>(&self, e: &Entity) -> Option<&U>
    where
        T: ComponentManager<U> + Sync + Send + 'static,
    {
        self.repo.get::<T>()?.get_e(e)
    }

    fn get_component2<T>(&self, e: &Entity) -> Option<&T>
    where
        // T: Default + Sync + Send + 'static,
        T: Sync + Send + 'static,
    {
        self.repo.get::<Manager<T>>()?.get_e(e)
    }

    // fn get_component_mut<T, U>(&mut self, e: &Entity) -> Option<&mut U>
    // where
    //     T: ComponentManager<U> + Sync + Send + 'static,
    // {
    //     self.repo.get_mut::<T>()?.get_e_mut(e)
    // }

    fn get_component_mut<T>(&mut self, e: &Entity) -> Option<&mut T>
    where
        T: Sync + Send + 'static,
    {
        self.repo.get_mut::<Manager<T>>()?.get_e_mut(e)
    }
}
