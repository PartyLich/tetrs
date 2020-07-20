use std::{sync::mpsc, thread, time};

use cell_engine::gfx;
use sdl2::{event::Event, keyboard::Keycode, render::Canvas, ttf, video::Window};
use tetrs::GameState;

#[cfg(debug_assertions)]
fn get_root_dir() -> String {
    env!("CARGO_MANIFEST_DIR").to_string()
}

#[cfg(not(debug_assertions))]
fn get_root_dir() -> String {
    ".".to_string()
}

fn main() {
    const CANVAS_WIDTH: u32 = 720_u32;
    const CANVAS_HEIGHT: u32 = 540;
    const ROWS: u32 = 36;
    const CELL_WIDTH: u32 = CANVAS_WIDTH / ROWS;

    let (mut canvas, mut event_pump) = gfx::init(tetrs::GAME_NAME, CANVAS_WIDTH, CANVAS_HEIGHT);

    // fonts. keep the ttf context on the stack, can't move it, etc
    let ttf_context = ttf::init().unwrap();
    let font_path = [&get_root_dir(), tetrs::FONT_PATH].concat();
    println!("font path: {}", font_path);
    let game_font = gfx::init_font(&ttf_context, &font_path, tetrs::FONT_SIZE_MD);

    run_game(&mut canvas, &mut event_pump, &game_font, CELL_WIDTH);
}

/// Panic while printing the error
fn panic_with_err(e: impl std::error::Error) {
    panic!("{}", e)
}

type MapInputFn = Box<dyn FnMut(&mut GameState, Keycode)>;
fn map_input(keystate: tetrs::KeyState) -> MapInputFn {
    Box::new(move |mut state, k| {
        let evt = tetrs::input::map_key_input(keystate, k);
        if let Some(evt) = evt {
            tetrs::input::on_input_event(&mut state, evt);
        }
    })
}

fn run_game(
    canvas: &mut Canvas<Window>,
    event_pump: &mut sdl2::EventPump,
    font: &ttf::Font,
    cell_width: u32,
) {
    let mut state = GameState::new();
    tetrs::component::load_registry(&mut *state.ecs.component_registry.lock().unwrap());
    tetrs::create_entities(&mut state);
    let _keymap = tetrs::input::default_keymap();

    let (to_game, game_rx) = mpsc::channel::<tetrs::TimerEvent>();
    let (to_timer, timer_rx) = mpsc::channel();
    create_timer(to_game, timer_rx, 300);

    println!("starting game loop");
    'game: loop {
        for event in event_pump.poll_iter() {
            match event {
                // exit on escape key
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    to_timer.send(tetrs::TimerEvent::Stop).unwrap();
                    break 'game;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => {
                    to_timer.send(tetrs::TimerEvent::Pause).unwrap();
                    continue 'game;
                }

                // user input keys
                Event::KeyDown {
                    keycode: Some(k), ..
                } => {
                    map_input(tetrs::KeyState::Down)(&mut state, k);
                }
                Event::KeyUp {
                    keycode: Some(k), ..
                } => {
                    map_input(tetrs::KeyState::Up)(&mut state, k);
                }

                _ => continue 'game,
            }
        }

        tetrs::clear_grid(&mut state);

        tetrs::draw_ui_bg(canvas, cell_width).unwrap_or_else(panic_with_err);
        tetrs::draw_ui(&mut state, canvas, cell_width, font).unwrap_or_else(panic_with_err);
        tetrs::draw_entities(&mut state, canvas, cell_width).unwrap_or_else(panic_with_err);
        tetrs::draw_previews(&mut state, canvas, cell_width).unwrap_or_else(panic_with_err);
        tetrs::draw_hold(&mut state, canvas, cell_width).unwrap_or_else(panic_with_err);
        gfx::display_frame(canvas);

        // stuff that happens each game tick
        match game_rx.try_recv() {
            Err(mpsc::TryRecvError::Empty) => (),
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("Timer disconnected");
                return;
            }
            _ => {
                // update entities
                match tetrs::simulate(&mut state) {
                    Err(e) => {
                        println!("err: {}", e);
                    }
                    Ok(Some(evt)) => {
                        println!("timer evt: {:?}", evt);
                        to_timer.send(evt).unwrap();
                    }
                    _ => (),
                }
            }
        }
    }

    to_timer.send(tetrs::TimerEvent::Stop).unwrap();
}

/// Create a timer thread that sends tick events when the interval has elapsed
fn create_timer(
    tx: mpsc::Sender<tetrs::TimerEvent>,
    receiver: mpsc::Receiver<tetrs::TimerEvent>,
    mut interval: u128,
) {
    thread::spawn(move || {
        let mut pause = false;
        let mut last_time = time::Instant::now();

        'timer: loop {
            match receiver.try_recv() {
                Err(mpsc::TryRecvError::Empty) => (),
                Ok(tetrs::TimerEvent::Pause) => {
                    pause = !pause;
                }
                Ok(tetrs::TimerEvent::SetInterval(i)) => {
                    interval = i;
                }
                // exit on signal or sender disconnect
                _ => {
                    println!("timer exiting");
                    break 'timer;
                }
            }

            let now = time::Instant::now();
            let dt = now.duration_since(last_time);

            if !pause && dt.as_millis() >= interval {
                last_time = now;
                if tx.send(tetrs::TimerEvent::Tick).is_err() {
                    break 'timer;
                }
            }
        }
    });
}
