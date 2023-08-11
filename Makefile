# Top makefile
#

# General options
ARCH ?= riscv64
PLATFORM ?=
SMP ?= 1
MODE ?= release
LOG ?= warn
V ?=

# App options
A ?= apps/helloworld
APP ?= $(A)

# QEMU options
BLK ?= n
NET ?= n
BUS ?= mmio

DISK_IMG ?= disk.img
QEMU_LOG ?= n
NET_DUMP ?= n
NET_DEV ?= user

# Network options
IP ?= 10.0.2.15
GW ?= 10.0.2.2

# App type
ifeq ($(wildcard $(APP)),)
  $(error Application path "$(APP)" is not valid)
endif

APP_TYPE := rust

ACCEL ?= n
PLATFORM_NAME ?= riscv64-qemu-virt
TARGET := riscv64gc-unknown-none-elf

export AX_ARCH=$(ARCH)
export AX_PLATFORM=$(PLATFORM_NAME)
export AX_SMP=$(SMP)
export AX_MODE=$(MODE)
export AX_LOG=$(LOG)
export AX_TARGET=$(TARGET)
export AX_IP=$(IP)
export AX_GW=$(GW)

OBJCOPY ?= rust-objcopy --binary-architecture=$(ARCH)

# Paths
OUT_DIR ?= $(APP)

APP_NAME := $(shell basename $(APP))
LD_SCRIPT := $(CURDIR)/modules/axhal/linker_$(PLATFORM_NAME).lds
OUT_ELF := $(OUT_DIR)/$(APP_NAME)_$(PLATFORM_NAME).elf
OUT_BIN := $(OUT_DIR)/$(APP_NAME)_$(PLATFORM_NAME).bin

all: build

include scripts/make/utils.mk
include scripts/make/build.mk
include scripts/make/qemu.mk

build: $(OUT_DIR) $(OUT_BIN)

run: build justrun

justrun:
	$(call run_qemu)

clean:
	rm -rf $(APP)/*.bin $(APP)/*.elf
	cargo clean

.PHONY: all build run justrun test test_no_fail_fast clean disk_image
