#[macro_use]
extern crate bitflags;

mod textures;
mod block_grid;
mod values;
mod board;
mod human_player;
mod robot_player;
mod board_renderer;
mod wrapper_types;

extern crate piston_window;
extern crate uuid;
extern crate graphics;
extern crate find_folder;
extern crate gfx;
extern crate gfx_texture;
extern crate sdl2_window;

use wrapper_types::GameWindow;
use piston_window::*;
use std::rc::*;

use textures::Textures;
use values::*;
use board::*;
use human_player::*;
use robot_player::*;
use board_renderer::*;

fn main() {
    // TODO: Get width + height from board
    let dimensions = Dimension::new(6, 13);
    let margin = 16.0;
    let gutter = 64.0;
    let cell = 32.0;

    let board_width = cell * (dimensions.w() + 1) as f64 + margin;
    let left_x = gutter;
    let right_x = gutter + board_width + gutter;
    let total_width = right_x + board_width + gutter;
    let total_height = gutter * 2.0 + cell * dimensions.h() as f64;

    let window: GameWindow =
        WindowSettings::new("Puzzle Fighter Turbo II", (total_width as u32, total_height as u32))
        .exit_on_esc(true)
        .build()
        .unwrap();

    let _ = window.window.borrow_mut().init_joysticks();

    let mut left_board = Board::new(dimensions);
    let mut right_board = Board::new(dimensions);

    let textures = Rc::new(Textures::new(&window));
    let mut left_board_renderer = BoardRenderer::new(
        textures.clone(),
        PixelPosition::new(left_x, gutter),
        dimensions
        );
    let mut right_board_renderer = BoardRenderer::new(
        textures.clone(),
        PixelPosition::new(right_x, gutter),
        dimensions
        );

    //let mut left_player = HumanPlayer::new(true);
    let mut left_player = RobotPlayer::new();
    let mut right_player = HumanPlayer::new(false);

    let mut left_render_state = RenderState::new();
    let mut right_render_state = RenderState::new();

    for e in window {
        e.draw_2d(|_c, g| {
            use graphics::*;

            // Black background
            clear([0.0, 0.0, 0.0, 1.0], g);
        });

        left_player.update(&e, &mut left_board);
        right_player.update(&e, &mut right_board);

        e.update(|args| {
            left_board.update(args.dt, &mut right_board, &left_render_state);
            right_board.update(args.dt, &mut left_board, &right_render_state);
        });

        // TODO: This return code pattern sucks
        match right_board_renderer.render(&e, &mut right_board) {
            Some(state) => { right_render_state = state },
            None => {}
        }
        // TODO: This return code pattern sucks
        match left_board_renderer.render(&e, &mut left_board) {
            Some(state) => { left_render_state = state },
            None => {}
        }

        // TODO: lol do clipping properly
        e.draw_2d(|c, g| {
            Rectangle::new([0.0, 0.0, 0.0, 1.0])
                .draw([0.0, 0.0, total_width as f64, gutter as f64],
                      &c.draw_state, c.transform, g);
        });
    }
}
