#!/bin/bash
set -xue

QEMU=qemu-system-riscv32

cargo build --verbose --release

$QEMU -machine virt -bios default -nographic -serial mon:stdio --no-reboot -kernel ./target/riscv32imac-unknown-none-elf/release/riscv32-nogui-os
