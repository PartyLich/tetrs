//! Entity creation functions
use crate::{component, GameState, Tetromino, TetrsResult};
use ecs::{types::Entity, World};
use std::collections::VecDeque;

type Position = ecs::types::Vector2<i32>;

/// add a tetromino entity to the game
fn create(ecs: &mut World, kind: Tetromino) -> TetrsResult<Entity> {
    let e = ecs
        .build_entity()
        .with(kind.mesh())
        .with(kind.color())
        .with(component::Size(kind.size()))
        .done();

    Ok(e)
}

/// create a preview piece
pub fn make_preview(state: &mut GameState) -> Entity {
    let ecs = &mut state.ecs;
    let e = create(ecs, crate::Tetromino::random()).unwrap();
    ecs.add_component_default::<crate::component::Preview>(e)
        .unwrap();
    state.next_pieces.push_back(e);

    e
}

/// add player control marker
fn add_player_control(ecs: &mut World, e: Entity) -> TetrsResult<Entity> {
    ecs.add_component_default::<component::Player>(e)
        .map_err(|e| e.into())
}

/// turn the next piece into a player controlled piece
pub fn make_player(state: &mut GameState) -> Entity {
    let e = state.next_pieces.pop_front().unwrap();
    let ecs = &mut state.ecs;
    ecs.remove_component::<component::Preview>(e).unwrap();
    ecs.add_component::<Position>(e, Position { x: 4, y: 0 })
        .unwrap();
    ecs.add_component_default::<component::Gravity>(e).unwrap();
    add_player_control(ecs, e).unwrap();
    make_preview(state);

    state.current_piece = Some(e);

    e
}

/// Create new single cell entity
pub fn create_single_cell(ecs: &mut World, color: ecs::types::Cell, position: Position) {
    let new_mesh = VecDeque::from(vec![Position { x: 0, y: 0 }]);
    let new_mesh = component::Mesh::new(new_mesh);
    ecs.build_entity()
        .with(new_mesh)
        .with(color)
        .with(position)
        .done();
}

/// Create a Scoring entity
pub fn make_scoring(state: &mut GameState) -> Entity {
    let ecs = &mut state.ecs;
    let e = ecs.build_entity().with(component::Scoring::new()).done();
    state.scoring = Some(e);

    e
}

/// Turn a Player controlled entity into a Hold entity
pub fn hold_from_player(ecs: &mut World, e: Entity) -> Entity {
    ecs.remove_component::<component::Player>(e).unwrap();
    ecs.remove_component::<Position>(e).unwrap();
    ecs.add_component_default::<component::Hold>(e).unwrap();
    e
}

/// Turn a Hold entity into a Player controlled entity
pub fn player_from_hold(ecs: &mut World, e: Entity) -> Entity {
    ecs.add_component_default::<component::Player>(e).unwrap();
    ecs.add_component::<Position>(e, Position { x: 4, y: 0 })
        .unwrap();
    ecs.remove_component::<component::Hold>(e).unwrap();
    e
}

/// Create initial game entities
pub fn create_entities(state: &mut GameState) {
    make_scoring(state);
    for _ in 0..4 {
        make_preview(state);
    }
    make_player(state);
}
