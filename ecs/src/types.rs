use sdl2::pixels::Color;

mod vector;

pub use entity_man::Entity;
pub use vector::Vector2;

/// A single square on the game board
pub type Cell = Color;

/// The gameboard
pub type Grid = Vec<Vec<Cell>>;

/// A location on the 2dimensional gameboard
pub type Position = (u32, u32);
