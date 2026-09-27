pub struct SievePrimeGenerator {
    index: usize,
    sieve: Vec<bool>,
    is_expanded: bool,
    emitted_two: bool,
}

impl SievePrimeGenerator {
    pub fn new() -> Self {
        SievePrimeGenerator {
            index: 0,
            sieve: vec![false; 1000],
            is_expanded: false,
            emitted_two: false,
        }
    }

    fn expand_sieve(&mut self) {
        self.is_expanded = true;
        let old_len = self.sieve.len();
        let new_len = old_len + 50_000;

        self.sieve.resize(new_len, false);

        let limit = self.get_number_by_idx(new_len - 1);

        for idx in 0..old_len {
            if !self.sieve[idx] {
                let current_prime = self.get_number_by_idx(idx);

                if current_prime * current_prime > limit {
                    break;
                }

                let old_limit = self.get_number_by_idx(old_len);
                let mut i = (old_limit + current_prime - 1) / current_prime;

                if i % 2 == 0 {
                    i += 1;
                }

                if i < current_prime {
                    i = current_prime;
                }

                let mut composite = current_prime * i;

                let step = current_prime * 2;

                while composite <= limit {
                    self.set_composite(composite);
                    composite += step;
                }
            }
        }
    }

    #[inline(always)]
    fn get_number_by_idx(&self, idx: usize) -> u64 {
        (idx * 2 + 3) as u64
    }

    #[inline(always)]
    fn get_idx_by_number(&self, number: u64) -> usize {
        ((number - 3) / 2) as usize
    }

    fn set_composite(&mut self, composite: u64) {
        let idx = self.get_idx_by_number(composite);

        self.sieve[idx] = true;
    }
}

impl Iterator for SievePrimeGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.emitted_two {
            self.emitted_two = true;
            return Some(2);
        }

        if self.index >= self.sieve.len() {
            self.expand_sieve();
        }

        while self.sieve[self.index] {
            self.index += 1;

            if self.index >= self.sieve.len() {
                self.expand_sieve();
            }
        }

        let current_prime = self.get_number_by_idx(self.index);

        if !self.is_expanded {
            let limit = self.get_number_by_idx(self.sieve.len() - 1);

            let mut composite = current_prime * current_prime;

            let step = current_prime * 2;

            while composite <= limit {
                self.set_composite(composite);
                composite += step;
            }
        }

        self.index += 1;
        Some(current_prime)
    }
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
