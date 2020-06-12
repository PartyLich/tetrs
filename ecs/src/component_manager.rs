//! component manager
use std::collections::HashMap;

use entity_man::{Entity, EntityManager};

/// Handle to a component instance.
pub type Instance = usize;

#[derive(Debug)]
struct InstanceData<T> {
    /// List of Entities with this component
    // entity: Vec<Box<dyn Entity>>,
    entity: Vec<Entity>,
    /// Entity colors
    value: Vec<T>,
}

impl<T> Default for InstanceData<T> {
    fn default() -> Self {
        Self {
            entity: Vec::new(),
            value: Vec::new(),
        }
    }
}

// pub trait ComponentManager<T> {
pub trait ComponentManager<T>: Default {
    /// Returns the component instance for the specified entity or a `None` if the entity doesn't
    /// have the component.
    fn lookup(&self, e: &Entity) -> Option<Instance>;

    //  returns an iterator of the entities this component manager has
    // fn entities_iter(&self) -> Iter<Item = Entity> ;

    // /// Return all entities with this component
    // pub fn entities(&self) -> Vec<&Entity>;
    fn entities(&self) -> std::collections::hash_map::Keys<Entity, usize>;

    /// Returns ref to the instance data for the specified component instance
    fn get(&self, i: Instance) -> &T;

    /// Returns mutable ref to the instance data for the specified component instance
    fn get_mut(&mut self, i: Instance) -> &mut T;

    // get methods based on entity
    fn get_e(&self, e: &Entity) -> Option<&T>;
    fn get_e_mut(&mut self, e: &Entity) -> Option<&mut T>;

    /// Set the value for the specified component instance
    fn set_value(&mut self, i: Instance, value: T);

    /// Add this component for the supplied Entity
    fn create(&mut self, e: Entity, value: T) -> Instance;

    /// Remove an entity from this system
    fn destroy(&mut self, i: Instance);

    /// Clean ALL dead entities. Could be time consuming, use only with small entity counts
    // fn clean_all(&mut self, manager: Box<dyn EntityManager>) ;
    fn clean_all(&mut self, manager: &dyn EntityManager);
}

/// A ComponentManager for
#[derive(Debug)]
pub struct Manager<T> {
    /// Entity data
    data: InstanceData<T>,
    /// Entity to vec index map
    map: HashMap<Entity, usize>,
}

impl<T> Manager<T> {
    pub fn new() -> Self {
        Self {
            data: InstanceData::default(),
            map: HashMap::new(),
        }
    }

    /// Create an instance from an index to the data arrays.
    fn make_instance(i: usize) -> Instance {
        i
    }
}

impl<T> Default for Manager<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ComponentManager<T> for Manager<T> {
    fn lookup(&self, e: &Entity) -> Option<Instance> {
        let i = self.map.get(e)?;
        Some(Self::make_instance(*i))
    }

    /// Returns the instance data for the specified component instance
    fn get(&self, i: Instance) -> &T {
        &self.data.value[i]
    }

    fn get_mut(&mut self, i: Instance) -> &mut T {
        &mut self.data.value[i]
    }

    fn get_e(&self, e: &Entity) -> Option<&T> {
        let i = self.lookup(e)?;
        Some(self.get(i))
    }

    fn get_e_mut(&mut self, e: &Entity) -> Option<&mut T> {
        let i = self.lookup(e)?;
        Some(self.get_mut(i))
    }

    /// Set the mesh for the specified component instance
    fn set_value(&mut self, i: Instance, value: T) {
        self.data.value[i] = value;
    }

    /// Add this component for the supplied Entity
    fn create(&mut self, e: Entity, value: T) -> Instance {
        let i = self.data.entity.len();

        self.data.entity.push(e);
        self.data.value.push(value);
        self.map.insert(e, i);

        Self::make_instance(i)
    }

    /// Remove an entity from this system
    fn destroy(&mut self, i: Instance) {
        let last = self.data.entity.len() - 1;
        let entity = self.data.entity[i];

        if i == last {
            self.map.remove(&entity);
            return;
        }

        let last_entity = self.data.entity[last];

        // move last entity to the index of the destroyed entity
        // self.data.entity[i] = self.data.entity[last];
        // self.data.value[i] = self.data.value.pop().unwrap();
        self.data.entity.swap_remove(i);
        self.data.value.swap_remove(i);

        // update entity -> instance map
        self.map.insert(last_entity, i);
        self.map.remove(&entity);
    }

    // /// Return all entities with this component
    // pub fn entities(&self) -> Vec<&Entity> {
    //     self.map.keys().collect()
    fn entities(&self) -> std::collections::hash_map::Keys<Entity, usize> {
        self.map.keys()
    }

    //
    // fn entities_iter(&self) -> Iter<Item = Entity> {
    //     self.map.keys()
    // }

    /// Clean ALL dead entities. Could be time consuming, use only with small entity counts
    // fn clean_all(&mut self, manager: Box<dyn EntityManager>) {
    fn clean_all(&mut self, manager: &dyn EntityManager) {
        for i in 0..self.data.entity.len() {
            if !manager.alive(&self.data.entity[i]) {
                self.destroy(i);
            }
        }
    }
}
