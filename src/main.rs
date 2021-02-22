use crate::game::{GameContainer};
use std::{io::stdin, ops::Add};

extern crate piston_window;

mod tests;
mod game;
use piston_window::*;

fn main() {

    // let mut start_window: PistonWindow = WindowSettings::new("Enter game settings.")
    
    // while let Some(event) = window.next() {

    // }
    // let mut main_window: PistonWindow =
    //     WindowSettings::new("Hello Piston!", [800, 800])
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