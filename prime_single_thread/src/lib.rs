pub struct PrimeGenerator {
    next: u64,
    discovered: Vec<u64>,
}

impl PrimeGenerator {
    pub fn new() -> Self {
        PrimeGenerator {
            next: 2,
            discovered: Vec::new(),
        }
    }
}

impl Iterator for PrimeGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current_prime = self.next;
        self.discovered.push(current_prime);

        let mut candidate = if current_prime == 2 {
            3
        } else {
            current_prime + 2
        };

        while self
            .discovered
            .iter()
            .skip(1)
            .take_while(|&&p| p * p <= candidate)
            .any(|&p| candidate % p == 0)
        {
            candidate += 2;
        }

        self.next = candidate;

        Some(current_prime)
    }
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
