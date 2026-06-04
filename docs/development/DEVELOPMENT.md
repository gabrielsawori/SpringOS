# Development Guide

## Environment Setup

### Prerequisites

- **Rust:** Latest nightly build
  ```bash
  rustup default nightly
  rustup component add rust-src
  ```

- **Build Tools:**
  ```bash
  sudo apt install build-essential xorriso grub-pc-bin mtools
  ```

- **Emulator (optional):**
  ```bash
  sudo apt install qemu-system-x86
  ```

### Project Structure

```
SpringOS/
├── kernel/                    # Kernel source code
│   ├── src/
│   │   ├── main.rs
│   │   ├── arch/
│   │   ├── memory/
│   │   └── task/
│   ├── Cargo.toml
│   ├── linker.ld             # Linker script
│   └── x86_64-springos.json  # Target specification
├── boot/                      # Bootloader configuration
│   └── limine.cfg
├── docs/                      # Documentation
├── Makefile                   # Build automation
└── README.md
```

## Building

### Build Kernel
```bash
cd kernel
cargo +nightly build --release -Z build-std=core,compiler_builtins \
  -Z build-std-features=compiler-builtins-mem -Z json-target-spec
```

### Build ISO
```bash
make iso
```

### Clean Build
```bash
make clean
```

## Running

### Run in QEMU
```bash
make run
```

### Run with Debug Output
```bash
qemu-system-x86_64 -M q35 -m 2G -cdrom SpringOS.iso -boot d -serial stdio
```

### Debug with GDB
```bash
# Terminal 1: Start QEMU with GDB stub
qemu-system-x86_64 -M q35 -m 2G -cdrom SpringOS.iso -boot d -s -S

# Terminal 2: Connect GDB
gdb -ex 'file kernel/target/x86_64-springos/release/kernel' \
    -ex 'target remote :1234'
```

## Development Workflow

### 1. Make Changes
Edit source files in `kernel/src/`

### 2. Test Compilation
```bash
cd kernel && cargo build --release
```

### 3. Build ISO and Run
```bash
cd /path/to/SpringOS && make run
```

### 4. Debug Issues
- Check serial output
- Use QEMU monitor commands
- Check kernel panic messages

## Code Style Guidelines

### Naming Conventions
- **Modules:** snake_case (`memory_manager`)
- **Functions:** snake_case (`initialize_gdt`)
- **Constants:** SCREAMING_SNAKE_CASE (`KERNEL_BASE`)
- **Types:** PascalCase (`GlobalDescriptorTable`)

### Safety
- Document all `unsafe` blocks with `SAFETY:` comments
- Use safe abstractions where possible
- Minimize unsafe code

### Comments
```rust
// Single line comments untuk simple explanations

/// Doc comments untuk public APIs
/// 
/// # Safety
/// Must be called only once during kernel initialization
pub unsafe fn init_idt() { }

// SAFETY: The pointer is guaranteed to be valid because...
unsafe { *(0x1000 as *mut u32) = 0; }
```

### Module Organization
```rust
// ✅ Good: logical grouping
pub mod memory {
    pub mod physical;
    pub mod virtual;
}

// ❌ Avoid: random organization
pub mod thing1;
pub mod memory;
pub mod another_thing;
```

## Architecture Decision Log

### Decision: Higher-Half Kernel
- **Why:** Allows 2GB user space, cleaner address space separation
- **Trade-off:** Requires identity mapping during boot
- **Alternative:** Lower-half kernel (simpler but limited address space)

### Decision: Bitmap Physical Allocator
- **Why:** Simple implementation for Phase 1, easy to understand
- **Trade-off:** Not optimal for fragmentation
- **Upgrade path:** Buddy allocator in future phases

### Decision: x86_64 Only
- **Why:** Focus on complete implementation for one arch
- **Trade-off:** Not portable to ARM, etc.
- **Future:** Abstract architecture layer for multi-arch support

## Debugging Tips

### Kernel Panic
Check serial output for panic message with:
- File name and line number
- Stack trace (to be implemented)

### Memory Corruption
- Check bitmap allocator for leaks
- Verify page table entries
- Check bounds on all array accesses

### Interrupt Issues
- Verify IDT entries are set correctly
- Check GDT selectors in IDT
- Ensure interrupts are enabled/disabled properly

### Compilation Errors
- Clear build cache: `cargo clean`
- Update nightly: `rustup update nightly`
- Check target spec JSON is valid

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md)

## Resources

- [OSDev Wiki](https://wiki.osdev.org)
- [Intel x86-64 Manuals](https://www.intel.com/content/www/en/en/developer/articles/technical/intel-sdm.html)
- [Rust for Linux](https://www.kernel.org/doc/html/latest/rust/index.html)
- [Limine Bootloader](https://github.com/limine-bootloader/limine)
