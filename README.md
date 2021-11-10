# kani - 64 bit multiboot toy OS written in Rust

## build

### Requirements

- `cargo`
- `grub`
- `xorriso`

### build kernel

```sh
$ make build-kernel
```

### generate iso file

```sh
$ make build-iso
```

### build kernel and generate iso file

```sh
$ make build
```

### Release build

```sh
$ RELEASE=1 make ...
```

## Run

### Requirements

- `qemu-system`

### Run QEMU

```sh
$ make run
```
