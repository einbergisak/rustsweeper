use std::process::exit;

use crate::game::GameContainer;
extern crate dialoguer;
extern crate rand;

mod event_handler;
mod game;

use ggez::{
    conf::WindowMode,
    event,
    graphics::{self},
};
use rand::{distributions::Alphanumeric, Rng};

const MAX_ROWS: usize = 50;
const MAX_COLS: usize = 100;
const DEFAULT_TILE_SIZE: f32 = 40.0;
const SPRITESHEET_WIDTH: f32 = 440.0;

fn main() {
    let (game_cols, game_rows, game_mines): (usize, usize, usize);
    let game_seed: Option<String>;
    let default_settings = [
        "Beginner [9x9] (81 tiles & 10 mines)",
        "Intermediate [16x16] (256 tiles & 40 mines)",
        "Expert [30x16] (480 tiles & 99 mines)",
        "Master [60x32] (1920 tiles & 400 mines)",
        "Legend [100x50] (5000 tiles & 1250 mines)",
        "Custom [seeded generation]",
    ];
    let random_seed = format!(
        "{}",
        rand::thread_rng()
            .sample_iter(Alphanumeric)
            .take(20)
            .map(char::from)
            .collect::<String>()
    );

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
                        3 => {
                            game_cols = 60;
                            game_rows = 32;
                            game_mines = 450;
                        }
                        4 => {
                            game_cols = 100;
                            game_rows = 50;
                            game_mines = 1250;
                        }
                        5 => 'inner: loop {
                            let input: String = dialoguer::Input::new()
                        .with_prompt(
                            format!("\nPlease enter preferred game settings in the format 'columns rows mines' (max {}x{})", MAX_COLS, MAX_ROWS,
                        ))
                        .interact()
                        .unwrap(); // TODO: Om man lämnar "mines" tomt så får man ett standardantal med minor
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
                                            game_seed = Some(dialoguer::Input::new()
                                                .with_prompt("\nPlease enter preferred game seed (press Enter for random)",)
                                                .default(random_seed)
                                                .show_default(false)
                                                .interact()
                                                .unwrap());
                                            break 'outer;
                                        }
                                        _ => {
                                            println!("\nFaulty input. Please make sure that you enter width, height and mines as integers.");
                                            continue 'inner;
                                        }
                                    }
                                }
                                _ => {
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
                    game_seed = None;
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

    // Scales the size of the tiles depending on game settings, this in order to make sure that the game fits on the screen.
    let scaled_tile_size = f32::min(DEFAULT_TILE_SIZE, 1800.0 / game_cols as f32)
        .min(1000.0 / game_rows as f32)
        .floor();

    let mut cb = ggez::ContextBuilder::new("Rustsweeper", "Isak Einberg").window_mode(
        WindowMode::default()
            .dimensions(
                game_cols as f32 * scaled_tile_size,
                game_rows as f32 * scaled_tile_size,
            )
            .resizable(false),
    );

    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        cb = cb.add_resource_path(path);
    } else {
        panic!("Could not find resource directory.")
    }

    let (mut ctx, mut event_loop) = cb.build().unwrap();

    // TODO! Set window icon
    graphics::set_window_title(&mut ctx, "Rustsweeper");

    let mut game = GameContainer::new(
        &mut ctx,
        game_rows,
        game_cols,
        game_mines,
        game_seed,
        scaled_tile_size,
    );
    println!("Game initialized successfully.");

    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => {
            println!("Game exited cleanly.");
        }
        Err(e) => println!("Error occured: {}", e),
    }
}
