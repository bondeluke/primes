use std::time::Instant;
use crate::prime_iterator::{stream};
mod prime_iterator;
mod wheel;

fn main() {
    let start_time = Instant::now();
    let mut prime_iterator = stream();
    // The 50 millionth prime is 982451653
    for _ in 1..50_000_000 {
        prime_iterator.next();
    }
    println!("{}", prime_iterator.next().unwrap());
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("Duration: {:01}.{:03}s", elapsed_time.as_secs(), elapsed_time.subsec_millis());
}
