use std::thread;

const STEP: usize = 250_000 * 8;
const THREADS_COUNT: usize = 16;

pub struct SievePrimeGenerator {
    index: usize,
    sieve: Vec<u8>,
    is_expanded: bool,
    emitted_two: bool,
    thread_buffers: Vec<Vec<u8>>,
}

impl SievePrimeGenerator {
    pub fn new() -> Self {
        SievePrimeGenerator {
            index: 0,
            sieve: vec![0; 500], // 4000 чисел
            is_expanded: false,
            emitted_two: false,
            thread_buffers: vec![vec![0; STEP / 8]; THREADS_COUNT],
        }
    }

    fn expand_sieve(&mut self) {
        self.is_expanded = true;
        let old_len_bits = self.sieve.len() * 8;

        let mut buffers = std::mem::take(&mut self.thread_buffers);

        thread::scope(|s| {
            let base_sieve = &self.sieve;

            for (i, buf) in buffers.iter_mut().enumerate() {
                buf.fill(0);

                let thread_start_idx = old_len_bits + (i * STEP);

                s.spawn(move || {
                    fill_new_sieve(base_sieve, thread_start_idx, STEP, buf);
                });
            }
        });

        for buf in &buffers {
            self.sieve.extend_from_slice(buf);
        }

        self.thread_buffers = buffers;
    }

    #[inline(always)]
    fn get_idx_by_number(&self, number: u64) -> usize {
        ((number - 3) / 2) as usize
    }
}

impl Iterator for SievePrimeGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.emitted_two {
            self.emitted_two = true;
            return Some(2);
        }

        if self.index >= self.sieve.len() * 8 {
            self.expand_sieve();
        }

        while is_composite_bit(&self.sieve, self.index) {
            self.index += 1;

            if self.index >= self.sieve.len() * 8 {
                self.expand_sieve();
            }
        }

        let current_prime = get_number_by_idx(self.index);

        if !self.is_expanded {
            let limit = get_number_by_idx(self.sieve.len() * 8 - 1);

            let mut composite = current_prime * current_prime;

            let step = current_prime * 2;

            while composite <= limit {
                let idx = self.get_idx_by_number(composite);
                set_composite_bit(&mut self.sieve, idx);
                composite += step;
            }
        }

        self.index += 1;
        Some(current_prime)
    }
}

#[inline(always)]
fn get_number_by_idx(idx: usize) -> u64 {
    (idx * 2 + 3) as u64
}

fn fill_new_sieve(sieve: &[u8], start_idx: usize, shift: usize, mut new_sieve: &mut [u8]) {
    let start = get_number_by_idx(start_idx);
    let limit = get_number_by_idx(start_idx + shift - 1);

    for idx in 0..(sieve.len() * 8) {
        if !is_composite_bit(&sieve, idx) {
            let current_prime = get_number_by_idx(idx);

            if current_prime * current_prime > limit {
                break;
            }

            let mut i = (start + current_prime - 1) / current_prime;

            if i % 2 == 0 {
                i += 1;
            }

            if i < current_prime {
                i = current_prime;
            }

            let mut composite = current_prime * i;

            let step = current_prime * 2;

            while composite <= limit {
                let local_idx = ((composite - start) / 2) as usize;
                set_composite_bit(&mut new_sieve, local_idx);
                composite += step;
            }
        }
    }
}

#[inline(always)]
fn set_composite_bit(sieve: &mut [u8], local_idx: usize) {
    let byte_idx = local_idx / 8;
    let bit_idx = local_idx % 8;
    sieve[byte_idx] |= 1 << bit_idx;
}

#[inline(always)]
fn is_composite_bit(sieve: &[u8], idx: usize) -> bool {
    let byte_idx = idx / 8;
    let bit_idx = idx % 8;
    (sieve[byte_idx] & (1 << bit_idx)) != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let count = 10_000;
        let primes = SievePrimeGenerator::new();
        let last_10: Vec<u64> = primes.skip(count - 10).take(10).collect();
        assert_eq!(
            last_10,
            [
                104677, 104681, 104683, 104693, 104701, 104707, 104711, 104717, 104723, 104729
            ]
        );
    }

    #[test]
    #[ignore]
    fn it_works_for_big_numbers() {
        let count = 10_000_000;
        let primes = SievePrimeGenerator::new();
        let last_10: Vec<u64> = primes.skip(count - 10).take(10).collect();
        assert_eq!(
            last_10,
            [
                179424551, 179424571, 179424577, 179424601, 179424611, 179424617, 179424629,
                179424667, 179424671, 179424673
            ]
        );
    }
}
