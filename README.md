# SpringOS 🖥️

A bare-metal x86_64 OS kernel written in Rust from scratch.

**Status:** Phase 2 (Core Features) - In Progress  
**Last Updated:** June 4, 2026

## 🚀 Quick Start

### Build
```bash
cd SpringOS
make iso
```

### Run
```bash
make run
```

## 📖 Documentation

- **[Project Summary](PROJECT_SUMMARY.md)** - Overview of Phase 1 & 2
- **[Development Guide](docs/development/DEVELOPMENT.md)** - Setup & workflow
- **[Architecture](docs/architecture/OVERVIEW.md)** - System design
- **[Roadmap](docs/ROADMAP.md)** - Phase 1-4 plan
- **[API Reference](docs/api/KERNEL_API.md)** - Kernel APIs

## 🎯 Project Phases

### ✅ Phase 1: Foundation
- [x] GDT (Global Descriptor Table)
- [x] IDT (Interrupt Descriptor Table)
- [x] Memory management basics
- [x] Boot sequence

**Status:** Complete

### 🔄 Phase 2: Core Features
- [x] Exception handling (all 32 exceptions)
- [x] PIC controller (interrupt routing)
- [ ] Timer interrupts
- [ ] Preemptive scheduling
- [ ] Context switching

**Status:** 50% Complete

### ⏳ Phase 3: Privilege & Processes
- [ ] User mode support
- [ ] System calls
- [ ] Process management
- [ ] Virtual memory

### ⏳ Phase 4: I/O & Drivers
- [ ] Device drivers
- [ ] Filesystem support
- [ ] Keyboard input
- [ ] Disk I/O

## 📊 Project Stats

- **Total Code:** ~650 LOC
- **Documentation:** 8 files, ~1800 lines
- **Build Time:** 0.3 seconds
- **Architecture:** x86_64 (single CPU)
- **Language:** Rust (no_std, no_main)

## 🏗️ Architecture

```
┌─────────────────────────────────────┐
│     User Applications (Ring 3)      │
├─────────────────────────────────────┤
│   Kernel (Ring 0) - SpringOS        │
│  ┌─────────────────────────────┐    │
│  │ Exception/Interrupt Handling│    │
│  │ Memory Management           │    │
│  │ Task Scheduling             │    │
│  │ Device Drivers              │    │
│  └─────────────────────────────┘    │
├─────────────────────────────────────┤
│   x86_64 Hardware Abstraction       │
│   GDT · IDT · Paging · Interrupts   │
└─────────────────────────────────────┘
```

## 🔧 Module Structure

```
kernel/src/
├── arch/x86_64/          # Architecture-specific code
│   ├── gdt.rs            # Segment table
│   ├── idt.rs            # Interrupt table
│   ├── exceptions.rs     # Exception handlers
│   └── pic.rs            # Interrupt controller
├── memory/               # Memory management
├── task/                 # Task/Process management (Phase 2)
└── drivers/              # Device drivers (Phase 4)
```

## 💡 Key Features (Current)

### ✅ Implemented
- **GDT:** Memory segmentation with Ring 0/3 separation
- **IDT:** All 32 CPU exceptions routed to handlers
- **PIC:** Hardware interrupt routing (IRQ 0-15)
- **Memory:** Physical allocator, page tables, kernel heap
- **I/O:** Serial port communication

### ⏳ Coming Soon
- **Timer:** Preemptive scheduling based on timer interrupt
- **Multitasking:** Context switching between multiple tasks
- **User Mode:** Ring 3 applications with syscalls
- **Filesystem:** File I/O with support for FAT32/ext2

## 🛠️ Development

### Prerequisites
```bash
rustup default nightly
rustup component add rust-src
sudo apt install xorriso build-essential
```

### Build
```bash
make kernel    # Build kernel only
make iso       # Build ISO
make clean     # Clean build artifacts
```

### Run & Debug
```bash
make run                              # Run in QEMU
qemu-system-x86_64 -M q35 -m 2G -cdrom SpringOS.iso -boot d -serial stdio
```

### View Source
All kernel code is in `kernel/src/` with comprehensive comments.

## 📚 Learning Resources

- [OSDev Wiki](https://wiki.osdev.org) - OS development tutorials
- [Intel x86-64 Manuals](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)
- [Rust Core Library Docs](https://doc.rust-lang.org/core/)
- [Limine Bootloader](https://github.com/limine-bootloader/limine)

## 🤝 Contributing

Contributions welcome! See [CONTRIBUTING.md](docs/CONTRIBUTING.md) for guidelines.

**Areas needing help:**
- Timer interrupt handler
- Context switching implementation
- VGA driver
- System call dispatcher

## 📝 Documentation

### For Users
- [Development Guide](docs/development/DEVELOPMENT.md) - How to build and run
- [Architecture Overview](docs/architecture/OVERVIEW.md) - System design
- [Roadmap](docs/ROADMAP.md) - Project timeline

### For Developers
- [API Reference](docs/api/KERNEL_API.md) - Public kernel interfaces
- [GDT Specification](docs/design/GDT_SPEC.md) - Detailed GDT info
- [IDT Specification](docs/design/IDT_SPEC.md) - Detailed IDT info

### For Contributors
- [Contributing Guide](docs/CONTRIBUTING.md) - How to contribute

## 📄 License

SpringOS is provided as-is for educational purposes. See LICENSE file for details.

## 🎓 Learning Outcomes

This project teaches:
- ✅ x86_64 CPU architecture
- ✅ Kernel design principles
- ✅ Interrupt handling
- ✅ Memory management
- ✅ Rust for bare-metal programming
- ✅ System design patterns

## 🔍 What's Next?

**Phase 2 (This Sprint):**
- [ ] Timer interrupt setup
- [ ] Preemptive scheduler
- [ ] Context switching

**Phase 3 (Next Sprint):**
- [ ] User mode support
- [ ] System calls
- [ ] Multi-process execution

**Phase 4 (Following Sprint):**
- [ ] Device drivers
- [ ] Filesystem support
- [ ] Complete bootable OS

## 📞 Contact

For questions about the project, check the documentation first, then:
- Read [ROADMAP.md](docs/ROADMAP.md) for timeline questions
- Check [FAQ](docs/README.md) (if available)
- Open an issue on GitHub

---

**SpringOS: Learning OS development, one interrupt at a time.** 🎯

Last Updated: June 4, 2026
