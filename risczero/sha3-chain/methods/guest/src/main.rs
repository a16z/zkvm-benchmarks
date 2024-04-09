#![no_main]
#![no_std]

use risc0_zkvm::guest::env;
use sha3::{Digest, Keccak256};

risc0_zkvm::guest::entry!(main);

fn main() {
    let input: [u8; 32] = env::read();
    let num_iters: u32 = env::read();
    let mut hash = input;
    for _ in 0..num_iters {
        let mut hasher = Keccak256::new();
        hasher.update(input);
        let res = &hasher.finalize();
        hash = Into::<[u8; 32]>::into (*res);
    }

    env::commit::<[u8; 32]>(&hash.into());
}
