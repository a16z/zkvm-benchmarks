#![feature(generic_const_exprs)]

use std::time::{Duration, Instant};

use sp1_sdk::{utils::BabyBearPoseidon2, SP1ProofWithIO, SP1Prover, SP1Stdin, SP1Verifier};
use utils::{benchmark, size};

const FIBONACCI_ELF: &[u8] = include_bytes!("../fibonacci/elf/riscv32im-succinct-zkvm-elf");
const SHA2_ELF: &[u8] = include_bytes!("../sha2/elf/riscv32im-succinct-zkvm-elf");
const SHA2_CHAIN_ELF: &[u8] = include_bytes!("../sha2-chain/elf/riscv32im-succinct-zkvm-elf");
const SHA3_CHAIN_ELF: &[u8] = include_bytes!("../sha2-chain/elf/riscv32im-succinct-zkvm-elf");
const SHA3_ELF: &[u8] = include_bytes!("../sha3/elf/riscv32im-succinct-zkvm-elf");
const BIGMEM_ELF: &[u8] = include_bytes!("../bigmem/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    init_logger();

    // 1 Shard
    let iters = [230, 460, 920, 1840, /* 3680 */ ];
    let shard_sizes = [1 << 20, 1 << 21, 1 << 22, 1 << 23, /* 1 << 24 */]; // Max shard_size = 2^24-1
    // let iters = [230, 460, 920, 1840, /* 3680 */];
    // let shard_sizes = [1 << 20, 1 << 21, 1 << 22, 1 << 23, /* 1 << 24 */];
    benchmark_with_shard_size(benchmark_sha2_chain, &iters, &shard_sizes, "../benchmark_outputs/sha2_chain_sp1_1_shard.csv", "iters");

    // 2 Shards
    let iters = [230, 460, 920, 1840, 3680];
    let shard_sizes = [1 << 19, 1 << 20, 1 << 21, 1 << 22, 1 << 23];
    benchmark_with_shard_size(benchmark_sha2_chain, &iters, &shard_sizes, "../benchmark_outputs/sha2_chain_sp1_2_shard.csv", "iters");

    // 4 Shards
    let shard_sizes = [1 << 18, 1 << 19, 1 << 20, 1 << 21, 1 << 22];
    benchmark_with_shard_size(benchmark_sha2_chain, &iters, &shard_sizes, "../benchmark_outputs/sha2_chain_sp1_4_shard.csv", "iters");

    // 8 Shards
    let shard_sizes = [1 << 17, 1 << 18, 1 << 19, 1 << 20, 1 << 21];
    benchmark_with_shard_size(benchmark_sha2_chain, &iters, &shard_sizes, "../benchmark_outputs/sha2_chain_sp1_8_shard.csv", "iters");

    // 16 Shards
    let shard_sizes = [1 << 16, 1 << 17, 1 << 18, 1 << 19, 1 << 20];
    benchmark_with_shard_size(benchmark_sha2_chain, &iters, &shard_sizes, "../benchmark_outputs/sha2_chain_sp1_16_shard.csv", "iters");

    // benchmark(benchmark_sha3_chain, &iters, "../benchmark_outputs/sha3_chain_sp1.csv", "iters");

    // let lengths = [32, 256, 512, 1024, 2048];
    // benchmark(benchmark_sha2, &lengths, "../benchmark_outputs/sha2_sp1.csv", "byte length");
    // benchmark(benchmark_sha3, &lengths, "../benchmark_outputs/sha3_sp1.csv", "byte length");

    // let ns = [100, 1000, 10000, 50000];
    // benchmark(bench_fibonacci, &ns, "../benchmark_outputs/fibonacci_sp1.csv", "n");

    // let values = [5u32];
    // benchmark(bench_bigmem, &values, "../benchmark_outputs/bigmem_sp1.csv", "value");
}

fn init_logger() {
    std::env::set_var("RUST_LOG", "info");
    sp1_core::utils::setup_logger();
}

