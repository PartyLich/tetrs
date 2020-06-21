use std::collections::HashMap;

use sdl2::keyboard::Keycode;

use crate::{component::Position, GameState, InputEvent, KeyState, RotationDirection, TetrsResult};

type KeyMap = HashMap<(KeyState, Keycode), InputEvent>;

pub fn default_keymap() -> KeyMap {
    let mut map = HashMap::new();
    map.insert((KeyState::Down, Keycode::Up), InputEvent::RotateCCW);
    map.insert((KeyState::Down, Keycode::W), InputEvent::RotateCCW);
    map.insert((KeyState::Down, Keycode::Left), InputEvent::Left);
    map.insert((KeyState::Down, Keycode::A), InputEvent::Left);
    map.insert((KeyState::Down, Keycode::Down), InputEvent::SoftDrop);
    map.insert((KeyState::Down, Keycode::S), InputEvent::SoftDrop);
    map.insert((KeyState::Down, Keycode::Right), InputEvent::Right);
    map.insert((KeyState::Down, Keycode::D), InputEvent::Right);
    map.insert((KeyState::Down, Keycode::Space), InputEvent::HardDrop);
    map.insert((KeyState::Down, Keycode::P), InputEvent::Pause);

    map.insert((KeyState::Down, Keycode::RCtrl), InputEvent::RotateCW);
    map.insert((KeyState::Down, Keycode::LShift), InputEvent::RotateCW);

    map.insert((KeyState::Down, Keycode::LAlt), InputEvent::Hold);

    map
}

/// Map keycodes to player movement direction
pub fn map_key_input(keyevent: KeyState, keycode: Keycode) -> Option<InputEvent> {
    match keyevent {
        KeyState::Down => match keycode {
            Keycode::Up | Keycode::W => Some(InputEvent::RotateCCW),
            Keycode::RCtrl | Keycode::LShift => Some(InputEvent::RotateCW),
            Keycode::Left | Keycode::A => Some(InputEvent::Left),
            Keycode::Down | Keycode::S => Some(InputEvent::SoftDrop),
            Keycode::Right | Keycode::D => Some(InputEvent::Right),
            Keycode::Space => Some(InputEvent::HardDrop),
            Keycode::LAlt => Some(InputEvent::Hold),
            Keycode::P => Some(InputEvent::Pause),
            _ => None,
        },
        KeyState::Up => match keycode {
            Keycode::Down | Keycode::S => Some(InputEvent::SoftDropEnd),
            _ => None,
        },
    }
}

/// Map keycodes to player movement direction using the map `keymap`
pub fn map_key_input2(
    keymap: &KeyMap,
    keyevent: KeyState,
    keycode: Keycode,
) -> Option<&InputEvent> {
    keymap.get(&(keyevent, keycode))
}

pub fn on_input_event(ecs: &mut crate::GameState, evt: InputEvent) {
    let on_left: MovePlayerFn = move_player(Position { x: -1, y: 0 });
    let on_right: MovePlayerFn = move_player(Position { x: 1, y: 0 });

    match evt {
        InputEvent::RotateCCW => {
            if let Some(e) = ecs.current_piece {
                crate::rotate_tetromino(ecs, e, RotationDirection::CCW).unwrap();
            }
        }
        InputEvent::RotateCW => {
            if let Some(e) = ecs.current_piece {
                crate::rotate_tetromino(ecs, e, RotationDirection::CW).unwrap();
            }
        }
        InputEvent::Right => {
            on_right(ecs).unwrap();
        }
        InputEvent::Left => {
            on_left(ecs).unwrap();
        }
        InputEvent::HardDrop => {
            hard_drop(ecs).unwrap();
        }
        InputEvent::SoftDrop => {
            // crate::set_gravity()
        }
        InputEvent::SoftDropEnd => {
            // crate::set_gravity()
        }
        InputEvent::Hold => {
            super::on_hold(ecs).unwrap();
        }
        _ => (),
    }
}

type MovePlayerFn = Box<dyn Fn(&mut GameState) -> TetrsResult<()>>;
fn move_player(delta: Position) -> MovePlayerFn {
    Box::new(move |state: &mut GameState| -> TetrsResult<()> {
        if let Some(e) = state.current_piece {
            if crate::collision::check_collision(state, e, delta)?.is_none() {
                crate::move_entity(&mut state.ecs, e, delta)?;
            }
        }
        Ok(())
    })
}

/// Drop the current piece until it collides
fn hard_drop(state: &mut GameState) -> crate::TetrsResult<()> {
    let piece = state.current_piece.ok_or(crate::Error::NoCurrentPiece)?;
    const STEP: Position = Position { x: 0, y: 1 };
    let mut delta = Position { x: 0, y: 1 };

    while crate::check_collision(state, piece, delta)?.is_none() {
        delta += STEP;
    }
    delta -= STEP;
    crate::move_entity(&mut state.ecs, piece, delta)?;

    Ok(())
}
