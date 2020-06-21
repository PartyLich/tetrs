use std::collections::VecDeque;

use ecs::{self, types};
use rand::Rng;

const O_COLOR: types::Cell = types::Cell::RGB(0, 0, 255);
const I_COLOR: types::Cell = types::Cell::RGB(255, 0, 0);
const J_COLOR: types::Cell = types::Cell::RGB(255, 255, 255);
const L_COLOR: types::Cell = types::Cell::RGB(255, 0, 255);
const Z_COLOR: types::Cell = types::Cell::RGB(0, 255, 255);
const S_COLOR: types::Cell = types::Cell::RGB(0, 255, 0);
const T_COLOR: types::Cell = types::Cell::RGB(255, 255, 0);

/// one-sided tetromino types
pub enum Tetromino {
    O,
    I,
    J,
    L,
    Z,
    S,
    T,
}

impl Tetromino {
    /// Create a mesh component for this type of Tetromino
    pub fn new(&self) -> crate::component::Mesh {
        match self {
            Self::O => tetro_o(),
            Self::I => tetro_i(),
            Self::J => tetro_j(),
            Self::L => tetro_l(),
            Self::Z => tetro_z(),
            Self::S => tetro_s(),
            Self::T => tetro_t(),
        }
    }

    /// Get the color for this type of Tetromino
    pub fn color(&self) -> types::Cell {
        match self {
            Self::O => O_COLOR,
            Self::I => I_COLOR,
            Self::J => J_COLOR,
            Self::L => L_COLOR,
            Self::Z => Z_COLOR,
            Self::S => S_COLOR,
            Self::T => T_COLOR,
        }
    }

    /// Get the size for this type of Tetromino
    pub fn size(&self) -> u32 {
        match self {
            Self::O => 2,
            Self::I => 4,
            Self::J => 3,
            Self::L => 3,
            Self::Z => 3,
            Self::S => 3,
            Self::T => 3,
        }
    }

    /// Get a random variant of Tetromino
    pub fn random() -> Self {
        let row = rand::thread_rng().gen_range(0, 7);
        match row {
            row if row == 0 => Tetromino::O,
            row if row == 1 => Tetromino::I,
            row if row == 2 => Tetromino::J,
            row if row == 3 => Tetromino::L,
            row if row == 4 => Tetromino::Z,
            row if row == 5 => Tetromino::S,
            _ => Tetromino::T,
        }
    }
}

fn tetro_t() -> crate::component::Mesh {
    let mesh = VecDeque::from(vec![
        types::Vector2 { x: 0, y: 1 },
        types::Vector2 { x: 1, y: 1 },
        types::Vector2 { x: 2, y: 1 },
        types::Vector2 { x: 1, y: 0 },
    ]);

    crate::component::Mesh::new(mesh)
}

fn tetro_i() -> crate::component::Mesh {
    let mesh = VecDeque::from(vec![
        types::Vector2 { x: 0, y: 1 },
        types::Vector2 { x: 1, y: 1 },
        types::Vector2 { x: 2, y: 1 },
        types::Vector2 { x: 3, y: 1 },
    ]);

    crate::component::Mesh::new(mesh)
}

fn tetro_o() -> crate::component::Mesh {
    let mesh = VecDeque::from(vec![
        types::Vector2 { x: 0, y: 1 },
        types::Vector2 { x: 1, y: 1 },
        types::Vector2 { x: 1, y: 0 },
        types::Vector2 { x: 0, y: 0 },
    ]);

    crate::component::Mesh::new(mesh)
}

fn tetro_j() -> crate::component::Mesh {
    let mesh = VecDeque::from(vec![
        types::Vector2 { x: 0, y: 1 },
        types::Vector2 { x: 1, y: 1 },
        types::Vector2 { x: 2, y: 1 },
        types::Vector2 { x: 2, y: 0 },
    ]);

    crate::component::Mesh::new(mesh)
}

fn tetro_l() -> crate::component::Mesh {
    let mesh = VecDeque::from(vec![
        types::Vector2 { x: 0, y: 1 },
        types::Vector2 { x: 1, y: 1 },
        types::Vector2 { x: 2, y: 1 },
        types::Vector2 { x: 0, y: 0 },
    ]);

    crate::component::Mesh::new(mesh)
}

fn tetro_s() -> crate::component::Mesh {
    let mesh = VecDeque::from(vec![
        types::Vector2 { x: 0, y: 1 },
        types::Vector2 { x: 1, y: 1 },
        types::Vector2 { x: 1, y: 0 },
        types::Vector2 { x: 2, y: 0 },
    ]);

    crate::component::Mesh::new(mesh)
}

fn tetro_z() -> crate::component::Mesh {
    let mesh = VecDeque::from(vec![
        types::Vector2 { x: 1, y: 1 },
        types::Vector2 { x: 2, y: 1 },
        types::Vector2 { x: 0, y: 0 },
        types::Vector2 { x: 1, y: 0 },
    ]);

    crate::component::Mesh::new(mesh)
}
