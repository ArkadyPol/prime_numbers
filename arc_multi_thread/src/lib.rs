use std::{sync::Arc, thread};

pub struct PrimeGenerator {
    index: usize,
    discovered: Vec<u64>,
}

impl PrimeGenerator {
    pub fn new() -> Self {
        PrimeGenerator {
            index: 0,
            discovered: vec![2],
        }
    }
}

impl Iterator for PrimeGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.discovered.len() {
            let prime = self.discovered[self.index];
            self.index += 1;
            return Some(prime);
        }

        let current_prime = *self.discovered.last().unwrap();

        let mut candidate = if current_prime == 2 {
            3
        } else {
            current_prime + 2
        };

        if candidate < 568 {
            while is_composite(candidate, &self.discovered) {
                candidate += 2;
            }
            self.discovered.push(candidate)
        } else {
            const THREADS_COUNT: usize = 16;
            const STEP: u64 = 20000;

            let mut handles = vec![];
            let max_possible_candidate = candidate + (THREADS_COUNT as u64 * STEP);
            let limit_value = (max_possible_candidate as f64).sqrt() as u64 + 1;
            let truncate_index = match self.discovered.binary_search(&limit_value) {
                Ok(idx) => idx,
                Err(idx) => idx,
            };
            let truncated_discovered = self.discovered[..truncate_index].to_vec();
            let discovered = Arc::new(truncated_discovered);

            for i in 0..THREADS_COUNT {
                let discovered_ref = Arc::clone(&discovered);
                let thread_candidate = candidate + (i as u64 * STEP);

                let handle =
                    thread::spawn(move || search_primes(discovered_ref, thread_candidate, STEP));

                handles.push(handle);
            }

            for handle in handles {
                let mut d_primes = handle.join().unwrap();
                self.discovered.append(&mut d_primes);
            }
        }

        self.index += 1;

        Some(current_prime)
    }
}

fn search_primes(discovered: Arc<Vec<u64>>, candidate: u64, shift: u64) -> Vec<u64> {
    let limit = candidate + shift;
    let mut primes = vec![];
    let mut d_candidate = candidate;

    let discovered_slice = &*discovered;

    loop {
        while is_composite(d_candidate, discovered_slice) {
            d_candidate += 2;
        }

        if d_candidate >= limit {
            break;
        }

        primes.push(d_candidate);
        d_candidate += 2;
    }

    primes
}

fn is_composite(candidate: u64, discovered: &[u64]) -> bool {
    discovered.iter().skip(1).any(|&p| candidate % p == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let count = 10_000;
        let primes = PrimeGenerator::new();
        let last_10: Vec<u64> = primes.skip(count - 10).take(10).collect();
        assert_eq!(
            last_10,
            [
                104677, 104681, 104683, 104693, 104701, 104707, 104711, 104717, 104723, 104729
            ]
        );
    }
}
