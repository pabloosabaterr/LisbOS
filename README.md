# LisbOS (name is WIP)

I'm taking an Operating Systems course and I'm getting really excited about
them, so this project is a way to learn more and get some practical experience.

Written in Rust.

## About

The repo is a monorepo containing both the bootloader and the kernel.

The goal is to improve in a lot of areas, since an OS is a huge project. Of
course, this won't be the next Linux (not even close), but I'll treat it as if
it were.

### Why write both the bootloader and the kernel?

Because the idea is to learn. I don't really care whether it's tedious, hard or
whatever; I'll try to do things myself. I might use some libraries from time to
time, and come back later to rewrite what I need.

### Why Rust?

Honestly, just because. I wanted to learn it properly, and I always fall back
to C because it's where I'm comfortable.

## Structure

- `bootloader/`: UEFI bootloader, built with [uefi-rs](https://github.com/rust-osdev/uefi-rs).
- `kernel/`: the kernel (`x86_64-unknown-none`).

## Running

Requirements:

- Rust, installed with [rustup](https://rustup.rs). The required targets are
  installed automatically from `rust-toolchain.toml`.
- [QEMU](https://www.qemu.org).
- OVMF (UEFI firmware for QEMU). On macOS it comes with `brew install qemu`;
  on Linux, install your distro's `ovmf` / `edk2-ovmf` package.

Then:

```sh
make run
```

If the firmware isn't found automatically, pass its path:

```sh
make run OVMF=/path/to/OVMF_CODE.fd
```

## Bibliography

- [The uefi-rs Book](https://rust-osdev.github.io/uefi-rs/)
- [OSDev Wiki](https://wiki.osdev.org/)
- [UEFI Specification](https://uefi.org/specifications)
- [Writing an OS in Rust](https://os.phil-opp.com/)
