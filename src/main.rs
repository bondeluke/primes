use std::env;
use std::time::Instant;
use crate::prime_iterator::{stream};
use num_format::{Locale, ToFormattedString};

mod prime_iterator;
mod wheel;

fn main() {
    let args: Vec<String> = env::args().collect();

    let count: usize = match args.len() {
        1 => 1_000_000,
        _ => args[1].parse::<usize>().unwrap()
    };

    let start_time = Instant::now();
    let mut prime_iterator = stream();
    for _ in 1..count {
        prime_iterator.next();
    }
    let prime = prime_iterator.next().unwrap();
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);

    // The 50,000,000th prime number is 982,451,653
    println!("The {}{} prime number is {}", count.to_formatted_string(&Locale::en), post_fix(count), prime.to_formatted_string(&Locale::en));
    println!("Duration: {:01}.{:03}s", elapsed_time.as_secs(), elapsed_time.subsec_millis());
}

fn post_fix(n: usize) -> &'static str {
    match n % 10 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th"
    }
}