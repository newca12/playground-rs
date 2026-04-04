mod target;

use std::time::Instant;
use std::hint::black_box;
use target::fibonacci;

fn main() {
    // 1. Validation (Ensures correctness)
    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(10), 55);
    assert_eq!(fibonacci(20), 6765);
    
    // 2. Benchmark
    let start = Instant::now();
    
    // Compute Fibonacci of 42 (Takes ~1-2 seconds with naive recursion)
    let result = black_box(fibonacci(42));
    
    let duration = start.elapsed();
    
    // 3. Output metric
    println!("result: {}", result);
    println!("execution_time_ns: {}", duration.as_nanos());
}