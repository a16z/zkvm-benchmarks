use std::time::Duration;

use methods::{SHA3_BENCH_ELF, SHA3_BENCH_ID};
use risc0_zkvm::{ExecutorEnv, LocalProver, Prover};
use utils::{benchmark, size};

fn main() {
    let lengths = [32, 256, 512, 1024, 2048];
    benchmark(bench_sha3, &lengths, "../../benchmark_outputs/sha3_risczero.csv", "n");
}

fn bench_sha3(num_bytes: usize) -> (Duration, usize) {
    let input = vec![5u8; num_bytes];
    let env = ExecutorEnv::builder().write(&input).unwrap().build().unwrap();
    let prover = LocalProver::new("prover");

    let start = std::time::Instant::now();
    let receipt = prover.prove(env, SHA3_BENCH_ELF).unwrap();
    let end = std::time::Instant::now();
    let duration = end.duration_since(start);

    let _output: [u8; 32] = receipt.journal.decode().unwrap();
    receipt.verify(SHA3_BENCH_ID).unwrap();
    
    (duration, size(&receipt))
}

