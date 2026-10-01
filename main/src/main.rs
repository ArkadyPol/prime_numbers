use std::time::{Duration, Instant};

use atkin_multi_thread::AtkinPrimeGenerator;

fn main() {
    let mut all_duration = Duration::new(0, 0);
    let iterations = 5;

    for i in 0..iterations {
        let count = 100_000_000;
        let primes = AtkinPrimeGenerator::new();

        let start = Instant::now();

        let last_10: Vec<u64> = primes.skip(count - 10).take(10).collect();

        let duration = start.elapsed();

        if i == 0 {
            println!("Последние 10 чисел: {:?}", last_10);
        }

        println!("Проход {}: {:?}", i + 1, duration);
        all_duration += duration;
    }

    println!("Среднее время : {:?}", all_duration / iterations);
}
