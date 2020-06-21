/// Directions of rotation. May be clockwise (CW) or counter-clockwise (CCW)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RotationDirection {
    CW,
    CCW,
}

/// Events generated by user input
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputEvent {
    HardDrop,
    Hold,
    Left,
    Pause,
    Right,
    RotateCCW,
    RotateCW,
    SoftDrop,
    SoftDropEnd,
}

/// Keystate. Press (down) or release (up)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyState {
    Up,
    Down,
}

/// Events generated by game rules
#[derive(Debug, Clone, PartialEq)]
pub enum GameEvent {
    GameOver,
    Collision(ecs::types::Entity),
    ClearedLines(Vec<usize>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TetrsEvent {
    InputEvent,
    GameEvent,
}

/// Events generated by or controlling a Timer
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimerEvent {
    Tick,
    Pause,
    Stop,
    SetInterval(u128),
}
