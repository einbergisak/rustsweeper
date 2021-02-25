use std::process::exit;

use crate::game::GameContainer;
extern crate dialoguer;
extern crate rand;

mod event_handler;
mod game;

use ggez::{
    event::{self, winit_event::ElementState, EventHandler},
    input::mouse,
};
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

    let mut cb = ggez::ContextBuilder::new("game_name", "author_name");

    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        cb = cb.add_resource_path(path);
    }

    let (mut ctx, mut event_loop) = cb.build().unwrap();

    let mut game = GameContainer::new(&mut ctx, game_rows, game_cols, game_mines, game_seed);
    println!("Game initialized successfully.");

    while ctx.continuing {
        event_loop.poll_events(|event| {
            ctx.process_event(&event);
            match event {
                ggez::event::winit_event::Event::WindowEvent { event, .. } => match event {
                    ggez::event::winit_event::WindowEvent::CloseRequested => event::quit(&mut ctx),
                    ggez::event::winit_event::WindowEvent::MouseInput { state, button, .. } => {
                        let (x, y) = (mouse::position(&mut ctx).x, mouse::position(&mut ctx).y);
                        match state {
                            ElementState::Pressed => {
                                game.mouse_button_down_event(&mut ctx, button, x, y)
                            }
                            ElementState::Released => {
                                game.mouse_button_up_event(&mut ctx, button, x, y)
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        })
    }
}
