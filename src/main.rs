#![allow(dead_code)]
extern crate pbr;
extern crate time;
extern crate rand;
extern crate rustc_serialize;
mod structures;
mod clients;
mod trainer;
use trainer::*;

use std::fs::File;
use std::io::Write;

fn main() {
    let mut rng = rand::thread_rng();
    let mut trainer = TrainerConfig::new().build_new_trainer(&mut rng);
    let mut generation_index = 0;
    loop {
        println!(" ");
        println!(" ");
        println!("Running generation #{}", generation_index);
        trainer.step(&mut rng);

        // let f = File::create("test.txt").unwrap();
        // f.write_all(&best_nn.unwrap().to_string());

        generation_index += 1;
    }
}
