#![allow(dead_code)]
extern crate pbr;
extern crate time;
extern crate rand;
extern crate rustc_serialize;
mod structures;
mod clients;
mod trainer;
use trainer::*;

use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut rng = rand::thread_rng();
    let mut trainer = TrainerConfig::new().build_new_trainer(&mut rng);
    let mut generation_index = 0;
    loop {
        println!(" ");
        println!(" ");
        println!("Running generation #{}", generation_index);
        let best_nn = trainer.step(&mut rng);


        let mut buffer = File::create("best.nn").unwrap();
        buffer.write_fmt(format_args!("{}", best_nn.encode().unwrap())).unwrap();

        generation_index += 1;
    }
}
