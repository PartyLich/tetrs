use crate::{component, GameState, TetrsResult};
use ecs::ComponentRegistry;

/// Calculate tetris score for a `line_count` cleared lines and current `level`
fn calc_score(line_count: usize, level: u32) -> u32 {
    if line_count > 4 {
        return 0;
    }
    const POINT_MAP: [u32; 5] = [0, 100, 300, 500, 800];

    POINT_MAP[line_count] * (level + 1)
}

/// Update the game score based on `line_count` lines cleared
pub fn update_score(
    state: &mut GameState,
    line_count: usize,
) -> TetrsResult<Option<crate::TimerEvent>> {
    let ecs = &mut state.ecs;
    let e = state.scoring.unwrap();
    let mut registry = ecs.component_registry.lock().unwrap();
    let scoring = registry
        .get_component_mut::<component::Scoring>(&e)
        .ok_or(crate::Error::MissingComponent("Scoring"))?;

    let mut result = None;
    let old_level = scoring.level;

    scoring.score += calc_score(line_count, scoring.level);
    scoring.lines += line_count as u32;
    scoring.level = scoring.lines / 10;

    // up the timer speed/gravity on level increase
    if old_level != scoring.level {
        result = Some(crate::TimerEvent::SetInterval(std::cmp::max(
            25,
            300 - ((scoring.level as u128 + 1) * 25),
        )));
    }
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calc_score_invalid() {
        {
            let expected = 0;
            let actual = calc_score(5, 0);
            assert_eq!(actual, expected);
        }
        {
            let expected = 0;
            let actual = calc_score(0, 0);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn calc_score_by_line() {
        {
            let expected = 100;
            let actual = calc_score(1, 0);
            assert_eq!(actual, expected);
        }
        {
            let expected = 300;
            let actual = calc_score(2, 0);
            assert_eq!(actual, expected);
        }
        {
            let expected = 500;
            let actual = calc_score(3, 0);
            assert_eq!(actual, expected);
        }
        {
            let expected = 800;
            let actual = calc_score(4, 0);
            assert_eq!(actual, expected);
        }
    }
}
