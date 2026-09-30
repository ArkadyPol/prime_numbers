const BASE_SIZE: usize = 250 * 16; // 4000 кандидатов ≈ 15000 чисел
const SEGMENT_SIZE: usize = 200_000 * 16; // 3_200_000 кандидатов ≈ 12_000_000 чисел
const REMAINDERS: [u64; 16] = [1, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 49, 53, 59];
const fn make_rem_to_idx() -> [i8; 60] {
    let mut table = [-1i8; 60];
    let mut i = 0;

    while i < REMAINDERS.len() {
        table[REMAINDERS[i] as usize] = i as i8;
        i += 1;
    }

    table
}

const REM_TO_IDX: [i8; 60] = make_rem_to_idx();

pub struct AtkinPrimeGenerator {
    base_size: usize,
    base_primes: Vec<u64>,
    segment_sieve: Vec<u8>,
    segment_output: Vec<u64>,
    segment_idx: usize,
    segment_pos: usize,
    first_primes: ([u64; 3], usize),
}

impl AtkinPrimeGenerator {
    pub fn new() -> Self {
        let mut base_sieve = vec![false; BASE_SIZE];
        let mut base_primes = Vec::new();
        let limit = get_number_by_idx(BASE_SIZE - 1);

        for idx in 1..BASE_SIZE {
            if !base_sieve[idx] {
                let current_prime = get_number_by_idx(idx);
                let start_composite = current_prime * current_prime;
                if start_composite > limit {
                    break;
                }

                let mut composite = start_composite;

                let mut i = get_idx_by_number(current_prime).unwrap() + 1;

                while composite <= limit {
                    let idx = get_idx_by_number(composite).unwrap();
                    base_sieve[idx] = true;
                    composite = current_prime * get_number_by_idx(i);
                    i += 1;
                }
            }
        }

        for idx in 1..BASE_SIZE {
            if !base_sieve[idx] {
                base_primes.push(get_number_by_idx(idx));
            }
        }

        AtkinPrimeGenerator {
            base_size: BASE_SIZE,
            base_primes,
            segment_sieve: vec![0; SEGMENT_SIZE / 8],
            segment_output: Vec::new(),
            segment_idx: 0,
            segment_pos: 0,
            first_primes: ([2, 3, 5], 0),
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

impl Iterator for AtkinPrimeGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_primes.1 < self.first_primes.0.len() {
            let prime = self.first_primes.0[self.first_primes.1];
            self.first_primes.1 += 1;
            return Some(prime);
        }

        loop {
            let max_base_number = get_number_by_idx(self.base_size - 1);
            let max_segment_number = get_number_by_idx((self.segment_idx + 1) * SEGMENT_SIZE - 1);

            if max_segment_number > max_base_number * max_base_number {
                self.expand_base_primes();
            }

            if self.segment_pos == 0 {
                fill_segment(
                    &self.base_primes,
                    self.segment_idx * SEGMENT_SIZE,
                    SEGMENT_SIZE,
                    &mut self.segment_sieve,
                    &mut self.segment_output,
                );
            }

            while self.segment_pos < self.segment_output.len() {
                let prime = self.segment_output[self.segment_pos];
                self.segment_pos += 1;

                return Some(prime);
            }

            self.segment_idx += 1;
            self.segment_pos = 0;
        }
    }
}

#[inline(always)]
fn get_number_by_idx(idx: usize) -> u64 {
    let quotient = (idx / 16) as u64;
    let rem_idx = idx % 16;
    (quotient * 60 + REMAINDERS[rem_idx]) as u64
}

#[inline(always)]
fn get_idx_by_number(number: u64) -> Option<usize> {
    let quotient = number / 60;
    let remainder = (number % 60) as usize;

    let rem_idx = REM_TO_IDX[remainder];

    if rem_idx < 0 {
        None
    } else {
        Some(quotient as usize * 16 + rem_idx as usize)
    }
}

fn fill_segment(
    primes: &[u64],
    start_idx: usize,
    shift: usize,
    sieve: &mut [u8],
    output: &mut Vec<u64>,
) {
    sieve.fill(0);
    output.clear();

    let start = get_number_by_idx(start_idx);
    let limit = get_number_by_idx(start_idx + shift - 1);

    // проверка уравнения 4x2 + y2 = n нечётно
    first_equation(start_idx, sieve, start, limit);

    // проверка уравнения 3x2 + y2 = n нечётно
    second_equation(start_idx, sieve, start, limit);

    // проверка уравнения 3x2 − y2 = n нечётно
    third_equation(start_idx, sieve, start, limit);

    // удаление кратныx квадратам простых чисел
    free_squares(primes, start_idx, sieve, start, limit);

    let mut idx = 0;

    while idx < shift {
        let mut bits = sieve[idx / 8];

        while bits != 0 {
            let bit = bits.trailing_zeros() as usize;

            output.push(get_number_by_idx(start_idx + idx + bit));

            bits &= bits - 1;
        }

        idx += 8;
    }
}

fn first_equation(start_idx: usize, sieve: &mut [u8], start: u64, limit: u64) {
    for x in 1.. {
        let x2 = 4 * x * x;

        if x2 + 1 > limit {
            break;
        }

        let mut y_start = if start <= x2 + 1 {
            1
        } else {
            (start - x2 - 1).isqrt() + 1
        };

        if y_start % 2 == 0 {
            y_start += 1;
        }

        if x % 3 == 0 {
            let mut y = y_start;

            if y % 3 == 0 {
                y += 2;
            }

            let mut step = if y % 6 == 1 { 4 } else { 2 };
            let mut n = x2 + y * y;

            while n <= limit {
                if let Some(idx) = get_idx_by_number(n) {
                    let local_idx = idx - start_idx;
                    toggle_bit(sieve, local_idx);
                }

                y += step;
                n = x2 + y * y;
                step = 6 - step; // 4 -> 2 -> 4 -> 2...
            }
        } else {
            let mut n = x2 + y_start * y_start;
            // (y + 2)² - y² = 4*y + 4
            let mut delta = 4 * y_start + 4;

            while n <= limit {
                if let Some(idx) = get_idx_by_number(n) {
                    let local_idx = idx - start_idx;
                    toggle_bit(sieve, local_idx);
                }

                n += delta;
                // 4*(y+2)+4 = (4y+4) + 8
                delta += 8;
            }
        }
    }
}

fn second_equation(start_idx: usize, sieve: &mut [u8], start: u64, limit: u64) {
    for x in (1..).step_by(2) {
        let x2 = 3 * x * x;
        if x2 + 4 > limit {
            break;
        }

        let mut y_start = if start <= x2 + 4 {
            2
        } else {
            (start - x2 - 1).isqrt() + 1
        };

        if y_start % 2 != 0 {
            y_start += 1;
        }

        let mut y = y_start;

        if y % 3 == 0 {
            y += 2;
        }

        let mut step = if y % 6 == 2 { 2 } else { 4 };

        loop {
            let n = x2 + y * y;
            if n > limit {
                break;
            }

            if let Some(idx) = get_idx_by_number(n) {
                let local_idx = idx - start_idx;
                toggle_bit(sieve, local_idx);
            }

            y += step;
            step = 6 - step; // 2 -> 4 -> 2 -> 4
        }
    }
}

fn third_equation(start_idx: usize, sieve: &mut [u8], start: u64, limit: u64) {
    let mut x_start = ((start + 1) / 3).isqrt().max(2);
    if 3 * x_start * x_start < start + 1 {
        x_start += 1;
    }

    for x in x_start.. {
        let x2 = 3 * x * x;

        if 2 * x * x + 2 * x - 1 > limit {
            break;
        }

        let mut y_start = if x2 <= limit + 1 {
            1
        } else {
            (x2 - limit - 1).isqrt() + 1
        };

        if x % 2 == y_start % 2 {
            y_start += 1;
        }

        let mut y = y_start;
        if y % 3 == 0 {
            y += 2;
        }

        let mut step = match y % 6 {
            1 => 4,
            2 => 2,
            4 => 4,
            5 => 2,
            _ => unreachable!(),
        };

        while y < x {
            let n = x2 - y * y;

            if n < start {
                break;
            }

            if let Some(idx) = get_idx_by_number(n) {
                let local_idx = idx - start_idx;
                toggle_bit(sieve, local_idx);
            }

            y += step;
            step = 6 - step;
        }
    }
}

fn free_squares(primes: &[u64], start_idx: usize, sieve: &mut [u8], start: u64, limit: u64) {
    for &prime in primes {
        let square = prime * prime;

        if square > limit {
            break;
        }

        let mut multiplier = (start + square - 1) / square;

        while REM_TO_IDX[(multiplier % 60) as usize] < 0 {
            multiplier += 1;
        }

        let mut composite = square * multiplier;

        let mut i = get_idx_by_number(multiplier).unwrap() + 1;

        while composite <= limit {
            let idx = get_idx_by_number(composite).unwrap();
            set_composite_bit(sieve, idx - start_idx);
            composite = square * get_number_by_idx(i);
            i += 1;
        }
    }
}

#[inline(always)]
fn toggle_bit(sieve: &mut [u8], idx: usize) {
    let byte_idx = idx / 8;
    let bit_idx = idx % 8;

    sieve[byte_idx] ^= 1 << bit_idx;
}

#[inline(always)]
fn set_composite_bit(sieve: &mut [u8], local_idx: usize) {
    let byte_idx = local_idx / 8;
    let bit_idx = local_idx % 8;
    sieve[byte_idx] &= !(1 << bit_idx);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let count = 10_000;
        let primes = AtkinPrimeGenerator::new();
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
        let primes = AtkinPrimeGenerator::new();
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
