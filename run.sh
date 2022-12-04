#rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/tos \
#        -O binary target/riscv64gc-unknown-none-elf/release/tos.bin
qemu-system-riscv64 \
            -machine virt \
            -nographic \
            -m 128M \
            -bios none \
            -kernel target/riscv64gc-unknown-none-elf/debug/tos \
            -S -gdb tcp::4321
