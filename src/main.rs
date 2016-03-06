extern crate time;
extern crate rand;
extern crate rustc_serialize;
mod structures;
mod clients;
mod trainer;
use trainer::*;

fn main() {
    let mut rng = rand::thread_rng();
    let mut trainer = TrainerConfig::new().build_new_trainer(&mut rng);
    let mut generation_index = 0;
    loop {
        println!("Starting generation #{}", generation_index);
        trainer.step(&mut rng);
        generation_index += 1;
    }
}
