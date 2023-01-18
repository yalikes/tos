set history save on
set history size 10000
set history filename ~/.cache/.gdb_history
set confirm off
set architecture riscv:rv64
file target/riscv64gc-unknown-none-elf/debug/tos
target remote localhost:4321
layout split
focus cmd
