# Main building script

include scripts/make/cargo.mk
include scripts/make/features.mk

rust_package := $(shell cat $(APP)/Cargo.toml | sed -n 's/^name = "\([a-z0-9A-Z_\-]*\)"/\1/p')
rust_target_dir := $(CURDIR)/target/$(TARGET)/$(MODE)
rust_elf := $(rust_target_dir)/$(rust_package)

$(if $(V), $(info RUSTFLAGS: "$(RUSTFLAGS)"))
export RUSTFLAGS

_cargo_build:
	@printf "    $(GREEN_C)Building$(END_C) App: $(APP_NAME), Arch: $(ARCH), Platform: $(PLATFORM_NAME), App type: $(APP_TYPE)\n"
	$(call cargo_build,--manifest-path $(APP)/Cargo.toml,$(AX_FEAT) $(LIB_FEAT) $(APP_FEAT))
	@cp $(rust_elf) $(OUT_ELF)

$(OUT_DIR):
	$(call run_cmd,mkdir,-p $@)

$(OUT_BIN): _cargo_build $(OUT_ELF)
	$(call run_cmd,$(OBJCOPY),$(OUT_ELF) --strip-all -O binary $@)

.PHONY: _cargo_build
