use std::time::Duration;

use methods::{
    FIBONACCI_ELF, FIBONACCI_ID
};
use risc0_zkvm::{LocalProver, ExecutorEnv, Prover};
use utils::{benchmark, size};

fn main() {
    let ns = [100, 1000, 10000, 50000];
    benchmark(bench_fibonacci, &ns, "../../benchmark_outputs/fibonacci_risczero.csv", "n");
}

fn bench_fibonacci(n: u32) -> (Duration, usize) {
    let env = ExecutorEnv::builder().write::<u32>(&n).unwrap().build().unwrap();
    let prover = LocalProver::new("prover");

    let start = std::time::Instant::now();
    let receipt = prover.prove(env, FIBONACCI_ELF).unwrap();
    let end = std::time::Instant::now();
    let duration = end.duration_since(start);

    let _output: u32 = receipt.journal.decode().unwrap();
    receipt.verify(FIBONACCI_ID).unwrap();
    
    (duration, size(&receipt))
}

