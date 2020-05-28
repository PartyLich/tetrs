const ENTITY_INDEX_BITS: u32 = 22;
const ENTITY_INDEX_MASK: u32 = (1 << ENTITY_INDEX_BITS) - 1;

const ENTITY_GENERATION_BITS: u32 = 8;
const ENTITY_GENERATION_MASK: u32 = (1 << ENTITY_GENERATION_BITS) - 1;

/// An Entity that exists within (one or more of) the systems
///
/// id uses 30 bits split into 22 bits for the index and 8 bits for the generation. This means that
/// we support a maximum of ~4 million simultaneous entities
#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
pub struct Entity {
    id: u32,
}

impl Entity {
    /// Returns the index of this Entity
    pub fn index(&self) -> u32 {
        self.id & ENTITY_INDEX_MASK
    }

    /// Returns the generation of this Entity
    pub fn generation(&self) -> u32 {
        (self.id >> ENTITY_INDEX_BITS) & ENTITY_GENERATION_MASK
    }

    pub fn value(&self) -> u32 {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_index() {
        let e = Entity {
            id: 0b10000000000000000001010,
        };
        let actual = e.index();
        let expected = 10;
        assert_eq!(actual, expected);
    }

    #[test]
    fn entity_generation() {
        let e = Entity {
            id: 0b100000000000000000001010,
        };
        let actual = e.generation();
        let expected = 2;
        assert_eq!(actual, expected);
    }

    #[test]
    fn entity_value() {
        let e = Entity {
            id: 0b100000000000000000001010,
        };
        let actual = e.value();
        let expected = 0b100000000000000000001010;
        assert_eq!(actual, expected);
    }
}
