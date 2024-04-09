use std::{time::{Duration, Instant}, usize};

use utils::benchmark;

pub fn main() {
    let iters = [230, 460, 920, 1840, 3680];
    benchmark(benchmark_sha2_chain, &iters, "../benchmark_outputs/sha2_chain_jolt.csv", "iters");
    
    // benchmark(benchmark_sha3_chain, &iters, "../benchmark_outputs/sha3_chain_jolt.csv", "iters");

    // let lengths = [32, 256, 512, 1024, 2048];
    // benchmark(benchmark_sha2, &lengths, "../benchmark_outputs/sha2_jolt.csv", "byte length");
    // benchmark(benchmark_sha3, &lengths, "../benchmark_outputs/sha3_jolt.csv", "byte length");

    // let ns = [100, 1000, 10000, 50000];
    // benchmark(benchmark_fib, &ns, "../benchmark_outputs/fiboancci_jolt.csv", "n");

    // let values = [5];
    // benchmark(benchmark_bigmem, &values, "../benchmark_outputs/bigmem_jolt.csv", "value");
}

fn benchmark_sha2_chain(iters: u32) -> (Duration, usize) {
    let (prove_sha2_chain, _verify_sha2_chain) = sha2_chain_guest::build_sha2_chain();
    let input = [5u8; 32];

    let start = Instant::now();
    let (_output, proof) = prove_sha2_chain(input, iters);
    let end = Instant::now();

    (end.duration_since(start), proof.size().unwrap())
}

fn benchmark_sha3_chain(iters: u32) -> (Duration, usize) {
    let (prove_sha3_chain, _verify_sha3_chain) = sha3_chain_guest::build_sha3_chain();
    let input = [5u8; 32];

    let start = Instant::now();
    let (_output, proof) = prove_sha3_chain(input, iters);
    let end = Instant::now();

    (end.duration_since(start), proof.size().unwrap())
}

fn benchmark_sha2(num_bytes: usize) -> (Duration, usize) {
    let (prove_sha2, _verify_sha2) = sha2_guest::build_sha2();

    let input = vec![5u8; num_bytes];
    let input = input.as_slice();

    let start = Instant::now();
    let (_output, proof) = prove_sha2(input);
    let end = Instant::now();

    (end.duration_since(start), proof.size().unwrap())
}

fn benchmark_sha3(num_bytes: usize) -> (Duration, usize) {
    let (prove_sha3, _verify_sha3) = sha3_guest::build_sha3();

    let input = vec![5u8; num_bytes];
    let input = input.as_slice();

    let start = Instant::now();
    let (_output, proof) = prove_sha3(input);
    let end = Instant::now();

    (end.duration_since(start), proof.size().unwrap())
}

fn benchmark_fib(n: u32) -> (Duration, usize) {
    let (prove_fib, _verify_fib) = fibonacci_guest::build_fib();

    let start = Instant::now();
    let (_output, proof) = prove_fib(n);
    let end = Instant::now();

    (end.duration_since(start), proof.size().unwrap())
}

fn benchmark_bigmem(value: u32) -> (Duration, usize) {
    let (prove_bigmem, verify_bigmem) = bigmem_guest::build_waste_memory();
    let start = Instant::now();
    let (_output, proof) = prove_bigmem(value);
    let end = Instant::now();

    (end.duration_since(start), proof.size().unwrap())
}
