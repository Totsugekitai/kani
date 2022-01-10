SHELL := bash
.RECIPEPREFIX = >
.ONESHELL:
MAKEFLAGS += --no-builtin-rules --no-builtin-variables

.SILENT:

export RELEASE ?=
export QEMU ?=
export LOG ?= info
export ARCH ?= x64

target_json := kernel/arch/$(ARCH)/$(ARCH).json
build_mode := $(if $(RELEASE),release,debug)
features := 

ifeq ($(QEMU),1)
features += qemu
endif

ifeq ($(LOG),error)
features += log_error
else ifeq ($(LOG),warn)
features += log_warn
else ifeq ($(LOG),debug)
features += log_debug
else ifeq ($(LOG),info)
features += log_info
else ifeq ($(LOG),trace)
features += log_trace
else
features += log_info
endif

export RUSTFLAGS = -Z emit-stack-sizes
CARGO ?= cargo +nightly
CARGOFLAGS += -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem
CARGOFLAGS += --features "$(features)"
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

QEMUFLAGS += -cdrom kani.iso -serial stdio

.PHONY: run
run:
> qemu-system-x86_64 $(QEMUFLAGS)

.PHONY: debug-run
debug-run:
> qemu-system-x86_64 $(QEMUFLAGS) -no-shutdown -no-reboot -monitor telnet::1234,server,nowait -gdb tcp::12345 -S #-d int 

.PHONY: debug-attach
debug-attach:
> gdb -ex 'file ./target/x64/$(build_mode)/kani' -ex 'target remote localhost:12345' 

.PHONY: all
all: build-kernel build-iso run

.PHONY: clean
clean:
> cargo clean
> rm -rf build kani.iso kani.x64.map
