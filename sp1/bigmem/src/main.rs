#![no_main]

use core::hint::black_box;
sp1_zkvm::entrypoint!(main);

pub fn main() {
    let value = sp1_zkvm::io::read::<u32>();

    let array = [value; 128000];
    black_box(array);
    let result = array[16000];

    sp1_zkvm::io::commit(&result);
}
