# Prime Generation
This repository is dedicated to finding prime numbers very quickly.

## Find 1 million primes in less than 3 seconds ✅
Inspired by [Prime Streaming (PG-13)](https://www.codewars.com/kata/prime-streaming-pg-13/)
```ps
PS C:\Users\Luke\projects\primes> .\target\release\generate_primes.exe 1000000
The 1,000,000th prime number is 15,485,863
Duration: 0.013s
```
## Find 50 million primes in less than 3 seconds ✅
Inspired by [Prime Streaming (NC-17)](https://www.codewars.com/kata/prime-streaming-nc-17/)
```ps
PS C:\Users\Luke\projects\primes> .\target\release\generate_primes.exe 50000000
The 50,000,000th prime number is 982,451,653
Duration: 0.567s
```
## Find 100 billion primes in less than 3 minutes ❌
Inspired by this Reddit post: [Find First 100 Billion Primes](https://www.reddit.com/r/learnprogramming/comments/du8bii/find_first_100_billion_primes/)
```ps
PS C:\Users\Luke\projects\primes> .\target\release\generate_primes.exe 100000000000
The 100,000,000,000th prime number is 2,510,361,116,537
Duration: 1575.457s
```

# Techniques
One of the quickest and most well-known strategies for finding prime numbers is the [Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes). I take this basic concept, and apply sieve segmentation, [wheel factorization](https://en.wikipedia.org/wiki/Wheel_factorization), and parallelization on top if it.
## Sieve of Eratosthenes
In progress...
## Segemented Sieve
In progress...
## Wheel Factorization
In progress...
## Parallelization
```rust
fn extend_in_parallel(&mut self) {
    (0..self.thread_count)
        .map(|i| {
            let segment = self.segment;
            let sieve = self.sieve.clone();
            let wheel = Arc::clone(&self.wheel);
            let primes = Arc::clone(&self.arc_primes);
            let next_spoke = Arc::clone(&self.next_spoke);
            thread::spawn(move || {
                PrimeIterator::extend(segment + i, sieve, wheel, primes, next_spoke)
            })
        })
        .collect::<Vec<JoinHandle<Vec<usize>>>>()
        .into_iter()
        .for_each(|handle| {
            self.primes.extend(handle.join().unwrap())
        });

    self.segment += self.thread_count; 
}
```
