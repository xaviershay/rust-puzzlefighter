extern crate puzzlefighter;

pub use self::puzzlefighter::*;
pub use self::puzzlefighter::robot_player::*;
pub use self::puzzlefighter::board_renderer::*;

extern crate rand;

use rand::*;
use rand::distributions::{IndependentSample, Range};

/*
use values::*;
use board::*;
use human_player::*;
use robot_player::*;
use board_renderer::*;
*/

fn normalize((x, y): (f64, f64)) -> (f64, f64) {
    let len = (x*x + y*y).sqrt();

    (x / len, y / len)
}

fn main() {
    let dimensions = Dimension::new(6, 13);
    let dt = 0.3;

    let mut rng = rand::thread_rng();

    let mut population: Vec<_> = (1..100).map(|_| {
        normalize((
            thread_rng().gen_range(-1.0, 1.0), 
            thread_rng().gen_range(-1.0, 1.0),
        ))
    }).collect();

    for round in 1..10 {
        let mut winners = Vec::new();

        for i in 1..20 {
            let w1 = *rand::sample(&mut rng, population.iter(), 1)[0];
            let w2 = *rand::sample(&mut rng, population.iter(), 1)[0];
            let mut left_wins = 0;
            let mut right_wins = 0;

            for i in 1..19 {
                let mut left_board = Board::new(dimensions);
                let mut right_board = Board::new(dimensions);
                let mut left_player = RobotPlayer::new(w1);
                let mut right_player = RobotPlayer::new(w2);
                let mut render_state = RenderState::new();

                loop {
                    left_player.update(dt, &mut left_board);
                    right_player.update(dt, &mut right_board);
                    left_board.update(dt, &mut right_board, &render_state);
                    right_board.update(dt, &mut left_board, &render_state);

                    if left_board.full() {
                        right_wins += 1;
                        break;
                    } else if right_board.full() {
                        left_wins += 1;
                        break;
                    }
                }
            }

            winners.push(if right_wins > left_wins {
                w2
            } else {
                w1
            });
            println!("({:.*}, {:.*}) {} - ({:.*}, {:.*}) {}",
                2, w1.0, 2, w1.1, left_wins,
                2, w2.0, 2, w2.1, right_wins);
        }


        println!("");
        println!("Winners");
        for x in winners.iter() {
            println!("({:.*}, {:.*})", 2, x.0, 2, x.1);
        }

        println!("");

        // Breed winners
        let offspring: Vec<_> = (1..100).map(|_| {
            let w1 = *rand::sample(&mut rng, winners.iter(), 1)[0];
            let w2 = *rand::sample(&mut rng, winners.iter(), 1)[0];

            normalize((
                w1.0 + w2.0,
                w1.1 + w2.1,
            ))
        }).collect();
        population = offspring;
    }

    //left_board.debug();
    //right_board.debug();
}
