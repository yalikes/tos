run:
	cargo build
	qemu-system-riscv64 \
		-machine virt \
		-m 128M \
		-bios none \
		-device VGA \
		-kernel target/riscv64gc-unknown-none-elf/debug/tos

debug:
	cargo build
	qemu-system-riscv64 \
		-machine virt \
		-m 128M \
		-bios none \
		-device VGA \
		-kernel target/riscv64gc-unknown-none-elf/debug/tos \
		-S -gdb tcp::4321
