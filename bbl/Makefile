# Makefile for RISC-V toolchain; run 'make help' for usage.

ROOT     := $(patsubst %/,%, $(dir $(abspath $(lastword $(MAKEFILE_LIST)))))
RISCV    ?= $(PWD)/install
DEST     := $(abspath $(RISCV))
PATH     := $(DEST)/bin:$(PATH)

NR_CORES := $(shell nproc)

# default configure flags
fesvr-co              = --prefix=$(RISCV) --target=riscv64-unknown-linux-gnu
isa-sim-co            = --prefix=$(RISCV) --with-fesvr=$(DEST)
gnu-toolchain-co-fast = --prefix=$(RISCV) --disable-gdb  #--with-arch=rv64ima --with-abi=lp64 # no multilib for fast
pk-co                 = --prefix=$(RISCV) --host=riscv64-unknown-linux-gnu CC=riscv64-unknown-linux-gnu-gcc OBJDUMP=riscv64-unknown-linux-gnu-objdump
tests-co              = --prefix=$(RISCV)/target

# default make flags
fesvr-mk                = -j$(NR_CORES)
isa-sim-mk              = -j$(NR_CORES)
gnu-toolchain-libc-mk   = linux -j$(NR_CORES)
pk-mk 					= -j$(NR_CORES)
tests-mk         		= -j$(NR_CORES)

# linux image
buildroot_defconfig = configs/buildroot_defconfig
linux_defconfig = configs/linux_defconfig
busybox_defconfig = configs/busybox.config

# rcore kernel
#rcore = $(PWD)/../os/target/riscv64/release/kernel.img
rcore = $(PWD)/../os/target/riscv64imac-unknown-none-elf/debug/kernel.bin
# rcore = /mnt/d/workspace/PCLBoard/rCore-Tutorial/os/target/riscv64imac-unknown-none-elf/debug/kernel.bin 
#cygwin = /mnt/d/cygwin64/home/shinbokuow/bbl_fpga.bin

install-dir:
	mkdir -p $(RISCV)

$(RISCV)/bin/riscv64-unknown-elf-gcc: gnu-toolchain-newlib
	cd riscv-gnu-toolchain/build;\
        make -j$(NR_CORES);\
        cd $(ROOT)

gnu-toolchain-newlib: install-dir
	mkdir -p riscv-gnu-toolchain/build
	cd riscv-gnu-toolchain/build;\
        ../configure --prefix=$(RISCV);\
        cd $(ROOT)

$(RISCV)/bin/riscv64-unknown-linux-gnu-gcc: gnu-toolchain-no-multilib
	cd riscv-gnu-toolchain/build;\
	make $(gnu-toolchain-libc-mk);\
	cd $(ROOT)

gnu-toolchain-libc: $(RISCV)/bin/riscv64-unknown-linux-gnu-gcc

gnu-toolchain-no-multilib: install-dir
	mkdir -p riscv-gnu-toolchain/build
	cd riscv-gnu-toolchain/build;\
	../configure $(gnu-toolchain-co-fast);\
	cd $(ROOT)

toolchain: $(RISCV)/bin/riscv64-unknown-elf-gcc $(RISCV)/bin/riscv64-unknown-linux-gnu-gcc


fesvr: install-dir $(RISCV)/bin/riscv64-unknown-linux-gnu-gcc
	mkdir -p riscv-fesvr/build
	cd riscv-fesvr/build;\
	../configure $(fesvr-co);\
	make $(fesvr-mk);\
	make install;\
	cd $(ROOT)

isa-sim: install-dir $(RISCV)/bin/riscv64-unknown-linux-gnu-gcc fesvr
	mkdir -p riscv-isa-sim/build
	cd riscv-isa-sim/build;\
	../configure $(isa-sim-co);\
	make $(isa-sim-mk);\
	make install;\
	cd $(ROOT)

tests: install-dir $(RISCV)/bin/riscv64-unknown-elf-gcc
	mkdir -p riscv-tests/build
	cd riscv-tests/build;\
	autoconf;\
	../configure $(tests-co);\
	make $(tests-mk);\
	make install;\
	cd $(ROOT)

pk: install-dir $(RISCV)/bin/riscv64-unknown-linux-gnu-gcc
	mkdir -p riscv-pk/build
	cd riscv-pk/build;\
	../configure $(pk-co);\
	make $(pk-mk);\
	make install;\
	cd $(ROOT)

all: gnu-toolchain-libc fesvr isa-sim tests pk

# benchmark for the cache subsystem
cachetest:
	cd ./cachetest/ && $(RISCV)/bin/riscv64-unknown-linux-gnu-gcc cachetest.c -o cachetest.elf
	cp ./cachetest/cachetest.elf rootfs/

# cool command-line tetris
rootfs/tetris:
	cd ./vitetris/ && make clean && ./configure CC=riscv64-unknown-linux-gnu-gcc && make
	cp ./vitetris/tetris $@

