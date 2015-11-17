extern crate rand;
use structures::*;
use self::rand::Rng;

pub fn run(_b: &Board, _l: &Links) -> Move {
    let x = rand::thread_rng().gen_range(0, BOARD_WIDTH);
    let y = rand::thread_rng().gen_range(0, BOARD_WIDTH);
    Move { x: x, y: y }
}
