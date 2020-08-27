use rand::seq::SliceRandom;
use rand::thread_rng;
use opener::open;

fn main() {
    let talks: Vec<&str> = include_str!("data/lightning_talks").split("\n").collect();
    let talk = talks.choose(&mut thread_rng()).unwrap();
    opener::open(talk).expect("Could not open link");
}
