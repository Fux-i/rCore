TARGET := riscv64gc-unknown-none-elf
MODE := release
APP_DIR ?= src/bin
TARGET_DIR := target/$(TARGET)/$(MODE)
APPS := $(wildcard $(APP_DIR)/*.rs)
APP_NAMES := $(patsubst $(APP_DIR)/%.rs,%,$(APPS))
ELFS := $(patsubst $(APP_DIR)/%.rs,$(TARGET_DIR)/%,$(APPS))
BINS := $(patsubst $(APP_DIR)/%.rs,$(TARGET_DIR)/%.bin,$(APPS))

OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

elf: $(APPS)
	@$(foreach app,$(APP_NAMES),cargo build --release --bin $(app);)

binary: elf
	@$(foreach elf,$(ELFS),$(OBJCOPY) $(elf) --strip-all -O binary $(patsubst $(TARGET_DIR)/%,$(TARGET_DIR)/%.bin,$(elf));)

build: binary

clean:
	@cargo clean

.PHONY: elf binary build clean
