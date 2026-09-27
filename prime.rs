use std::time::Instant;
struct PrimeGenerator {
    next: u64,
    discovered: Vec<u64>,
}

impl PrimeGenerator {
    fn new() -> Self {
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

fn main() {
    let count = 1_000_000; // Количество простых чисел для генерации
    let primes = PrimeGenerator::new();

    // Засекаем время НАЧАЛА вычислений
    let start = Instant::now();

    // Быстро пропускаем первые (count - 10) элементов и берем последние 10
    // Метод .skip() в Rust оптимизирован и не делает лишней работы
    let last_10: Vec<u64> = primes.skip(count - 10).take(10).collect();

    // Засекаем время ОКОНЧАНИЯ вычислений
    let duration = start.elapsed();

    // Выводим результат
    println!("Последние 10 чисел: {:?}", last_10);
    println!("Время выполнения на Rust: {:?}", duration);
}
