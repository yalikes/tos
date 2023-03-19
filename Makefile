run:
	cargo build
	qemu-system-riscv64 \
		-machine virt \
		-m 128M \
		-bios none \
		-global virtio-mmio.force-legacy=false \
		-device virtio-gpu \
		-drive file=target/fs.img,if=none,format=raw,id=x0 \
		-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
		-kernel target/riscv64gc-unknown-none-elf/debug/tos

debug:
	cargo build
	qemu-system-riscv64 \
		-machine virt \
		-m 128M \
		-bios none \
		-global virtio-mmio.force-legacy=false \
		-device virtio-gpu \
		-drive file=target/fs.img,if=none,format=raw,id=x0 \
		-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
		-kernel target/riscv64gc-unknown-none-elf/debug/tos \
		-S -gdb tcp::4321
