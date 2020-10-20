TARGET      := riscv64imac-unknown-none-elf
MODE        := release

# 用户程序目录
RUST_SRC_DIR	:= rust/src/bin
# 编译后执行文件目录
RUST_TARGET_DIR	:= rust/target/$(TARGET)/$(MODE)
# 用户程序源文件
RUST_SRC_FILES	:= $(wildcard $(RUST_SRC_DIR)/*.rs)
# 根据源文件取得编译后的执行文件
RUST_BIN_FILES	:= $(patsubst $(RUST_SRC_DIR)/%.rs, $(RUST_TARGET_DIR)/%, $(RUST_SRC_FILES))

OUT_DIR		:= build/disk
IMG_FILE	:= build/raw.img
QCOW_FILE	:= build/disk.img

SDCARD		?= /dev/sdb

.PHONY: dependency build clean

# 安装 rcore-fs-fuse 工具
dependency:
ifeq ($(shell which rcore-fs-fuse),)
	@echo Installing rcore-fs-fuse
	@cargo install rcore-fs-fuse --git https://github.com/rcore-os/rcore-fs
endif

# 编译、打包、格式转换、预留空间
build: dependency
ifeq ($(MODE),release)
	cd rust && cargo build --release
else
	cd rust && cargo build
endif
	$(foreach bin_file,$(BIN_FILES),riscv64-unknown-elf-objcopy --strip-debug $(bin_file) $(bin_file);)
	@echo Targets: $(patsubst $(SRC_DIR)/%.rs, %, $(SRC_FILES))
	@rm -rf $(OUT_DIR)
	@mkdir -p $(OUT_DIR)
	@cp $(BIN_FILES) $(OUT_DIR)
	@rcore-fs-fuse --fs sfs $(IMG_FILE) $(OUT_DIR) zip
	@qemu-img convert -f raw $(IMG_FILE) -O qcow2 $(QCOW_FILE)
	@qemu-img resize $(QCOW_FILE) +1G

sdcard: build
	@sudo dd if=/dev/zero of=$(SDCARD) bs=1M count=50
	@sudo dd if=$(IMG_FILE) of=$(SDCARD)

clean:
	@cargo clean
	@rm -rf $(OUT_DIR) $(IMG_FILE) $(QCOW_FILE)
