BOOT_DIR := bootloader
EFI      := $(BOOT_DIR)/target/x86_64-unknown-uefi/debug/bootloader.efi
ESP      := $(BOOT_DIR)/bootimg
OVMF     := $(BOOT_DIR)/ovmf/OVMF.fd

.PHONY: all build esp run clean

all: run

build:
	cd $(BOOT_DIR) && cargo build

esp: build
	mkdir -p $(ESP)/EFI/BOOT
	cp $(EFI) $(ESP)/EFI/BOOT/BOOTx64.efi

run: esp
	qemu-system-x86_64 \
		-drive if=pflash,format=raw,readonly=on,file=$(OVMF) \
		-drive format=raw,file=fat:rw:$(ESP)

clean:
	cd $(BOOT_DIR) && cargo clean
	rm -rf $(ESP)
