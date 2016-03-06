extern crate rand;
use structures::*;
use self::rand::Rng;
use clients::Client;

pub struct SimpleClient {
    _player: u8
}

impl Client for SimpleClient {
    fn run(&self, _b: &Board, _l: &Links, _player: u8) -> Move {
        let x = rand::thread_rng().gen_range(0, BOARD_WIDTH);
        let y = rand::thread_rng().gen_range(0, BOARD_WIDTH);
        Move { x: x, y: y }
    }
}

impl SimpleClient {
    pub fn new(player: u8) -> SimpleClient {
        SimpleClient {
            _player: player
        }
    }
}
