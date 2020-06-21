use sdl2::pixels::Color;
use sdl2::{rect::Rect, render::Canvas, ttf, video::Window, EventPump};

use crate::menu;

/// Initialize the canvas
pub fn init(game_name: &str, width: u32, height: u32) -> (Canvas<Window>, EventPump) {
    let sdl_context = sdl2::init().expect("Failed to init SDL");
    let video_subsystem = sdl_context.video().expect("Failed to init video subsystem");

    let window = video_subsystem
        .window(game_name, width + 1, height + 1)
        .position_centered()
        .build()
        .expect("Failed to build window");
    let canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .expect("Failed to get canvas from window");
    let event_pump = sdl_context
        .event_pump()
        .expect("Failed to get SDL2 event pump");

    (canvas, event_pump)
}

/// Clear the current draw buffer
pub fn clear_frame(renderer: &mut Canvas<Window>, bg_color: Color) {
    renderer.set_draw_color(bg_color);
    renderer.clear();
}

/// Render a single cell from the grid with an inside border
///
/// Translates from game space to pixel space
pub fn display_cell_bordered(
    renderer: &mut Canvas<Window>,
    row: u32,
    col: u32,
    cell: Color,
    cell_width: u32,
    border_width: u32,
) {
    let cell_height = cell_width; // All cells are square
    let x = cell_width * col + border_width;
    let y = cell_width * row + border_width;

    renderer.set_draw_color(cell);
    renderer
        .fill_rect(Rect::new(
            x as i32,
            y as i32,
            cell_width - (border_width * 2),
            cell_height - (border_width * 2),
        ))
        .unwrap_or_else(|e| println!("{}", e));
}

/// Render a single cell from the grid
///
/// Translates from game space to pixel space
pub fn display_cell(
    renderer: &mut Canvas<Window>,
    row: u32,
    col: u32,
    cell: Color,
    cell_width: u32,
) {
    let cell_height = cell_width; // All cells are square
    let x = cell_width * col;
    let y = cell_width * row;

    renderer.set_draw_color(cell);
    if let Err(e) = renderer.fill_rect(Rect::new(x as i32, y as i32, cell_width, cell_height)) {
        println!("{}", e)
    }
}

/// Render a `Grid` on the current draw buffer
pub fn render_frame(renderer: &mut Canvas<Window>, grid: &[Vec<Color>], cell_width: u32) {
    clear_frame(renderer, Color::RGB(0, 0, 0));

    for row in 0..grid.len() as u32 {
        for col in 0..grid[0].len() as u32 {
            let cell = grid[row as usize][col as usize];
            display_cell(renderer, row, col, cell, cell_width);
        }
    }
}

/// Move the draw buffer to the display (ie swap back buffer to front)
pub fn display_frame(renderer: &mut Canvas<Window>) {
    renderer.present();
}

/// Initialize a TrueType Font
// lifetime specifiers from https://users.rust-lang.org/t/rust-sdl2-does-not-live-long-enought-fighting-the-borrow-checher/9464/8
pub fn init_font<'a, 'b>(
    ttf_context: &'a ttf::Sdl2TtfContext,
    path: &str,
    size: u16,
) -> ttf::Font<'a, 'b> {
    ttf_context.load_font(path, size).unwrap()
}

/// Display a text `&str` centered at `(text_x, text_y)`
pub fn render_text(
    font: &ttf::Font,
    renderer: &mut Canvas<Window>,
    text_color: Color,
    (text_x, text_y): (i32, i32),
    text: &str,
) -> (u32, u32) {
    let surface = font.render(text).blended(text_color).unwrap();
    let width = surface.width();
    let height = surface.height();

    let text_center = (text_x as i32, text_y as i32);

    let texture_creator = renderer.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(surface)
        .unwrap();

    renderer
        .copy(
            &texture,
            None,
            Rect::from_center(text_center, width, height),
        )
        .unwrap();

    (width, height)
}

/// Render a `Menu`
pub fn render_menu<T>(renderer: &mut Canvas<Window>, font: &ttf::Font, menu: &menu::Menu<T>)
where
    T: Clone + PartialEq + std::fmt::Debug,
{
    clear_frame(renderer, Color::RGB(0, 0, 0));

    // render each menu item
    let (mut x, mut y) = renderer.window().size();
    x /= 2;
    y /= 4;
    let vertical_step = font.height() as u32;

    for (i, item) in menu.menu_items.iter().enumerate() {
        let (selected, color) = if i == menu.selection() {
            // selected color
            (true, Color::RGB(255, 255, 255))
        } else {
            // base color
            (false, Color::RGB(0, 0, 0))
        };
        render_menu_item(renderer, font, item, selected, color, x as i32, y as i32);
        y += vertical_step;
    }
}

/// Render a `MenuItem`
pub fn render_menu_item<T>(
    renderer: &mut Canvas<Window>,
    font: &ttf::Font,
    item: &menu::MenuItem<T>,
    selected: bool,
    color: Color,
    x: i32,
    y: i32,
) where
    T: Clone + PartialEq + std::fmt::Debug,
{
    let (color, text) = if selected {
        (color, format!("> {}", item.label))
    } else {
        (color, item.label.to_string())
    };

    render_text(font, renderer, color, (x, y), &text);
}
