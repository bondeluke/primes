use std::env;
use std::time::Instant;
use crate::prime_iterator::{stream};
use num_format::{Locale, ToFormattedString};

mod prime_iterator;
mod wheel;

fn main() {
    let args: Vec<String> = env::args().collect();

    let count: usize = match args.len() > 1 {
        true => args[1].parse::<usize>().unwrap(),
        false => 50_000_000,
    };

    let thread_count: usize = match args.len() > 2 {
        true => args[2].parse::<usize>().unwrap(),
        false => if count > 1000000 { 128 } else { 32 },
    };

    let start_time = Instant::now();
    let mut prime_iterator = stream(thread_count);
    for _ in 0..count {
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