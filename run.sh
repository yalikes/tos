#qemu-system-riscv64 \
#            -machine virt \
#            -nographic \
#            -m 128M \
#            -bios none \
#            -kernel target/riscv64gc-unknown-none-elf/debug/tos \
#            -S -gdb tcp::4321

qemu-system-riscv64 \
            -machine virt \
            -nographic \
            -m 128M \
            -bios none \
            -kernel target/riscv64gc-unknown-none-elf/debug/tos
