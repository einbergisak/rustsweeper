use ggez::nalgebra::Vector2;
use ggez::{
    event::EventHandler,
    graphics::{self, Color, DrawParam, Rect},
    nalgebra::Point2,
};

use crate::{game::GameContainer, DEFAULT_TILE_SIZE, SPRITESHEET_WIDTH};

const TILE_IMAGE_FRACTION: f32 = DEFAULT_TILE_SIZE / SPRITESHEET_WIDTH;

impl EventHandler for GameContainer {
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) {
        let tile_x = (x / self.scaled_tile_size).floor() as usize;
        let tile_y = (y / self.scaled_tile_size).floor() as usize;
        match button {
            ggez::event::MouseButton::Left => {
                // Reveal
                self.reveal_tile_at((tile_x, tile_y));
            }
            ggez::event::MouseButton::Right => {
                // Flag
                self.toggle_flag_at((tile_x, tile_y));
            }
            ggez::event::MouseButton::Middle => {
                // Chord
                self.chord_at((tile_x, tile_y));
            }
            _ => {}
        }
    }

    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, Color::from_rgb(0, 0, 0));

        let mut sprite_batch = self.sprite_batch.clone();
        let tile_array = &*self.tile_array;
        for (x, vec) in tile_array.into_iter().enumerate() {
            for (y, tile) in vec.into_iter().enumerate() {
                // Sets the "source" of the image for each tile, which is a part of the /resources/spritesheet.bmp image which is loaded into the sprite batch.
                let src = if tile.is_flagged {
                    Rect::new(2.0 * TILE_IMAGE_FRACTION, 0.0, TILE_IMAGE_FRACTION, 1.0)
                } else {
                    match (tile.is_revealed, tile.number) {
                        (false, _) => Rect::new(TILE_IMAGE_FRACTION, 0.0, TILE_IMAGE_FRACTION, 1.0),

                        (true, Some(number)) => Rect::new(
                            TILE_IMAGE_FRACTION * (2.0 + number as f32),
                            0.0,
                            TILE_IMAGE_FRACTION,
                            1.0,
                        ),
                        (true, None) => Rect::new(0.0, 0.0, TILE_IMAGE_FRACTION, 1.0),
                    }
                };

                sprite_batch.add(
                    DrawParam::new()
                        .src(src)
                        .scale(Vector2::new(
                            self.scaled_tile_size / DEFAULT_TILE_SIZE,
                            self.scaled_tile_size / DEFAULT_TILE_SIZE,
                        ))
                        .dest(Point2::new(
                            x as f32 * self.scaled_tile_size,
                            y as f32 * self.scaled_tile_size,
                        )),
                );
            }
        }

        graphics::draw(ctx, &sprite_batch, (Point2::<f32>::new(0.0, 0.0),))
            .expect("Something went wrong rendering the game.");
        // let a = ggez::graphics::Mesh::new_circle(
        //     ctx,
        //     DrawMode::Fill(FillOptions::default()),
        //     Point2::new(10.0, 10.0),
        //     5.0,
        //     1.0,
        //     graphics::WHITE,
        // )
        // .unwrap();

        // todo std::thread::yield_now();
        graphics::present(ctx)
    }
}
