# Nama file hasil akhir
ISO_NAME = SpringOS.iso

# Direktori kerja
KERNEL_DIR = kernel
ISO_ROOT = iso_root

# Variabel Limine
LIMINE_BRANCH = v5.x-branch-binary
LIMINE_DIR = limine-dl

.PHONY: all kernel limine iso run clean

# Target utama yang dijalankan saat Anda mengetik 'make'
all: iso

# 1. Kompilasi kernel Rust
kernel:
	@echo "Membangun Kernel Rust..."
	cd $(KERNEL_DIR) && cargo +nightly build --release -Z build-std=core,compiler_builtins -Z build-std-features=compiler-builtins-mem -Z json-target-spec
	cp $(KERNEL_DIR)/target/x86_64-springos/release/kernel $(ISO_ROOT)/system/kernel
# 2. Mengunduh Limine secara otomatis (jika belum ada)
limine:
	@if [ ! -d "$(LIMINE_DIR)" ]; then \
		echo "Mengunduh Limine bootloader..."; \
		git clone https://github.com/limine-bootloader/limine.git --branch=$(LIMINE_BRANCH) --depth=1 $(LIMINE_DIR); \
		make -C $(LIMINE_DIR); \
	fi

# 3. Merakit semuanya menjadi file .iso
iso: kernel limine
	@echo "Merakit file ISO..."
	cp boot/limine.cfg $(ISO_ROOT)/limine.cfg
	cp $(LIMINE_DIR)/limine-bios.sys $(LIMINE_DIR)/limine-bios-cd.bin $(LIMINE_DIR)/limine-uefi-cd.bin $(ISO_ROOT)/boot/limine/
	xorriso -as mkisofs -b boot/limine/limine-bios-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot boot/limine/limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		$(ISO_ROOT) -o $(ISO_NAME)
	$(LIMINE_DIR)/limine bios-install $(ISO_NAME)

# 4. Perintah pintas untuk menjalankan OS di emulator QEMU
run: iso
	qemu-system-x86_64 -M q35 -m 2G -cdrom $(ISO_NAME) -boot d -serial stdio

# 5. Membersihkan file hasil kompilasi
clean:
	cd $(KERNEL_DIR) && cargo clean
	rm -f $(ISO_NAME)
	rm -rf $(LIMINE_DIR)