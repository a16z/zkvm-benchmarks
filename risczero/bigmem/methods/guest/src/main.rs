#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental


use risc0_zkvm::guest::env;
use core::hint::black_box;

risc0_zkvm::guest::entry!(main);


fn main() {
    let value = env::read::<u32>();

    let array = [value; 128000];
    black_box(array);
    let result = array[16000];

    env::commit(&result);
}
