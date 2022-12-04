set confirm off
set architecture riscv:rv64
file target/riscv64gc-unknown-none-elf/debug/tos
target remote localhost:4321
layout asm
focus cmd