fn benchmark_with_shard_size(func: fn(u32) -> (Duration, usize), iters: &[u32], shard_sizes: &[usize], file_name: &str, input_name: &str) {
    assert_eq!(iters.len(), shard_sizes.len());
    let mut info = Vec::new();
    for bench_i in 0..iters.len() {
        std::env::set_var("SHARD_SIZE", format!("{}", shard_sizes[bench_i]));
        let duration_and_size = func(iters[bench_i]);
        info.push(duration_and_size);
    }
    utils::write_csv(file_name, input_name, iters, &info);
}

fn benchmark_sha2_chain(iters: u32) -> (Duration, usize) {
    let mut stdin = SP1Stdin::new();
    let input = [5u8; 32];
    stdin.write(&input);
    stdin.write(&iters);

    let start = Instant::now();
    let mut proof = SP1Prover::prove_with_config(SHA2_CHAIN_ELF, stdin, BabyBearPoseidon2::new()).unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);

    let _hash = proof.public_values.read::<[u8; 32]>();
    // SP1Verifier::verify_with_config(SHA2_CHAIN_ELF, &proof, BabyBearPoseidon2::new()).expect("verification failed");

    (duration, size(&proof))
}

fn benchmark_sha3_chain(iters: u32) -> (Duration, usize) {
    let mut stdin = SP1Stdin::new();
    let input = [5u8; 32];
    stdin.write(&input);
    stdin.write(&iters);

    let start = Instant::now();
    let mut proof = SP1Prover::prove_with_config(SHA3_CHAIN_ELF, stdin, BabyBearPoseidon2::new()).unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);

    let _hash = proof.public_values.read::<[u8; 32]>();
    SP1Verifier::verify_with_config(SHA3_CHAIN_ELF, &proof, BabyBearPoseidon2::new()).expect("verification failed");

    (duration, size(&proof))
}

fn benchmark_sha2(num_bytes: usize) -> (Duration, usize) {
    let mut stdin = SP1Stdin::new();
    let input = vec![5u8; num_bytes];
    stdin.write(&input);

    let start = Instant::now();
    let mut proof = SP1Prover::prove_with_config(SHA2_ELF, stdin, BabyBearPoseidon2::new()).unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);

    let _hash = proof.public_values.read::<[u8; 32]>();
    SP1Verifier::verify_with_config(SHA2_ELF, &proof, BabyBearPoseidon2::new()).expect("verification failed");

    (duration, size(&proof))
}

fn benchmark_sha3(num_bytes: usize) -> (Duration, usize) {
    let mut stdin = SP1Stdin::new();
    let input = vec![5u8; num_bytes];
    stdin.write(&input);

    let start = Instant::now();
    let mut proof = SP1Prover::prove_with_config(SHA3_ELF, stdin, BabyBearPoseidon2::new()).unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);

    let _hash = proof.public_values.read::<[u8; 32]>();
    SP1Verifier::verify_with_config(SHA3_ELF, &proof, BabyBearPoseidon2::new()).expect("verification failed");

    (duration, size(&proof))
}

fn bench_fibonacci(n: u32) -> (Duration, usize) {
    let mut stdin = SP1Stdin::new();
    stdin.write(&n);

    let start = Instant::now();
    let mut proof = SP1Prover::prove_with_config(FIBONACCI_ELF, stdin, BabyBearPoseidon2::new()).unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);

    let _output = proof.public_values.read::<u128>();
    SP1Verifier::verify_with_config(FIBONACCI_ELF, &proof, BabyBearPoseidon2::new()).expect("verification failed");

    (duration, size(&proof))
}

fn bench_bigmem(value: u32) -> (Duration, usize) {
    let mut stdin = SP1Stdin::new();
    stdin.write(&value);

    let start = Instant::now();
    let mut proof = SP1Prover::prove_with_config(BIGMEM_ELF, stdin, BabyBearPoseidon2::new()).unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);

    let _output = proof.public_values.read::<u32>();
    SP1Verifier::verify_with_config(BIGMEM_ELF, &proof, BabyBearPoseidon2::new()).expect("verification failed");

    (duration, size(&proof))
}
