SHELL := bash
.RECIPEPREFIX = >
.ONESHELL:
MAKEFLAGS += --no-builtin-rules --no-builtin-variables

export RELEASE ?=
export ARCH ?= x64

target_json := kernel/arch/$(ARCH)/$(ARCH).json

export RUSTFLAGS = -Z emit-stack-sizes
CARGO ?= cargo +nightly
CARGOFLAGS += -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem
CARGOFLAGS += --target $(target_json)
CARGOFLAGS += $(if $(RELEASE),--release,)

.PHONY: build-kernel
build-kernel:
> $(CARGO) build $(CARGOFLAGS) --manifest-path kernel/Cargo.toml

.PHONY: iso
iso:
> mkdir -p iso/boot/grub
> cp boot/grub.cfg iso/boot/grub
> cp target/x64/debug/kani iso/kani.elf
> grub-mkrescue -o kani.iso iso

.PHONY: run
run:
> qemu-system-x86_64 -drive format=raw,file=kani.iso