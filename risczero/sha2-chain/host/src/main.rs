use std::time::Duration;

use methods::{SHA2_CHAIN_BENCH_ELF, SHA2_CHAIN_BENCH_ID};
use risc0_zkvm::{ExecutorEnv, LocalProver, Prover};
use utils::{benchmark, size};

fn main() {
    let iters = [230, 460, 920, 1840, /* 3680 */ ];
    benchmark(bench_sha2_chain, &iters, "../../benchmark_outputs/sha2_chain_risczero.csv", "n");
}

fn bench_sha2_chain(iters: u32) -> (Duration, usize) {
    let input = [5u8; 32];
    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .write(&iters)
        .unwrap()
        .build()
        .unwrap();

    let prover = LocalProver::new("prover");

    let start = std::time::Instant::now();
    let receipt = prover.prove(env, SHA2_CHAIN_BENCH_ELF).unwrap();
    let end = std::time::Instant::now();
    let duration = end.duration_since(start);

    let _output: [u8; 32] = receipt.journal.decode().unwrap();
    receipt.verify(SHA2_CHAIN_BENCH_ID).unwrap();
    
    (duration, size(&receipt))
}

