use std::time::Instant;

use segmented_multi_thread::SegmentedPrimeGenerator;

fn main() {
    let count = 100_000_000;
    let primes = SegmentedPrimeGenerator::new();

    let start = Instant::now();

    let last_10: Vec<u64> = primes.skip(count - 10).take(10).collect();

    let duration = start.elapsed();

    println!("Последние 10 чисел: {:?}", last_10);
    println!("Время выполнения на Rust: {:?}", duration);
}
