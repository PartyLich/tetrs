//! Collision functions
use crate::{component, Error, GameEvent, GameState, TetrsResult};
use ecs::{types as cell_types, types::Entity, ComponentRegistry, World};

pub fn check_collision(
    state: &mut GameState,
    e: Entity,
    delta: component::Position,
) -> TetrsResult<Option<GameEvent>> {
    let ecs = &mut state.ecs;
    let registry = ecs.component_registry.lock().unwrap();
    let pos = registry
        .get_component2::<component::Position>(&e)
        .ok_or(Error::MissingComponent("Position"))?;
    let mesh = registry
        .get_component::<component::MeshComponent, _>(&e)
        .ok_or(Error::MissingComponent("Mesh"))?;

    let new_pos = *pos + delta;
    let old_mesh: Vec<cell_types::Vector2<_>> = Vec::from(mesh.0.clone())
        .iter()
        .map(|c| *c + *pos)
        .collect();

    for cell in mesh.0.iter() {
        let y = cell.y + new_pos.y;
        let x = cell.x + new_pos.x;
        if old_mesh.contains(&cell_types::Vector2 { x, y }) {
            continue;
        }

        if x >= crate::GLASS_WIDTH as i32
            || y >= crate::GLASS_HEIGHT as i32
            || x < 0
            || state.grid[y as usize][x as usize] != crate::BG_CELL
        {
            // collision
            return Ok(Some(GameEvent::Collision(e)));
        }
    }

    Ok(None)
}

/// Get cleared lines. Returns an empty vec (len == 0) if no lines are cleared.
///
/// A cleared line is any row in the game grid that has no empty (background) cells.
pub fn get_clear_lines(state: &mut GameState) -> Option<GameEvent> {
    let mut cleared = Vec::new();
    for (r, row) in state.grid.iter().enumerate().rev() {
        // an empty column -> we're done with this row
        if !row.iter().any(|cell| *cell == crate::BG_CELL) {
            cleared.push(r);
        }
    }

    if cleared.is_empty() {
        return None;
    }
    Some(GameEvent::ClearedLines(cleared))
}

/// sort mesh by y, then x, for convenient collision checks
pub fn mesh_sort<T>(a: &cell_types::Vector2<T>, b: &cell_types::Vector2<T>) -> std::cmp::Ordering
where
    T: Clone + std::cmp::Ord,
{
    if a.y > b.y {
        return std::cmp::Ordering::Less;
    }
    if a.y < b.y {
        return std::cmp::Ordering::Greater;
    }
    a.x.cmp(&b.x)
}

pub fn check_gameover(ecs: &mut World, e: Entity) -> TetrsResult<bool> {
    let registry = ecs.component_registry.lock().unwrap();
    let pos = registry
        .get_component2::<component::Position>(&e)
        .ok_or(Error::MissingComponent("Position"))?;

    Ok(pos.y <= 1)
}
