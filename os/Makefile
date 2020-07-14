target = riscv64imac-unknown-none-elf
mode = debug
kernel = target/$(target)/$(mode)/os
image = target/$(target)/$(mode)/kernel.bin

kernel:
	cargo build

image: kernel
	rust-objcopy $(kernel) --strip-all -O binary $(image)

build: image

clean:
	cargo clean

asm-view: kernel
	rust-objdump -d $(kernel) | less

asm-save: kernel
	rust-objdump -d $(kernel) > asm
	echo 'save disassembly to file asm'

qemu: image
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios default \
		-device loader,file=$(image),addr=0x80200000

run: build qemu
