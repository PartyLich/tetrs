use std::collections::VecDeque;

use ecs::{
    component_manager::Manager,
    types::{Cell, Vector2},
    ComponentRegistry,
};

pub type ColorComponent = Manager<Cell>;

#[derive(Debug, Default)]
pub struct Size(pub u32);
pub type SizeComponent = Manager<Size>;

pub type Position = Vector2<i32>;
pub type PositionComponent = Manager<Position>;

#[derive(Debug, Default)]
pub struct Scoring {
    pub lines: u32,
    pub level: u32,
    pub score: u32,
}

impl Scoring {
    pub fn new() -> Self {
        Self::default()
    }
}

pub type ScoringComponent = Manager<Scoring>;

/// marker component. indicates player control
#[derive(Debug, Default)]
pub struct Player;
pub type PlayerComponent = Manager<Player>;

/// marker component. upcoming piece preview marker
#[derive(Debug, Default)]
pub struct Preview;
pub type PreviewComponent = Manager<Preview>;

/// marker component. indicates player control
#[derive(Debug, Default)]
pub struct Hold;
pub type HoldComponent = Manager<Hold>;

#[derive(Debug)]
pub struct Gravity {
    pub g: Vector2<i32>,
}

impl Gravity {
    pub fn new(g: Vector2<i32>) -> Self {
        Self { g }
    }
}

impl Default for Gravity {
    fn default() -> Self {
        Self {
            g: crate::BASE_GRAVITY,
        }
    }
}

pub type GravityComponent = Manager<Gravity>;

#[derive(Debug, Clone)]
pub struct Mesh(pub VecDeque<Vector2<i32>>);

impl Mesh {
    pub fn new(value: VecDeque<Vector2<i32>>) -> Self {
        Self(value)
    }

    pub fn translate(&mut self, vector: Vector2<i32>) {
        for node in self.0.iter_mut() {
            *node += vector;
        }
    }

    pub fn rotate_cw(&self) -> Self {
        let mut new_mesh = Vec::from(self.0.clone());

        for cell in new_mesh.iter_mut() {
            // apply matrix rotation
            cell.y = -cell.x;
            cell.x = cell.y;
        }

        Self::new(VecDeque::from(new_mesh))
    }

    pub fn rotate_ccw(&mut self) -> Self {
        let mut new_mesh = Vec::from(self.0.clone());

        for cell in new_mesh.iter_mut() {
            // apply matrix rotation
            cell.y = cell.x;
            cell.x = -cell.y;
        }

        Self::new(VecDeque::from(new_mesh))
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self(VecDeque::new())
    }
}

pub type MeshComponent = Manager<Mesh>;

pub fn load_registry<'a, T>(component_registry: &mut T)
where
    T: ComponentRegistry<'a>,
{
    component_registry.register::<ColorComponent>();
    component_registry.register2::<Mesh>();
    component_registry.register2::<Size>();
    component_registry.register2::<Position>();

    component_registry.register2::<Player>();
    component_registry.register2::<Preview>();

    component_registry.register2::<Scoring>();
    component_registry.register2::<Gravity>();

    component_registry.register2::<Hold>();
}
