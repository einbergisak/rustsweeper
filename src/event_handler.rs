use ggez::{
    event::EventHandler,
    graphics::{self, Color, DrawParam, Rect},
    nalgebra::Point2,
};
use ggez::{
    graphics::{DrawMode, FillOptions},
    nalgebra::Vector2,
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
        match button {
            ggez::event::MouseButton::Left => {
                // Reveal
                todo!()
            }
            ggez::event::MouseButton::Right => {
                // Flag!
                todo!()
            }
            ggez::event::MouseButton::Middle => {
                // Chord implementation!
                todo!()
            }
            _ => {}
        }
    }

    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, Color::from_rgb(0, 0, 0));

        // Scales the size of the tiles depending on game settings, this in order to make sure that the game fits on the screen.
        let scaled_tile_size = f32::min(DEFAULT_TILE_SIZE, 1800.0 / self.game_cols as f32)
            .min(1000.0 / self.game_rows as f32);
        let mut sprite_batch = self.sprite_batch.clone();
        for (x, vec) in self.get_tiles().into_iter().enumerate() {
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
                            scaled_tile_size / DEFAULT_TILE_SIZE,
                            scaled_tile_size / DEFAULT_TILE_SIZE,
                        ))
                        .dest(Point2::new(
                            x as f32 * scaled_tile_size,
                            y as f32 * scaled_tile_size,
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
