use std::env;
use std::time::Instant;
use crate::prime_iterator::{stream};

mod prime_iterator;
mod wheel;

fn main() {
    let args: Vec<String> = env::args().collect();

    let count: usize = match args.len() {
        1 => 50_000_000_000,
        _ => args[1].parse::<usize>()
    };

    let start_time = Instant::now();
    let mut prime_iterator = stream();
    for _ in 1..count {
        prime_iterator.next();
    }
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);

    // The 50 millionth prime number is 982451653
    println!("The {}th prime number is {}", count, prime_iterator.next().unwrap());
    println!("Duration: {:01}.{:03}s", elapsed_time.as_secs(), elapsed_time.subsec_millis());
}
