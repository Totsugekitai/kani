SHELL := bash
.RECIPEPREFIX = >
.ONESHELL:
MAKEFLAGS += --no-builtin-rules --no-builtin-variables

.SILENT:

export RELEASE ?=
export ARCH ?= x64

target_json := kernel/arch/$(ARCH)/$(ARCH).json
build_mode := $(if $(RELEASE),release,debug)

export RUSTFLAGS = -Z emit-stack-sizes
CARGO ?= cargo +nightly
CARGOFLAGS += -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem
CARGOFLAGS += --target $(target_json)
CARGOFLAGS += $(if $(RELEASE),--release,)

.PHONY: build-kernel
build-kernel:
> $(CARGO) build $(CARGOFLAGS) --manifest-path kernel/Cargo.toml

.PHONY: build-iso
build-iso:
> mkdir -p build/boot/grub
> cp boot/grub.cfg build/boot/grub
> cp target/$(ARCH)/$(build_mode)/kani build/kani.elf
> grub-mkrescue -o kani.iso build

.PHONY: build
build: build-kernel build-iso

.PHONY: run
run:
> qemu-system-x86_64 -d int -no-shutdown -no-reboot -cdrom kani.iso -serial stdio -monitor telnet::1234,server,nowait -gdb tcp::1235

.PHONY: all
all: build-kernel build-iso run

.PHONY: clean
clean:
> cargo clean
> rm -rf build kani.iso kani.x64.map