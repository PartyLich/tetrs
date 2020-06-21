use {
    ecs::{
        self,
        // component_registry::Registry,
        types::{self as cell_types, Entity},
        ComponentRegistry,
    },
    std::collections::vec_deque::VecDeque,
};

mod collision;
pub mod component;
mod entity;
mod error;
pub mod input;
mod score;
mod system;
mod tetromino;
mod types;

pub use collision::*;
pub use entity::*;
pub use error::Error;
pub use system::*;
pub use tetromino::Tetromino;
pub use types::*;

pub const GAME_NAME: &str = "tetrs";

pub const WHITISH: cell_types::Cell = cell_types::Cell::RGB(225, 225, 225);
pub const PINK: cell_types::Cell = cell_types::Cell::RGB(255, 1, 154);
pub const BLUE: cell_types::Cell = cell_types::Cell::RGB(41, 12, 255);
pub const MIDNIGHT_BLUE: cell_types::Cell = cell_types::Cell::RGB(60, 52, 92);
pub const PURPLE: cell_types::Cell = cell_types::Cell::RGB(155, 0, 232);
pub const FONT_PATH: &str = "/resource/NotoSans-Regular.ttf";
pub const FONT_SIZE_SM: u16 = 12;
pub const FONT_SIZE_MD: u16 = 18;
pub const FONT_SIZE_LG: u16 = 24;

pub const TEXT_COLOR: cell_types::Cell = WHITISH;
pub const BG_CELL: cell_types::Cell = cell_types::Cell::RGB(0, 0, 0);
pub const UI_BG: cell_types::Cell = cell_types::Cell::RGB(30, 30, 30);
pub const GLASS_HEIGHT: usize = 20;
pub const GLASS_WIDTH: usize = 10;

// glass location
pub const X_OFFSET: i32 = 12;
pub const Y_OFFSET: i32 = 3;

pub const BASE_GRAVITY: cell_types::Vector2<i32> = cell_types::Vector2 { x: 0, y: 1 };

pub type TetrsResult<T> = Result<T, Error>;

pub struct GameState {
    pub ecs: ecs::World,

    pub current_piece: Option<Entity>,
    pub scoring: Option<Entity>,
    pub next_pieces: VecDeque<Entity>,
    pub hold_piece: Option<Entity>,
    pub grid: Vec<Vec<cell_types::Cell>>,
}

impl GameState {
    /// Create a new instance
    pub fn new() -> Self {
        Self {
            ecs: ecs::World::new(),
            scoring: None,
            current_piece: None,
            next_pieces: VecDeque::new(),
            hold_piece: None,
            grid: vec![vec![BG_CELL; GLASS_WIDTH]; GLASS_HEIGHT],
        }
    }