vmlinux: $(buildroot_defconfig) $(linux_defconfig) $(busybox_defconfig) cachetest rootfs/tetris
	mkdir -p build
	make -C buildroot defconfig BR2_DEFCONFIG=../$(buildroot_defconfig)
	make -C buildroot
	cp buildroot/output/images/vmlinux build/vmlinux
	cp build/vmlinux vmlinux

bbl_dtb:
	dtc -I dts -O dtb riscv_spike.dts -o riscv_spike.dtb
	cp riscv_spike.dtb build/bbl_payload_dtb

bbl_dtb_fpga:
	dtc -I dts -O dtb riscv_fpga.dts -o riscv_fpga.dtb
	cp riscv_fpga.dtb build/bbl_payload_dtb

bbl_dtb_sim:
	dtc -I dts -O dtb riscv_sim.dts -o riscv_sim.dtb
	cp riscv_sim.dtb build/bbl_payload_dtb

bbl: vmlinux
	cd build && ../riscv-pk/configure --host=riscv64-unknown-elf CC=riscv64-unknown-linux-gnu-gcc OBJDUMP=riscv64-unknown-linux-gnu-objdump --with-payload=vmlinux --enable-logo --with-logo=../configs/logo.txt --enable-print-device-tree
	make -C build
	cp build/bbl bbl

bbl_fpga: bbl_dtb_fpga
	make -C ../os build
	chmod a+x riscv-pk/configure
	cd build && ../riscv-pk/configure --host=riscv64-unknown-elf --with-dtb=riscv_fpga.dtb --with-payload=$(rcore) --enable-logo --with-logo=../configs/logo.txt --enable-print-device-tree
	make -C build
	cp build/bbl bbl_fpga

bbl_fpga_asm: bbl_fpga
	riscv64-unknown-elf-objdump -x bbl_fpga > asm

bbl_sim: vmlinux bbl_dtb_sim
	cd build && ../riscv-pk/configure --host=riscv64-unknown-elf CC=riscv64-unknown-linux-gnu-gcc OBJDUMP=riscv64-unknown-linux-gnu-objdump --enable-dmr-patch --with-dtb=riscv_sim.dtb --with-payload=vmlinux --enable-logo --with-logo=../configs/logo.txt --enable-print-device-tree
	make -C build
	cp build/bbl bbl_sim

bbl_binary: bbl
	riscv64-unknown-elf-objcopy -O binary bbl bbl_binary

bbl_fpga_binary: bbl_fpga
	riscv64-unknown-elf-objcopy -O binary bbl_fpga bbl_fpga_binary

bbl_sim_binary: bbl_sim
	riscv64-unknown-elf-objcopy -O binary bbl_sim bbl_sim_binary

clean:
	rm -rf vmlinux bbl riscv-pk/build/vmlinux riscv-pk/build/bbl cachetest/*.elf rootfs/tetris
	make -C buildroot distclean

bbl.bin: bbl
	riscv64-unknown-elf-objcopy -S -O binary --change-addresses -0x80000000 $< $@

bbl_fpga.bin: bbl_fpga
	riscv64-unknown-elf-objcopy -S -O binary --change-addresses -0x80000000 $< $@
	#cp $@ $(cygwin)

burn: bbl_fpga.bin
	sudo dd if=bbl_fpga.bin of=/dev/sdb1 status=progress oflag=sync bs=1M

openocd:
	openocd -f ariane.cfg
gdb:
	riscv64-unknown-elf-gdb \
		-ex 'set remotetimeout 100' \
		-ex 'file ../os/target/riscv64imac-unknown-none-elf/debug/os' \
		-ex 'target remote localhost:3333' \
		-ex 'b *0x80000000' \
		-ex 'c' \
		-ex 'b *0x80200000' \
		-ex 'c'

bbl_sim.bin: bbl_sim
	riscv64-unknown-elf-objcopy -S -O binary --change-addresses -0x800000000 $< $@

clean-all: clean
	rm -rf riscv-fesvr/build riscv-isa-sim/build riscv-gnu-toolchain/build riscv-tests/build riscv-pk/build

.PHONY: cachetest rootfs/tetris rcore

help:
	@echo "usage: $(MAKE) [RISCV='<install/here>'] [tool/img] ..."
	@echo ""
	@echo "install [tool] to \$$RISCV with compiler <flag>'s"
	@echo "    where tool can be any one of:"
	@echo "        fesvr isa-sim gnu-toolchain tests pk"
	@echo ""
	@echo "build linux images for ariane"
	@echo "    build vmlinux with"
	@echo "        make vmlinux"
	@echo "    build bbl (with vmlinux) with"
	@echo "        make bbl"
	@echo ""
	@echo "There are two clean targets:"
	@echo "    Clean only buildroot"
	@echo "        make clean"
	@echo "    Clean everything (including toolchain etc)"
	@echo "        make clean-all"
	@echo ""
	@echo "defaults:"
	@echo "    RISCV='$(DEST)'"
