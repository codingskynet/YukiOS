# YukiOS

## How to Build & Run
This is very temporary. I'll make it as Makefile.

```bash
brew install qemu
cargo install cargo-binutils
rustup component add llvm-tools-preview

RUSTFLAGS="-C link-args=--script=src/machine/rv64/virt/kernel.ld" cargo rustc --target=riscv64gc-unknown-none-elf --release
rust-objcopy --strip-all -O binary ./target/riscv64gc-unknown-none-elf/release/kernel ./kernel.img
qemu-system-riscv64 -machine virt -nographic -bios none  -kernel kernel.img
```

# Reference
- https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials
- https://github.com/cccriscv/mini-riscv-os
