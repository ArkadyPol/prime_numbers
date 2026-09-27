use std::thread;

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

            let mut all_new_primes = vec![Vec::new(); THREADS_COUNT];

            thread::scope(|s| {
                let discovered_ref = &self.discovered;

                for (i, thread_cell) in all_new_primes.chunks_mut(1).enumerate() {
                    let thread_candidate = candidate + (i as u64 * STEP);

                    s.spawn(move || {
                        let primes = search_primes(discovered_ref, thread_candidate, STEP);
                        thread_cell[0] = primes;
                    });
                }
            });

            for mut sub_vec in all_new_primes {
                self.discovered.append(&mut sub_vec);
            }
        }

        self.index += 1;

        Some(current_prime)
    }
}

fn search_primes(discovered: &[u64], candidate: u64, shift: u64) -> Vec<u64> {
    let limit = candidate + shift;
    let mut primes = vec![];
    let mut d_candidate = candidate;

    loop {
        while is_composite(d_candidate, discovered) {
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
    discovered
        .iter()
        .skip(1)
        .take_while(|&&p| p * p <= candidate)
        .any(|&p| candidate % p == 0)
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