    // pub fn on_event(&mut self, evt: TetrsEvent) {
    pub fn on_event(&mut self, evt: GameEvent) {
        match evt {
            GameEvent::ClearedLines(cleared) => {
                on_cleared(self, cleared);
            }
            GameEvent::Collision(e) => {
                on_collision(self, e).unwrap();
            }
            _ => (),
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

/// Set all grid cells to initial state
pub fn clear_grid(state: &mut GameState) {
    state.grid = vec![vec![BG_CELL; GLASS_WIDTH]; GLASS_HEIGHT];
}

/// Translate the active tetromino one cell downward
pub fn simulate(state: &mut GameState) -> TetrsResult<Option<TimerEvent>> {
    // TODO apply based on aspect
    let piece = state.current_piece.ok_or(Error::NoCurrentPiece)?;
    let delta = {
        let ecs = &mut state.ecs;
        let registry = ecs.component_registry.lock().unwrap();
        let gravity = registry
            .get_component::<component::GravityComponent, _>(&piece)
            .ok_or(Error::MissingComponent("Gravity"))?;
        gravity.g
    };

    // check collision
    // let evt = check_collision(ecs, piece, delta)?;
    if let Some(GameEvent::Collision(e)) = check_collision(state, piece, delta)? {
        if let Ok(b) = check_gameover(&mut state.ecs, e) {
            if b {
                println!("\tGAME OVER");
                return Ok(Some(TimerEvent::Stop));
            }
        }
        return on_collision(state, e);
    }

    // move piece
    move_entity(&mut state.ecs, piece, delta)?;

    Ok(None)
}

/// translate an entity without collision check
fn move_entity(
    ecs: &mut ecs::World,
    e: Entity,
    delta: cell_types::Vector2<i32>,
) -> TetrsResult<()> {
    if !ecs.entity_manager.alive(&e) {
        return Ok(());
    }

    let mut registry = ecs.component_registry.lock().unwrap();
    let pos = registry
        .get_component_mut::<component::Position>(&e)
        .ok_or(Error::MissingComponent("Position"))?;

    *pos += delta;

    Ok(())
}

/// Rotate a tetromino entity
///
/// Components: mesh, size
fn rotate_tetromino(
    state: &mut GameState,
    e: Entity,
    direction: RotationDirection,
) -> TetrsResult<()> {
    let ecs = &mut state.ecs;
    if !ecs.entity_manager.alive(&e) {
        println!("rotate: ded ent");
        return Ok(());
    }

    let mut registry = ecs.component_registry.lock().unwrap();
    let mesh = &registry
        .get_component::<component::MeshComponent, _>(&e)
        .ok_or(Error::MissingComponent("Mesh"))?;
    let size = registry
        .get_component::<component::SizeComponent, _>(&e)
        .ok_or(Error::MissingComponent("Size"))?;
    let pos = registry
        .get_component::<component::PositionComponent, _>(&e)
        .ok_or(Error::MissingComponent("Position"))?;

    let mut new_mesh = Vec::from(mesh.0.clone());
    println!("pre rotate mesh: {:?}", new_mesh);

    // no point rotating an O tetromino
    if size.0 == 2 {
        return Ok(());
    }
    // translate origin
    let translate = size.0 as i32 - 2;
    println!("size - 2: {}", translate);

    let mut shift = ecs::types::Vector2 { x: 0, y: 0 };
    for cell in new_mesh.iter_mut() {
        // apply matrix rotation
        let (y, x) = match direction {
            RotationDirection::CW => {
                let y: i32 = 1 - (cell.x as i32 - translate);
                let x: i32 = cell.y as i32;
                (y, x)
            }
            RotationDirection::CCW => {
                let y: i32 = cell.x as i32;
                let x: i32 = 1 - (cell.y as i32 - translate);
                (y, x)
            }
        };

        cell.x = x as i32;
        cell.y = y as i32;
        let adjusted_x = x + pos.x as i32;
        let adjusted_y = y + pos.y as i32;

        // check boundaries
        println!("checking bounds: ({}, {})", adjusted_x, adjusted_y);
        if adjusted_y as usize >= GLASS_HEIGHT {
            println!(
                "rotation failed: bounds height {} + {}\n\t {}",
                y, pos.y, adjusted_y
            );
            return Ok(());
        }

        // wall kick
        if adjusted_x >= GLASS_WIDTH as i32 {
            let left_shift = GLASS_WIDTH as i32 - 1 - adjusted_x;
            shift.x = std::cmp::min(shift.x, left_shift);
            println!(
                "x ({}) shift adjust: {}, {}",
                adjusted_x, left_shift, shift.x
            );
        } else if adjusted_x < 0 {
            let right_shift = 0 - adjusted_x;
            shift.x = std::cmp::max(shift.x, right_shift);
            println!(
                "x ({}) shift adjust: {}, {}",
                adjusted_x, right_shift, shift.x
            );
        }
    }

    let new_pos = cell_types::Vector2 {
        x: (pos.x as i32 + shift.x) as i32,
        y: (pos.y as i32 + shift.y) as i32,
    };

    let mut old_mesh = mesh.0.clone();
    for cell in old_mesh.iter_mut() {
        *cell += *pos;
    }

    for cell in new_mesh.iter() {
        let y = new_pos.y + cell.y;
        let x = new_pos.x + cell.x;

        // it shouldnt be able to collide with itself
        if old_mesh.contains(&ecs::types::Vector2 { x, y }) {
            println!("prev posi");
            continue;
        }

        if state.grid[y as usize][x as usize] != BG_CELL {
            println!("rotation failed: bounds occupied ({}, {})", x, y);
            return Ok(());
        }
    }

    // update entity
    let new_comp = component::Mesh::new(VecDeque::from(new_mesh));
    if let Some(m) = registry.get_component_mut::<component::Mesh>(&e) {
        *m = new_comp;
    }
    if let Some(m) = registry.get_component_mut::<component::Position>(&e) {
        *m = new_pos;
    }

    Ok(())
}

/// remove entities in cleared lines
fn remove_cleared(state: &mut GameState, cleared: Vec<usize>) {
    if cleared.is_empty() {
        return;
    }

    let ecs = &mut state.ecs;
    let mut move_list = Vec::new();

    for line in cleared {
        for e in ecs.entity_list.iter() {
            let registry = ecs.component_registry.lock().unwrap();
            let pos = registry.get_component::<component::PositionComponent, _>(&e);
            let preview = registry.get_component::<component::PreviewComponent, _>(&e);
            if preview.is_some() || pos.is_none() {
                continue;
            }
            let pos = pos.unwrap();

            if pos.y == line as i32 {
                // remove it
                ecs.entity_manager.destroy(e);
                move_list.retain(|x| x != e);
            }
            if pos.y < line as i32 {
                // lower it
                move_list.push(*e);
            }
        }

        ecs.prune_dead();
    }

    for e in move_list {
        const DOWN: component::Position = component::Position { x: 0, y: 1 };
        move_entity(ecs, e, DOWN).unwrap_or_else(|e| panic!("{}", e));
    }
}

// update state afte line(s) cleared
fn on_cleared(ecs: &mut GameState, cleared: Vec<usize>) -> Option<TimerEvent> {
    let line_count = cleared.len();
    remove_cleared(ecs, cleared);
    score::update_score(ecs, line_count).unwrap_or(None)
}

// update state following a collision event
fn on_collision(ecs: &mut GameState, e: Entity) -> TetrsResult<Option<TimerEvent>> {
    let mut result = None;
    ecs.current_piece = None;
    decompose_mesh(ecs, e)?;

    let cleared = get_clear_lines(ecs);
    println!("cleared: {:?}", cleared);
    if let Some(GameEvent::ClearedLines(cleared)) = cleared {
        result = on_cleared(ecs, cleared);
    }

    make_player(ecs);

    Ok(result)
}

/// Break a single mesh into multiple entities, one per cell
fn decompose_mesh(state: &mut GameState, e: Entity) -> TetrsResult<()> {
    let ecs = &mut state.ecs;
    if !ecs.entity_manager.alive(&e) {
        return Ok(());
    }

    let (mesh, color, pos) = {
        let registry = ecs.component_registry.lock().unwrap();
        let mesh = registry
            .get_component::<component::MeshComponent, _>(&e)
            .ok_or(Error::MissingComponent("Mesh"))?;
        let color = registry
            .get_component::<component::ColorComponent, _>(&e)
            .ok_or(Error::MissingComponent("Color"))?;
        let pos = registry
            .get_component::<component::PositionComponent, _>(&e)
            .ok_or(Error::MissingComponent("Position"))?;

        (mesh.0.clone(), *color, *pos)
    };

    for position in mesh {
        create_single_cell(ecs, color, position + pos);
    }

    // remove old entity
    ecs.entity_manager.destroy(&e);
    ecs.prune_dead();

    Ok(())
}

fn get_scoring<F>(state: &GameState, f: F) -> TetrsResult<u32>
where
    F: Fn(&component::Scoring) -> u32,
{
    let e = state.scoring.unwrap();
    let ecs = &state.ecs;
    let registry = ecs.component_registry.lock().unwrap();
    registry
        .get_component::<component::ScoringComponent, _>(&e)
        .ok_or(Error::MissingComponent("Scoring"))
        .map(f)
}

fn get_lines(state: &GameState) -> TetrsResult<u32> {
    get_scoring(state, |s| s.lines)
}

fn get_level(state: &GameState) -> TetrsResult<u32> {
    get_scoring(state, |s| s.level)
}

fn get_score(state: &GameState) -> TetrsResult<u32> {
    get_scoring(state, |s| s.score)
}

/// adjust state after a Hold event
fn on_hold(state: &mut GameState) -> TetrsResult<()> {
    let player = state.current_piece.unwrap();

    // use the existing hold piece, or take a new player from the queue
    if let Some(hold) = state.hold_piece {
        player_from_hold(&mut state.ecs, hold);
        state.current_piece = Some(hold);
    } else {
        make_player(state);
    }

    hold_from_player(&mut state.ecs, player);
    state.hold_piece = Some(player);

    Ok(())
}
