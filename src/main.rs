use std::str::FromStr;

use crate::game::GameContainer;
extern crate dialoguer;
extern crate piston_window;
extern crate rand;

mod game;
mod tests;
use piston_window::*;

fn main() {
    // let mut start_window: PistonWindow = WindowSettings::new("Enter game settings.", [200,400]).exit_on_esc(true).build().unwrap();

    // if Confirm::new()
    //     .with_prompt("Do you want to play the game with the default 16x16 settings?")
    //     .interact()
    //     .expect("Prompt error")
    // {
    //     cols = 16;
    //     rows = 16;
    //     mines = 40;
    // } else {
    let (mut cols, mut rows, mut mines, mut seed): (usize, usize, usize, String);
    loop {
        let randseed = rand::random::<u32>();
        let input = dialoguer::Input::new()
        .with_prompt(
            "\nPlease enter preferred game settings in the format 'width height mines seed' (empty for default)",
        )
        .with_initial_text("")
        .default(format!("16 16 40 {}", randseed))
        .show_default(false)
        .interact_text()
        .unwrap();
        let mut iter = input.split_whitespace();
        match (iter.next(), iter.next(), iter.next(), iter.next()) {
            (Some(w), Some(h), Some(m), Some(s)) => {
                match (w.parse::<usize>(), h.parse::<usize>(), m.parse::<usize>()) {
                    (Ok(c), Ok(r), Ok(m)) => {
                        cols = c;
                        rows = r;
                        mines = m;
                        match String::from_str(&s) {
                            Ok(s) => seed = s,
                            Err(_) => {
                                println!("\nFaulty seed input. Please make sure that it is written exclusively with alphanumeric characters.");
                                continue;
                            }
                        }
                        break;
                    }
                    (_, _, _) => {
                        println!("\nFaulty input. Please make sure that you enter width, height and mines as numbers.");
                        continue;
                    }
                }
            }
            (_, _, _, _) => {
                println!("\nFaulty input format. Please try again.");
                continue;
            }
        }
    }

    let mut game = GameContainer::new(rows, cols, mines, seed);
    println!("Game initialized successfully.")
    // }

    // let mut main_window: PistonWindow =
    //     WindowSettings::new("Rustsweeper", [800, 800])
    //     .exit_on_esc(true).build().unwrap();

    // while let Some(event) = main_window.next() {
    //     main_window.draw_2d(&event, |context, graphics, _device| {
    //         clear([0.2; 4], graphics);
    //         rectangle([1.0, 0.0, 0.0, 1.0], // red
    //                   [0.0, 0.0, 100.0, 100.0],
    //                   context.transform,
    //                   graphics);
    //         rectangle([1.0, 1.0, 0.0, 1.0], // red
    //                   [0.0, 0.0, 50.0, 100.0],
    //                   context.transform,
    //                   graphics);
    //     });
    // }
}
