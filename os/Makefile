target = riscv64imac-unknown-none-elf
mode = debug
kernel = target/$(target)/$(mode)/os
image = target/$(target)/$(mode)/kernel.bin
k210 = target/$(target)/$(mode)/k210.bin
_k210 = target/$(target)/$(mode)/k210.kfpkg
k210-serialport = /dev/ttyUSB0
sbi ?= opensbi

kernel:
	cargo build

image: kernel
	rust-objcopy $(kernel) --strip-all -O binary $(image)

k210: image
	\cp built/$(sbi).bin $(k210)
	dd if=$(image) of=$(k210) bs=128K seek=1


_k210: image
	mkdir built/temp
	dd if=built/$(sbi).bin of=built/temp/sbi.bin bs=512K count=1
	cp $(image) built/temp/k210.bin
	cp built/flash-list.json built/temp/flash-list.json
	cd built/temp && zip -r k210.zip . && cd ../..
	cp built/temp/k210.zip $(_k210)
	rm -rf built/temp

run-k210: k210
	python3 built/kflash.py -p $(k210-serialport) -b 1500000 -t $(k210)

build: image

clean:
	cargo clean

asm-view: kernel
	rust-objdump -d $(kernel) | less

asm-view-x: kernel
	rust-objdump -x $(kernel) | less

asm-save: kernel
	rust-objdump -d $(kernel) > asm
	echo 'save disassembly to file asm'

qemu: image
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios default \
		-device loader,file=$(image),addr=0x80200000

	
debug: image
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios default \
		-device loader,file=$(image),addr=0x80200000 \
		-s -S
gdb: image
	riscv64-unknown-elf-gdb \
		-ex 'file $(kernel)' \
		-ex 'set arch riscv:rv64' \
		-ex 'target remote localhost:1234'
run: build qemu
