#[macro_use]
extern crate bitflags;

mod textures;
mod block_grid;
mod values;
mod renderer;
mod board;
mod human_player;

extern crate piston_window;
extern crate uuid;
extern crate graphics;
extern crate find_folder;
extern crate gfx;
extern crate gfx_texture;

use piston_window::*;
use std::rc::*;

use textures::Textures;
use values::*;
use renderer::*;
use board::*;
use human_player::*;

//        if cfg!(feature = "animation_test") {
//            let blocks = vec!(
//                PositionedBlock::new(Block::new(Color::Red,   true), Position::new(0, 0)),
//                PositionedBlock::new(Block::new(Color::Green, false), Position::new(0, 1)),
//                PositionedBlock::new(Block::new(Color::Green, false), Position::new(0, 2)),
//                PositionedBlock::new(Block::new(Color::Green, false), Position::new(0, 3)),
//                PositionedBlock::new(Block::new(Color::Red,   false), Position::new(0, 4)),
//                PositionedBlock::new(Block::new(Color::Red,   false), Position::new(0, 5)),
//                PositionedBlock::new(Block::new(Color::Red,   false), Position::new(0, 6)),
//                PositionedBlock::new(Block::new(Color::Red,   false), Position::new(0, 7)),
//                PositionedBlock::new(Block::new(Color::Blue,  false), Position::new(0, 8)),
//                PositionedBlock::new(Block::new(Color::Green, true), Position::new(1, 0)),
//                PositionedBlock::new(Block::new(Color::Green, false), Position::new(1, 1)),
//                PositionedBlock::new(Block::new(Color::Red,   false), Position::new(1, 2)),
//                PositionedBlock::new(Block::new(Color::Blue,  false), Position::new(1, 3)),
//                PositionedBlock::new(Block::new(Color::Red,   false), Position::new(1, 4)),
//                PositionedBlock::new(Block::new(Color::Blue,  false), Position::new(1, 5)),
//            );
//            for block in blocks {
//                grid.set(block);
//                renderer.add_block(block);
//            }

fn main() {
    // TODO: Get width + height from board
    let margin = 16;
    let gutter = 64;
    let cell = 32;

    let board_width = cell * (6 + 1) + margin;
    let left_x = gutter;
    let right_x = gutter + board_width + gutter;
    let total_width = right_x + board_width + gutter;
    let total_height = gutter * 2 + cell * 13;

    let window: PistonWindow =
        WindowSettings::new("Puzzle Fighter Turbo II", (total_width, total_height))
        .exit_on_esc(true)
        .build()
        .unwrap();

    let textures = Textures::new(&window);
    let settings = Rc::new(GlRenderSettings::new(textures));

    let mut left_board = Board::new(settings.clone(),
                                    Dimension::new(6, 13),
                                    PixelPosition::new(left_x, gutter));

    let mut right_board = Board::new(settings.clone(),
                                    Dimension::new(6, 13),
                                    PixelPosition::new(right_x, gutter));

    let left_player = HumanPlayer::new(true);
    let right_player = HumanPlayer::new(false);

    for e in window {
        e.draw_2d(|c, g| {
            use graphics::*;

            // Black background
            clear([0.0, 0.0, 0.0, 1.0], g);
        });

        left_player.update(&e, &mut left_board);
        right_player.update(&e, &mut right_board);
        left_board.update(&e, &mut right_board);
        right_board.update(&e, &mut left_board);

        // TODO: lol do clipping properly
        e.draw_2d(|c, g| {
            Rectangle::new([0.0, 0.0, 0.0, 1.0])
                .draw([0.0, 0.0, total_width as f64, gutter as f64],
                      &c.draw_state, c.transform, g);
        });
    }
}
