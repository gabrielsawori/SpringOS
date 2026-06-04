# Troubleshooting Guide - QEMU & Kernel Testing

## ❌ Problem: QEMU Restart setelah Limine

Beberapa kemungkinan penyebab dan solusinya:

---

## 1. Install QEMU Properly di Ubuntu

### Jika QEMU dari snap (bermasalah):
```bash
# Remove snap version
sudo snap remove qemu

# Install dari apt (native)
sudo apt update
sudo apt install qemu-system-x86 qemu-utils
```

### Verify installation:
```bash
which qemu-system-x86_64
qemu-system-x86_64 --version
```

---

## 2. Test Kernel dengan Output Logging

### Run dengan log output:
```bash
cd /home/gabriel-e-sawori/Documents/SpringOS
timeout 5 qemu-system-x86_64 \
  -M q35 \
  -m 2G \
  -cdrom SpringOS.iso \
  -boot d \
  -serial stdio \
  2>&1 | tee qemu_output.log
```

### Output yang diharapkan:
```
KERNEL STARTING...
Kernel entry point reached!
====================================
Booting SpringOS (Phase 1)...
====================================
[*] Initializing GDT...
[+] GDT Initialized
[+] IDT Initialized
[+] PIC Initialized
[+] Interrupts Enabled
====================================
[+] Halo Arsitek! SpringOS berhasil hidup!
...
```

---

## 3. Debugging dengan QEMU Monitor

### Run dengan monitor:
```bash
qemu-system-x86_64 \
  -M q35 \
  -m 2G \
  -cdrom SpringOS.iso \
  -boot d \
  -serial stdio \
  -monitor telnet:127.0.0.1:55555,server,nowait
```

### Di terminal lain:
```bash
telnet 127.0.0.1 55555
(qemu) info registers
(qemu) x /10i $rip
(qemu) quit
```

---

## 4. Common Issues & Solutions

### Issue A: QEMU Restart Loop
**Cause:** Kernel tidak boot properly

**Solution:**
1. Lihat serial output untuk error message
2. Check linker script (`kernel/linker.ld`)
3. Verify limine.cfg

### Issue B: Library Error
**Error:** `symbol lookup error: libpthread.so.0`

**Solution:**
```bash
# Option 1: Install native QEMU
sudo apt install qemu-system-x86 --no-install-recommends

# Option 2: Fix snap env
export LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libpthread.so.0

# Option 3: Use AppImage QEMU
wget https://github.com/probonopd/qemu/releases/download/continuous/QEMU-x86_64.AppImage
chmod +x QEMU-x86_64.AppImage
./QEMU-x86_64.AppImage ...
```

### Issue C: kernel binary tidak ditemukan
**Error:** `No such file or directory`

**Solution:**
```bash
# Ensure kernel built
cd kernel && cargo build --release ...
cd ..

# Check binary exists
ls -lh kernel/target/x86_64-springos/release/kernel

# Rebuild ISO
make clean
make iso
```

---

## 5. Build & Run Commands

### Full build sequence (recommended):
```bash
cd /home/gabriel-e-sawori/Documents/SpringOS

# Clean old build
make clean

# Build kernel
cd kernel
cargo +nightly build --release \
  -Z build-std=core,compiler_builtins \
  -Z build-std-features=compiler-builtins-mem \
  -Z json-target-spec
cd ..

# Build ISO
make iso

# Run & capture output
timeout 5 qemu-system-x86_64 \
  -M q35 \
  -m 2G \
  -cdrom SpringOS.iso \
  -boot d \
  -serial stdio \
  2>&1 | tee kernel_output.log
```

### Check output:
```bash
cat kernel_output.log | grep "KERNEL\|Initialized\|Error"
```

---

## 6. Verify Each Component

### Check linker script:
```bash
cat kernel/linker.ld | grep -E "ENTRY|0x"
```

### Check limine config:
```bash
cat boot/limine.cfg
cat iso_root/limine.cfg
```

### Check kernel binary:
```bash
file kernel/target/x86_64-springos/release/kernel
objdump -h kernel/target/x86_64-springos/release/kernel | head -20
```

### Check ISO content:
```bash
mkdir -p /tmp/iso_check
mount -o loop SpringOS.iso /tmp/iso_check
ls -la /tmp/iso_check/boot/
ls -la /tmp/iso_check/system/
umount /tmp/iso_check
```

---

## 7. Serial Port Debugging

### Send data ke serial port:
```bash
# Check if serial working
timeout 5 qemu-system-x86_64 \
  -M q35 \
  -m 2G \
  -cdrom SpringOS.iso \
  -boot d \
  -serial file:serial_output.txt \
  &

# Monitor output
tail -f serial_output.txt
```

### If no output:
1. Check `outb()` function di `main.rs`
2. Verify serial port address (0x3F8 untuk COM1)
3. Ensure print_serial() calls before architecture init

---

## 8. GDB Debugging

### Setup GDB debugging:
```bash
# Terminal 1: Start QEMU dengan GDB stub
qemu-system-x86_64 \
  -M q35 \
  -m 2G \
  -cdrom SpringOS.iso \
  -boot d \
  -s -S \
  -serial stdio

# Terminal 2: Connect GDB
gdb \
  -ex 'file kernel/target/x86_64-springos/release/kernel' \
  -ex 'target remote :1234' \
  -ex 'break _start' \
  -ex 'continue'
```

### GDB commands:
```
(gdb) disassemble /m _start
(gdb) info registers
(gdb) x /20i $rip
(gdb) step
(gdb) continue
(gdb) quit
```

---

## 9. Checklist untuk Troubleshooting

- [ ] QEMU native (bukan snap) installed
- [ ] Kernel compiles tanpa error
- [ ] ISO builds successfully
- [ ] Serial output muncul di terminal
- [ ] Limine bootloader messages visible
- [ ] Kernel messages visible
- [ ] No hangs or restarts

---

## 10. Expected Boot Sequence

```
1. BIOS/UEFI startup
   ↓
2. Limine bootloader loading
   (QEMU shows bootloader messages)
   ↓
3. Limine loads kernel binary
   ↓
4. Limine jumps to kernel _start()
   ↓
5. Kernel prints: "KERNEL STARTING..."
   ↓
6. Kernel initializes GDT
   ↓
7. Kernel initializes IDT
   ↓
8. Kernel initializes PIC
   ↓
9. Success messages
   ↓
10. Kernel waits in `loop { hlt }`
```

---

## Quick Commands

```bash
# Full build & test
cd /home/gabriel-e-sawori/Documents/SpringOS && \
make clean && \
cd kernel && \
cargo +nightly build --release -Z build-std=core,compiler_builtins -Z build-std-features=compiler-builtins-mem -Z json-target-spec && \
cd .. && \
make iso && \
timeout 5 qemu-system-x86_64 -M q35 -m 2G -cdrom SpringOS.iso -boot d -serial stdio

# Check last output
tail -20 /tmp/qemu_last_output.log

# Validate build
file kernel/target/x86_64-springos/release/kernel
```

---

## Still Not Working?

Buat file baru: `DEBUG_SESSION.md` dan document:
1. Output dari `make iso`
2. Output dari QEMU
3. Output dari `objdump -h kernel binary`
4. Ubuntu version dan QEMU version
5. Steps yang sudah dicoba

Kemudian debug lebih lanjut berdasarkan actual error messages.
