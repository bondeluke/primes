use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use crate::wheel::{get_wheel, Wheel};

pub fn stream(thread_count: usize) -> impl Iterator<Item=usize> {
    PrimeIterator::new(thread_count)
}

struct PrimeIterator {
    sieve: Vec<bool>,
    next_spoke: Arc<Vec<usize>>,
    wheel: Arc<Wheel>,
    primes: Vec<usize>,
    kernel: Arc<Vec<usize>>,
    segment: usize,
    cursor: i32,
    thread_count: usize,
}

impl PrimeIterator {
    fn new(thread_count: usize) -> Self {
        let wheel = get_wheel(7);
        Self {
            sieve: PrimeIterator::initialize_sieve(&wheel),
            next_spoke: Arc::new(PrimeIterator::initialize_next_spoke(&wheel)),
            wheel: Arc::new(wheel),
            primes: Vec::with_capacity(42331), // 42,331 primes in kernel with basis_size == 7
            segment: 0,
            cursor: -1,
            kernel: Arc::new(vec![]),
            thread_count,
        }
    }

    fn initialize_sieve(wheel: &Wheel) -> Vec<bool> {
        let mut sieve = vec![false; wheel.circumference() / 3];
        for &spoke in &wheel.spokes {
            sieve[spoke / 3] = true;
        }
        sieve
    }

    fn initialize_next_spoke(wheel: &Wheel) -> Vec<usize> {
        let circ = wheel.circumference();

        let mut next_spoke = vec![0; circ];
        let mut spoke_index = 0;
        for i in 0..circ {
            let spoke = wheel.spokes[spoke_index];
            next_spoke[i] = spoke_index;
            if i == spoke {
                spoke_index += 1;
            }
        }
        next_spoke
    }

    fn initialize_kernel(&mut self) {
        let spokes = &self.wheel.spokes;
        let upper_limit = self.wheel.circumference();
        let basis_size = self.wheel.basis.len();

        // (0) Start with the basis and the first non-1 spoke
        self.primes.extend(&self.wheel.basis);
        self.primes.push(spokes[1]);

        // (1) Create a sieve to track spoke primality
        let mut sieve = self.sieve.clone();

        // (2) Cross out composites using spoke multiples
        let mut spoke_index = 2;
        for i in basis_size.. {
            let prime = self.primes[i];
            let p_squared = prime * prime;
            if p_squared > upper_limit { break; }

            for &spoke in &spokes[self.next_spoke[prime]..] {
                let n = prime * spoke;
                if n > upper_limit { break; }

                sieve[n / 3] = false;
            }

            while spokes[spoke_index] < p_squared {
                if sieve[spokes[spoke_index] / 3] {
                    self.primes.push(spokes[spoke_index]);
                }
                spoke_index += 1;
            }
        }

        // (3) Add remaining spokes that have not been crossed out
        for &spoke in &spokes[spoke_index..] {
            if sieve[spoke / 3] {
                self.primes.push(spoke);
            }
        }

        // println!("Last prime added to kernel was {}", self.primes[self.primes.len() - 1]);
        self.kernel = Arc::new(self.primes.clone());
        self.segment += 1; // Since the kernel is 0-th segment, we can move to the next segment
    }

    fn extend(
        segment: usize,
        mut sieve: Vec<bool>,
        wheel: Arc<Wheel>,
        primes: Arc<Vec<usize>>,
        next_spoke: Arc<Vec<usize>>,
    ) -> Vec<usize> {
        let spokes = &wheel.spokes;
        let circ = wheel.circumference();
        let lower_limit = segment * circ;
        let upper_limit = (segment + 1) * circ;
        let basis_size = wheel.basis.len();

        for &prime in &primes[basis_size..] {
            let p_squared = prime * prime;
            if p_squared > upper_limit { break; }

            let lowest_factor = lower_limit / prime + 1;
            let spoke_approx = lowest_factor % circ;
            let k = lowest_factor / circ;
            for &spoke in &spokes[next_spoke[spoke_approx]..] {
                let n = prime * (k * circ + spoke);
                if n > upper_limit { break; }
                sieve[(n - lower_limit) / 3] = false;
            }
        }

        let mut new_primes: Vec<usize> = Vec::with_capacity(spokes.len());

        for &spoke in spokes {
            if sieve[spoke / 3] {
                new_primes.push(lower_limit + spoke);
            }
        }

        new_primes
    }

    fn extend_in_parallel(&mut self) {
        // let circ = self.wheel.circumference();
        // let lower_limit = self.segment * circ;
        // let upper_limit = (self.segment + self.thread_count) * circ;
        // println!("Expanding primes in range {} - {}...", lower_limit, upper_limit);

        // Reset the primes for the new segments, drop the previous segments
        // From research, there will be at most 3845751 (and at least 2767611 primes)
        self.primes = Vec::with_capacity(3845751);
        self.cursor = 0;

        (0..self.thread_count)
            .map(|i| {
                let segment = self.segment;
                let sieve = self.sieve.clone();
                let wheel = Arc::clone(&self.wheel);
                let kernel = Arc::clone(&self.kernel);
                let next_spoke = Arc::clone(&self.next_spoke);
                thread::spawn(move || {
                    PrimeIterator::extend(segment + i, sieve, wheel, kernel, next_spoke)
                })
            })
            .collect::<Vec<JoinHandle<Vec<usize>>>>()
            .into_iter()
            .for_each(|handle| {
                self.primes.extend(handle.join().unwrap())
            });

        self.segment += self.thread_count;
    }
}

impl Iterator for PrimeIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor += 1;

        if self.cursor as usize == self.primes.len() {
            if self.segment == 0 {
                self.initialize_kernel();
            } else {
                self.extend_in_parallel();
            }
        }

        Some(self.primes[self.cursor as usize])
    }
}

#[cfg(test)]
mod tests {
    use crate::prime_iterator::stream;

    fn test_segment(start: usize, numbers: [usize; 10]) {
        let mut prime_iterator = stream(32);
        for _ in 0..start {
            prime_iterator.next();
        }
        for i in numbers {
            assert_eq!(Some(i), prime_iterator.next(),
                       "\nYour result (left) did not match the expected output (right)");
        }
    }

    #[test]
    fn tests() {
        println!("testing segment from 0");
        test_segment(0, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);

        println!("testing segment from 10");
        test_segment(10, [31, 37, 41, 43, 47, 53, 59, 61, 67, 71]);

        println!("testing segment from 100");
        test_segment(100, [547, 557, 563, 569, 571, 577, 587, 593, 599, 601]);

        println!("testing segment from 1,000");
        test_segment(1_000, [7927, 7933, 7937, 7949, 7951, 7963, 7993, 8009, 8011, 8017]);

        println!("testing segment from 10,000");
        test_segment(10_000, [104743, 104759, 104761, 104773, 104779, 104789, 104801, 104803, 104827, 104831]);

        println!("testing segment from 100,000");
        test_segment(100_000, [1299721, 1299743, 1299763, 1299791, 1299811, 1299817, 1299821, 1299827, 1299833, 1299841]);

        println!("testing segment from 1,000,000");
        test_segment(1_000_000, [15485867, 15485917, 15485927, 15485933, 15485941, 15485959, 15485989, 15485993, 15486013, 15486041]);

        println!("testing segment from 10,000,000");
        test_segment(10_000_000, [179424691, 179424697, 179424719, 179424731, 179424743, 179424779, 179424787, 179424793, 179424797, 179424799]);
    }
}
