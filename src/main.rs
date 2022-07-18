use goban::rules::{game::Game, Move, CHINESE};
use rand::prelude::IteratorRandom;

fn main() {
    let mut gb = Game::builder()
        .size((19, 19))
        .rule(CHINESE)
        .build()
        .unwrap();

    let mut i = 35;
    while !gb.is_over() && i > 0 {
        gb.play(
            gb.legals()
                .choose(&mut rand::thread_rng())
                .map(|p| Move::Play(p.0, p.1))
                .unwrap(),
        );

        gb.display_goban();

        i -= 1;
    }
}
