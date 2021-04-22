use std::process::exit;

use crate::game::GameContainer;
extern crate dialoguer;
extern crate rand;

mod event_handler;
mod game;

use colored::*;
use ggez::{
    conf::WindowMode,
    event,
    graphics::{self},
};
use rand::{distributions::Alphanumeric, Rng};

const DEFAULT_TILE_SIZE: f32 = 40.0;
const SPRITESHEET_WIDTH: f32 = 440.0;

fn main() {
    let (game_cols, game_rows, game_mines): (usize, usize, usize);
    let game_seed: Option<String>;
    let default_settings = [
        "Novice [9x9] (81 tiles, 12% mines)".color(Color::TrueColor {
            r: 20,
            g: 220,
            b: 0,
        }),
        "Versed [16x16] (256 tiles, 16% mines)".color(Color::TrueColor {
            r: 147,
            g: 230,
            b: 0,
        }),
        "Expert [30x16] (480 tiles, 21% mines)".color(Color::TrueColor {
            r: 212,
            g: 230,
            b: 0,
        }),
        "Master [40x24] (960 tiles, 26% mines)".color(Color::TrueColor {
            r: 255,
            g: 200,
            b: 0,
        }),
        "Savant [60x35] (2160 tiles, 32% mines)".color(Color::TrueColor {
            r: 255,
            g: 148,
            b: 0,
        }),
        "Legend [82x44] (3608 tiles, 37% mines)".color(Color::TrueColor {
            r: 255,
            g: 88,
            b: 0,
        }),
        "Unreal [120x50] (6000 tiles, 42% mines)".color(Color::TrueColor { r: 174, g: 0, b: 0 }),
    ];

    loop {
        match dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .items(&default_settings)
            .default(0)
            .with_prompt("\nPlease select one of game settings below using the arrow keys.\nConfirm with Enter, exit with Q.")
            .interact_opt()
        {
            Ok(option) => {
                if let Some(alt) = option {
                    let (cols, rows, mines) = match alt {
                        0 => {
                            (9, 9, 10)
                        }
                        1 => {
                            (16, 16, 40)
                        }
                        2 => {
                            (30, 16, 99)
                        }
                        3 => {
                            (40, 24, 250)
                        }
                        4 => {
                            (60, 35, 691)
                        }
                        5 => {
                            (82, 44, 1334)
                        }
                        6 => {
                            (120, 50, 2520)
                        }
                        _ => {
                            println!(
                                "You need to choose one of the options. Please try again or press Q to exit."
                            );
                            continue;
                        }
                    };
                    game_cols = cols;
                    game_rows = rows;
                    game_mines = mines;
                    break;
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

    loop {
        match dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .items(&["Random", "Seeded"])
            .default(0)
            .with_prompt("Do you want to play a random or seeded game?")
            .interact_opt()
        {
            Ok(option) => {
                if let Some(alt) = option {
                    match alt {
                        0 => {
                            game_seed = None;
                            break;
                        }
                        1 => {
                            let random_seed = format!(
                                "{}",
                                rand::thread_rng()
                                    .sample_iter(Alphanumeric)
                                    .take(20)
                                    .map(char::from)
                                    .collect::<String>()
                            );
                            println!("WARNING: Due to the nature of seeded generation, the first click is not guaranteed to be safe. To still make it playable, the game makes sure that the top left corner is safe on a seeded game.");
                            game_seed = Some(
                                dialoguer::Input::new()
                                    .with_prompt("Please enter game seed (press Enter for random)")
                                    .default(random_seed)
                                    .show_default(false)
                                    .interact()
                                    .unwrap(),
                            );
                            break;
                        }
                        _ => {
                            println!(
                                "You need to choose one of the options. Please try again or press Q to exit."
                            );
                            continue;
                        }
                    }
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
