use std::thread;

const BASE_SIZE: usize = 500 * 8; // 4000 нечётных кандидатов ≈ 8000 чисел
const SEGMENT_SIZE: usize = 200_000 * 8; // 1_600_000 нечётных кандидатов ≈ 3.2 млн чисел
const THREADS_COUNT: usize = 16; // итого 3.2 млн * 16 = 51.2 млн чисел

pub struct SegmentedPrimeGenerator {
    base_size: usize,
    base_primes: Vec<u64>,
    segment_sieves: Vec<Vec<u8>>,
    segment_output: Vec<Vec<u64>>,
    segment_idx: usize,
    segment_buffer_idx: usize,
    segment_pos: usize,
    emitted_two: bool,
}

impl SegmentedPrimeGenerator {
    pub fn new() -> Self {
        let mut base_sieve = vec![0; BASE_SIZE / 8];
        let mut base_primes = Vec::new();
        let limit = get_number_by_idx(BASE_SIZE - 1);

        for idx in 0..BASE_SIZE {
            if !is_composite_bit(&base_sieve, idx) {
                let current_prime = get_number_by_idx(idx);
                let composite = current_prime * current_prime;
                if composite > limit {
                    break;
                }

                let mut local_idx = ((composite - 3) / 2) as usize;
                let step_idx = current_prime as usize;

                while local_idx < BASE_SIZE {
                    set_composite_bit(&mut base_sieve, local_idx);
                    local_idx += step_idx;
                }
            }
        }

        for idx in 0..BASE_SIZE {
            if !is_composite_bit(&base_sieve, idx) {
                base_primes.push(get_number_by_idx(idx));
            }
        }

        SegmentedPrimeGenerator {
            base_size: BASE_SIZE,
            base_primes,
            segment_sieves: vec![vec![0; SEGMENT_SIZE / 8]; THREADS_COUNT],
            segment_output: vec![Vec::new(); THREADS_COUNT],
            segment_idx: 0,
            segment_buffer_idx: 0,
            segment_pos: 0,
            emitted_two: false,
        }
    }

    fn expand_base_primes(&mut self) {
        let mut new_sieve = vec![0; BASE_SIZE / 8];
        let mut new_base_primes = Vec::new();

        fill_segment(
            &self.base_primes,
            self.base_size,
            BASE_SIZE,
            &mut new_sieve,
            &mut new_base_primes,
        );

        self.base_primes.extend(new_base_primes);
        self.base_size += BASE_SIZE;
    }
}

impl Iterator for SegmentedPrimeGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.emitted_two {
            self.emitted_two = true;
            return Some(2);
        }

        loop {
            let max_base_number = get_number_by_idx(self.base_size - 1);
            let max_segment_number =
                get_number_by_idx((self.segment_idx + 1) * SEGMENT_SIZE * THREADS_COUNT - 1);

            if max_segment_number > max_base_number * max_base_number {
                self.expand_base_primes();
            }

            if self.segment_buffer_idx == 0 && self.segment_pos == 0 {
                thread::scope(|s| {
                    let base_primes = &self.base_primes;
                    let sieves = &mut self.segment_sieves;
                    let output = &mut self.segment_output;
                    let local_idx = self.segment_idx * SEGMENT_SIZE * THREADS_COUNT;

                    for (i, (sieve, output)) in sieves.iter_mut().zip(output.iter_mut()).enumerate()
                    {
                        let thread_start_idx = local_idx + i * SEGMENT_SIZE;

                        s.spawn(move || {
                            fill_segment(
                                base_primes,
                                thread_start_idx,
                                SEGMENT_SIZE,
                                sieve,
                                output,
                            );
                        });
                    }
                });
            }

            while self.segment_buffer_idx < THREADS_COUNT {
                let output = &self.segment_output[self.segment_buffer_idx];

                while self.segment_pos < output.len() {
                    let prime = output[self.segment_pos];
                    self.segment_pos += 1;

                    return Some(prime);
                }

                self.segment_buffer_idx += 1;
                self.segment_pos = 0;
            }

            self.segment_idx += 1;
            self.segment_buffer_idx = 0;
            self.segment_pos = 0;
        }
    }
}

#[inline(always)]
fn get_number_by_idx(idx: usize) -> u64 {
    (idx * 2 + 3) as u64
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

fn fill_segment(
    primes: &[u64],
    start_idx: usize,
    shift: usize,
    sieve: &mut [u8],
    output: &mut Vec<u64>,
) {
    let start = get_number_by_idx(start_idx);
    let limit = get_number_by_idx(start_idx + shift - 1);

    sieve.fill(0);
    output.clear();

    for &prime in primes {
        if prime * prime > limit {
            break;
        }
        let mut i = (start + prime - 1) / prime;
        if i % 2 == 0 {
            i += 1;
        }
        if i < prime {
            i = prime;
        }

        let composite = prime * i;

        let mut local_idx = ((composite - start) / 2) as usize;
        let step_idx = prime as usize;

        while local_idx < shift {
            set_composite_bit(sieve, local_idx);
            local_idx += step_idx;
        }
    }

    let mut idx = 0;

    while idx < shift {
        // Инвертированный байт: 1 означает, что кандидат является простым числом.
        let mut bits = !sieve[idx / 8];

        while bits != 0 {
            // Находим следующее число в этом байте, являющееся кандидатом в простые числа.
            let bit = bits.trailing_zeros() as usize;

            output.push(get_number_by_idx(start_idx + idx + bit));

            // Очищаем младший установленный бит, чтобы на следующей итерации найти следующий.
            bits &= bits - 1;
        }

        idx += 8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let count = 10_000;
        let primes = SegmentedPrimeGenerator::new();
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
        let primes = SegmentedPrimeGenerator::new();
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
