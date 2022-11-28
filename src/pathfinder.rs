use serde::{Deserialize, Serialize};

use crate::constants::*;
use crate::snake_game::SnakeGame;

#[derive(Serialize, Deserialize, PartialEq, Copy, Clone)]
pub enum Pathfinder {
    // Keeps going in one direction until aligned with the collectible. Then turn towards it.
    LazyWalker,
    // Keep switching planes in the most direct way towards the collectible.
    StepWalker,
}

pub fn find_path(snake_game: &SnakeGame) -> Direction {
    match snake_game.pathfinder {
        Pathfinder::LazyWalker => lazy_walker_fn(snake_game),
        Pathfinder::StepWalker => step_walker_fn(snake_game),
    }
}

fn lazy_walker_fn(snake_game: &SnakeGame) -> Direction {
    let plane_of_direction = plane_of_direction(&snake_game.player_state.player_direction);
    let player_x_pos = snake_game.player_state.player_x_pos();
    let player_y_pos = snake_game.player_state.player_y_pos();

    if player_x_pos != snake_game.char_x_pos && plane_of_direction == Plane::Horizontal {
        // If we aren't aligned with the collectible horizontally while traversing the horizontal
        // plane, just keep going.
        return snake_game.player_state.player_direction;
    }

    if player_y_pos != snake_game.char_y_pos && plane_of_direction == Plane::Vertical {
        // If we aren't aligned with the collectible vertically while traversing the vertical plane,
        // just keep going.
        return snake_game.player_state.player_direction;
    }

    // At this point, we know that we need to change direction.
    return if player_x_pos == snake_game.char_x_pos {
        // Horizontally aligned, so we need to either go north or south.
        if player_y_pos < snake_game.char_y_pos {
            Direction::South
        } else {
            Direction::North
        }
    } else {
        // Vertically aligned, so we need to either go west or east.
        if player_x_pos < snake_game.char_x_pos {
            Direction::East
        } else {
            Direction::West
        }
    };
}

fn step_walker_fn(snake_game: &SnakeGame) -> Direction {
    let player_x_pos = snake_game.player_state.player_x_pos();
    let player_y_pos = snake_game.player_state.player_y_pos();
    let char_x_pos = snake_game.char_x_pos;
    let char_y_pos = snake_game.char_y_pos;

    return if player_x_pos == char_x_pos {
        // The player is aligned horizontally, so just pick the correct vertical direction.
        get_optimal_direction(
            player_y_pos,
            char_y_pos,
            snake_game.snake_config.vertical_slots,
            Plane::Vertical,
        )
    } else if player_y_pos == char_y_pos {
        // The player is aligned vertically, so just pick the correct horizontal direction.
        get_optimal_direction(
            player_x_pos,
            char_x_pos,
            snake_game.snake_config.horizontal_slots,
            Plane::Horizontal,
        )
    } else {
        // Not aligned at all, so there are two valid directions to take at this point. We don't
        // want to continue on the current direction, so we switch based on that.
        let plane = plane_of_direction(&snake_game.player_state.player_direction);
        match plane {
            Plane::Vertical => get_optimal_direction(
                player_x_pos,
                char_x_pos,
                snake_game.snake_config.horizontal_slots,
                Plane::Horizontal,
            ),
            Plane::Horizontal => get_optimal_direction(
                player_y_pos,
                char_y_pos,
                snake_game.snake_config.vertical_slots,
                Plane::Vertical,
            ),
        }
    };
}

/// Decides, whether traversal over the edge of the screen is quicker than line-of-sight direction.
fn get_optimal_direction(start: i32, target: i32, total: i32, plane: Plane) -> Direction {
    let distance = start - target;

    let (line_of_sight_direction, over_edge_direction) = match plane {
        Plane::Vertical => {
            if distance < 0 {
                (Direction::South, Direction::North)
            } else {
                (Direction::North, Direction::South)
            }
        }
        Plane::Horizontal => {
            if distance < 0 {
                (Direction::East, Direction::West)
            } else {
                (Direction::West, Direction::East)
            }
        }
    };

    if (total - distance.abs()) > (total / 2) {
        line_of_sight_direction
    } else {
        over_edge_direction
    }
}
