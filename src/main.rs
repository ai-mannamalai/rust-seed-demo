use rand::{Rng, SeedableRng, rngs::StdRng};
use rand::prelude::*;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author,version,about,long_about=None)]
struct Args {
    #[arg(short,long)]
    seed: u64,
}
   
fn main() {
    let args = Args::parse();
    
    // Create a PRNG with a fixed seed for reproducible results
    let seed_value: u64 = args.seed;
    let mut rng = StdRng::seed_from_u64(seed_value);

    // Generate random numbers
    let random_u32: u32 = rng.random_range(0..100);
    let random_bool: bool = rng.random::<bool>();

    println!("Random u32: {}", random_u32);
    println!("Random bool: {}", random_bool);
}
