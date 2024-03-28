# Prime Generation
This repository is dedicated to finding prime numbers very quickly.
- Finding 1,000,000 primes in less than a few seconds ✅
  - Inspired by [Prime Streaming (PG-13)](https://www.codewars.com/kata/5519a584a73e70fa570005f5)
  -
    ```ps
    PS C:\Users\Luke\projects\primes> .\target\release\generate_primes.exe 1000000
    The 1,000,000th prime number is 15,485,863
    Duration: 0.038s
    ```
- Finding 50,000,000 primes in less than a few seconds ✅
  - Inspired by [Prime Streaming (NC-17)](https://www.codewars.com/kata/5519a584a73e70fa570005f5)
  -
    ```ps
    PS C:\Users\Luke\projects\primes> .\target\release\generate_primes.exe 50000000
    The 50,000,000th prime number is 982,451,653
    Duration: 0.567s
    ```
- Finding 100,000,000,000 primes in less than 3 minutes...🔲
  - Inspired by [Find First 100 Billion Primes](https://www.reddit.com/r/learnprogramming/comments/du8bii/find_first_100_billion_primes/)
- Finding 100,000,000,000 primes in less than 1 minutes... 🔲 
  - Inspired by [Find First 100 Billion Primes](https://www.reddit.com/r/learnprogramming/comments/du8bii/find_first_100_billion_primes/)

# Techniques
One of the quickest and most well-known strategies for finding prime numbers is the Sieve of Eratosthenes. I take this basic concept, and apply sieve segmentation, wheel factorization, and parallelization on top if it.
## Sieve of Eratosthenes
## Segemented Sieve
## Wheel Factorazation
## Parallelization
```rust
    fn extend_in_parallel(&mut self) {
        //println!("Expanding primes in range {} - {}...", self.segment * self.wheel.circumference(), (self.segment + THREAD_COUNT) * self.wheel.circumference());

        const THREAD_COUNT: usize = 128;
        (0..THREAD_COUNT)
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

        self.segment += THREAD_COUNT;
    }
```
