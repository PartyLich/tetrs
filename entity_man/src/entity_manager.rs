use std::collections::VecDeque;

use super::{Entity, EntityManager, ENTITY_INDEX_BITS};

/// Keeps track of entitie with generational indexes
#[derive(Debug, PartialEq)]
pub struct EntityManagerU32 {
    /// bucket array where keys are entity id and values are generation
    generation: Vec<u8>,
    /// indices available for reuse
    free_indices: VecDeque<u32>,
}

impl EntityManagerU32 {
    const MINIMUM_FREE_INDICES: usize = 1024;

    /// Create a new `EntityManager`
    pub fn new() -> Self {
        Self {
            generation: Vec::new(),
            free_indices: VecDeque::new(),
        }
    }

    /// Combine index and generation
    fn make_entity(index: u32, generation: u8) -> Entity {
        let id = index | ((generation as u32) << ENTITY_INDEX_BITS);
        Entity { id }
    }
}

impl EntityManager for EntityManagerU32 {
    /// Create a new Entity
    fn create(&mut self) -> Entity {
        let idx: u32 = if self.free_indices.len() > Self::MINIMUM_FREE_INDICES {
            // take from the recycling queue only if there are a min amount available
            self.free_indices.pop_front().unwrap()
        } else {
            // extend generation vec
            self.generation.push(0);
            // set id to last index in generation vec
            let idx = (self.generation.len() - 1) as u32;
            // ??? ensure index fits within the available bits
            x_ensure(idx < (1 << ENTITY_INDEX_BITS));

            idx
        };

        Self::make_entity(idx, self.generation[idx as usize])
    }

    /// Returns true if the supplied Entity is still alive
    fn alive(&self, e: &Entity) -> bool {
        self.generation[e.index() as usize] as u32 == e.generation()
    }

    /// Removes the supplied Entity and frees its index for reuse
    fn destroy(&mut self, e: &Entity) {
        let idx = e.index();
        // increment generation for this id. if the id is reused, it will have a higher generation
        self.generation[idx as usize] += 1;
        // add index to the recycling queue
        self.free_indices.push_back(idx);
    }
}

/// ???
/// # Panic
/// Panics if the predicate is `false`
fn x_ensure(b: bool) {
    if !b {
        panic!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_entity() {
        let actual = EntityManagerU32::make_entity(10, 2);
        let expected = Entity {
            id: 0b100000000000000000001010,
        };
        assert_eq!(actual, expected);
    }
}
