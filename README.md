# zkVM Benchmarks

## Installation
### Install Jolt
```bash
rustup target add riscv32i-unknown-none-elf
```

### Install Risc Zero
```bash
cargo install cargo-binstall
cargo binstall cargo-risczero
cargo risczero install
```

### Install SP1
```bash
curl -L https://sp1.succinct.xyz | bash
```

Follow the instructions outputted by this command then run:
```bash
sp1up
```

## Running
To run all benchmarks run:
```bash
make bench-all
```

The benchmark results should be outputted in CSV form in `benchmark_outputs`.

To run an individual benchmark run `make bench-jolt`, `make bench-risczero` or `make bench-sp1`.

