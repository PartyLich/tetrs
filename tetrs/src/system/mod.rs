use crate::{component::*, Error, GameState};
use cell_engine::gfx;
use ecs::ComponentRegistry;
use sdl2::{rect::Rect, render::Canvas, ttf, video::Window};

/// Draw entities on the gamefield
pub fn draw_entities(
    state: &mut GameState,
    canvas: &mut Canvas<Window>,
    cell_width: u32,
) -> Result<(), Error> {
    let ecs = &mut state.ecs;
    let registry = ecs.component_registry.lock().unwrap();

    for e in ecs.entity_list.iter() {
        if !ecs.entity_manager.alive(e) {
            continue;
        }

        let color = registry.get_component::<ColorComponent, _>(&e);
        let mesh = registry.get_component::<MeshComponent, _>(&e);
        let pos = registry.get_component::<PositionComponent, _>(&e);
        let preview = registry.get_component::<PreviewComponent, _>(&e);
        let hold = registry.get_component::<HoldComponent, _>(&e);
        // aspect check - if a component is missing, do nothing
        if preview.is_some() || hold.is_some() || color.is_none() || mesh.is_none() || pos.is_none()
        {
            continue;
        }

        let mesh = &mesh.unwrap().0;
        let color = color.unwrap();
        let pos = pos.unwrap();
        for Position { x, y } in mesh.iter() {
            let y = *y + pos.y;
            let x = *x + pos.x;
            gfx::display_cell_bordered(
                canvas,
                (y + crate::Y_OFFSET) as u32,
                (x + crate::X_OFFSET) as u32,
                *color,
                cell_width,
                1,
            );
            // collision cache
            state.grid[y as usize][x as usize] = *color;
        }
    }

    Ok(())
}

/// Draw the UI background
pub fn draw_ui_bg(renderer: &mut Canvas<Window>, cell_width: u32) -> Result<(), Error> {
    gfx::clear_frame(renderer, crate::UI_BG);

    renderer.set_draw_color(crate::BG_CELL);
    renderer
            .fill_rect(Rect::new(
                    (cell_width * crate::X_OFFSET as u32) as i32,
                    (cell_width * crate::Y_OFFSET as u32) as i32,
                    cell_width * crate::GLASS_WIDTH as u32,
                    cell_width * crate::GLASS_HEIGHT as u32,
                    ))?;

    Ok(())
}

/// Draw preview entities
pub fn draw_previews(
    state: &mut GameState,
    canvas: &mut Canvas<Window>,
    cell_width: u32,
) -> Result<(), Error> {
    let ecs = &mut state.ecs;
    let registry = ecs.component_registry.lock().unwrap();

    const PREVIEW_X: i32 = 3 + crate::X_OFFSET + crate::GLASS_WIDTH as i32;
    let mut y_offset = crate::Y_OFFSET;

    for e in state.next_pieces.iter() {
        if !ecs.entity_manager.alive(e) {
            continue;
        }

        let color = registry.get_component::<ColorComponent, _>(&e);
        let mesh = registry.get_component::<MeshComponent, _>(&e);
        let preview = registry.get_component::<PreviewComponent, _>(&e);
        // aspect check - if a component is missing, do nothing
        if preview.is_none() || color.is_none() || mesh.is_none() {
            continue;
        }

        let mesh = &mesh.unwrap().0;
        let color = color.unwrap();
        for Position { x, y } in mesh.iter() {
            let x = (x + PREVIEW_X) as u32;
            let y = (y + y_offset) as u32;
            gfx::display_cell_bordered(canvas, y, x, *color, cell_width, 1);
        }

        y_offset += 3;
    }

    Ok(())
}

/// Draw hold entities
pub fn draw_hold(
    state: &mut GameState,
    canvas: &mut Canvas<Window>,
    cell_width: u32,
) -> Result<(), Error> {
    let ecs = &mut state.ecs;
    let registry = ecs.component_registry.lock().unwrap();

    let x_offset: i32 = (crate::X_OFFSET) / 2;
    let y_offset = crate::Y_OFFSET * 3;

    let e = state.hold_piece.and_then(|e| {
        if ecs.entity_manager.alive(&e) {
            Some(e)
        } else {
            // nothing to draw
            None
        }
    });
    if e.is_none() {
        return Ok(());
    }
    let e = e.unwrap();

    let color = registry.get_component::<ColorComponent, _>(&e);
    let mesh = registry.get_component::<MeshComponent, _>(&e);
    let hold = registry.get_component::<HoldComponent, _>(&e);
    // aspect check - if a component is missing, do nothing
    if hold.is_none() || color.is_none() || mesh.is_none() {
        return Ok(());
    }

    let mesh = &mesh.unwrap().0;
    let color = color.unwrap();
    for Position { x, y } in mesh.iter() {
        let x = (x + x_offset) as u32;
        let y = (y + y_offset) as u32;
        gfx::display_cell_bordered(canvas, y, x, *color, cell_width, 1);
    }

    Ok(())
}

/// Draw user interface (outside of game field)
pub fn draw_ui(
    ecs: &mut GameState,
    renderer: &mut Canvas<Window>,
    cell_width: u32,
    font: &ttf::Font,
) -> Result<(), Error> {
    let v_step = font.height() as i32;
    let line_txt = &format!("Lines: {}", crate::get_lines(ecs).unwrap());
    let level_txt = &format!("Level: {}", crate::get_level(ecs).unwrap() + 1);
    let score_txt = &format!("Score: {}", crate::get_score(ecs).unwrap());

    let x = (crate::X_OFFSET * cell_width as i32) / 2;
    let mut y = crate::Y_OFFSET * cell_width as i32 + (font.height() / 2);
    gfx::render_text(font, renderer, crate::PINK, (x, y), level_txt);

    y += v_step;
    gfx::render_text(font, renderer, crate::BLUE, (x, y), line_txt);

    y += v_step;
    gfx::render_text(font, renderer, crate::TEXT_COLOR, (x, y), score_txt);

    y += v_step;
    gfx::render_text(font, renderer, crate::PINK, (x, y), "Hold Piece: ");

    Ok(())
}
