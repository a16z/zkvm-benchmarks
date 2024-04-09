#![cfg_attr(feature = "guest", no_std)]
#![no_main]

use core::hint::black_box;

#[jolt::provable(stack_size = 2_000_000)]
fn waste_memory(value: u32) -> u32 {
    let array = [value; 128000];
    black_box(array);
    array[16000]
}

