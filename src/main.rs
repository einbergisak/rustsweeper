use std::process::exit;

use crate::game::GameContainer;
extern crate dialoguer;
extern crate piston_window;
extern crate rand;

mod game;
mod tests;

use piston_window::*;
use rand::{distributions::Alphanumeric, Rng};

const MAX_ROWS: usize = 75;
const MAX_COLS: usize = 135;

fn main() {
    let (game_cols, game_rows, game_mines, game_seed): (usize, usize, usize, String);
    let default_settings = [
        "Beginner (9x9)",
        "Intermediate (16x16)",
        "Expert (30x16)",
        "Custom",
    ];

    'outer: loop {
        match dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .items(&default_settings)
            .default(0)
            .with_prompt("\nPlease select one of game settings below using the arrow keys.\nConfirm with Enter, exit with Q.")
            .interact_opt()
        {
            Ok(option) => {
                if let Some(alt) = option {
                    match alt {
                        0 => {
                            game_cols = 9;
                            game_rows = 9;
                            game_mines = 10;
                        }
                        1 => {
                            game_cols = 16;
                            game_rows = 16;
                            game_mines = 40;
                        }
                        2 => {
                            game_cols = 30;
                            game_rows = 16;
                            game_mines = 99;
                        }
                        3 => 'inner: loop {
                            let input: String = dialoguer::Input::new()
                        .with_prompt(
                            format!("\nPlease enter preferred game settings in the format 'columns rows mines' (max {}x{})", MAX_COLS, MAX_ROWS,
                        ))
                        .interact()
                        .unwrap();
                            let mut iter = input.split_whitespace();
                            match (iter.next(), iter.next(), iter.next()) {
                                (Some(cols), Some(rows), Some(mines)) => {
                                    match (
                                        cols.parse::<usize>(),
                                        rows.parse::<usize>(),
                                        mines.parse::<usize>(),
                                    ) {
                                        (Ok(cols), Ok(rows), Ok(mines)) => {
                                            if rows > MAX_ROWS || cols > MAX_COLS {
                                                println!("\nFaulty input. The game size cannot exceed {} columns (width) or {} rows (height). Please enter another game size.", MAX_COLS, MAX_ROWS);
                                                continue 'inner;
                                            } else if mines > rows * cols / 2 {
                                                println!("\nFaulty input. The amount of mines cannot exceed 50% of the number of total game tiles (rows*columns).")
                                            }
                                            game_cols = cols;
                                            game_rows = rows;
                                            game_mines = mines;
                                            break 'inner;
                                        }
                                        (_, _, _) => {
                                            println!("\nFaulty input. Please make sure that you enter width, height and mines as integers.");
                                            continue 'inner;
                                        }
                                    }
                                }
                                (_, _, _) => {
                                    println!("\nFaulty input format. Please try again.");
                                    continue 'inner;
                                }
                            }
                        },
                        _ => {
                            println!(
                                "You need to choose one of the options. Please try again or press Q to exit."
                            );
                            continue 'outer;
                        }
                    }
                    game_seed = dialoguer::Input::new()
                        .with_prompt(
                            "\nPlease enter preferred game seed (press Enter for random)",
                        )
                        .default(format!("{}", rand::thread_rng().sample_iter(Alphanumeric).take(20).map(char::from).collect::<String>()))
                        .show_default(false)
                        .interact()
                        .unwrap();

                    break 'outer;
                } else {
                    exit(0);
                }
            }
            Err(_) => {
                println!(
                    "You need to choose one of the options. Please try again or press Q to exit."
                );
                continue;
            }
        }
    }

    let mut game = GameContainer::new(game_rows, game_cols, game_mines, game_seed);
    let tile_size = f64::min(40.0, 1800.0 / game_cols as f64).min(1000.0 / game_rows as f64);
    println!("Game initialized successfully.");

    let mut window: PistonWindow = WindowSettings::new(
        "Rustsweeper",
        [game_cols as f64 * tile_size, game_rows as f64 * tile_size],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.2; 4], graphics);
            for n in 0..game_rows {
                rectangle(
                    [1.0, 0.0, 0.0, n as f32 / game_rows as f32],
                    [0.0, n as f64 * tile_size, tile_size, tile_size],
                    context.transform,
                    graphics,
                );
            }
            for n in 0..game_cols {
                rectangle(
                    [1.0, 0.0, n as f32 / game_cols as f32, 1.0],
                    [n as f64 * tile_size, 0.0, tile_size, tile_size],
                    context.transform,
                    graphics,
                );
            }
        });
    }
}
