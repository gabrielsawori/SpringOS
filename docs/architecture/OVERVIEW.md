# Architecture Overview

## System Architecture

SpringOS menggunakan arsitektur x86_64 dengan privilege level separation antara kernel dan user mode.

```
┌─────────────────────────────────────────┐
│         User Applications (Ring 3)      │
├─────────────────────────────────────────┤
│              System Call Interface      │
├─────────────────────────────────────────┤
│    Kernel (Ring 0) - Supervisor Mode    │
├─────────────────────────────────────────┤
│  Core Components:                       │
│  • Memory Management (Paging)           │
│  • Process/Task Management              │
│  • Interrupt & Exception Handling       │
│  • Device Drivers & I/O                 │
├─────────────────────────────────────────┤
│      x86_64 Hardware Abstraction        │
│  • GDT (Segmentation)                   │
│  • IDT (Interrupt/Exception)            │
│  • Page Tables (Virtual Memory)         │
│  • TSS (Task State Segment)             │
└─────────────────────────────────────────┘
```

## Boot Sequence

```
1. Bootloader (Limine) 
   ↓
2. Kernel Entry (_start)
   ↓
3. GDT Initialization
   ↓
4. IDT Initialization
   ↓
5. Memory Management Setup
   ↓
6. Exception Handlers Setup
   ↓
7. Timer/Scheduler Initialization
   ↓
8. Enter Main Kernel Loop
```

## Memory Layout

```
┌──────────────────────────────────────┐
│      Higher Half Kernel Space        │ 0xFFFF_FFFF_FFFF_FFFF
│      (Canonical Addresses)           │
├──────────────────────────────────────┤
│      Kernel Code & Data              │ 0xFFFF_8000_0000_0000
├──────────────────────────────────────┤
│      (Unmapped - Guard Pages)        │
├──────────────────────────────────────┤
│      User Space                      │ 0x0000_7FFF_FFFF_FFFF
│      (Applications & Data)           │
├──────────────────────────────────────┤
│      (Reserved by System)            │ 0x0000_0000_0000_0000
└──────────────────────────────────────┘
```

## Module Structure

```
kernel/src/
├── main.rs                    # Kernel entry point
├── arch/
│   ├── mod.rs                 # Architecture traits & init
│   └── x86_64/
│       ├── mod.rs             # x86_64 specific initialization
│       ├── gdt.rs             # Segment table management
│       ├── idt.rs             # Interrupt table management
│       ├── interrupts.rs       # Exception/IRQ handlers (Phase 2)
│       ├── timer.rs           # Timer interrupt handler (Phase 2)
│       └── task.rs            # Task switching (Phase 2)
│
├── memory/
│   ├── mod.rs                 # Memory management subsystem
│   ├── physical.rs            # Physical memory allocator
│   ├── virtual.rs             # Virtual memory & paging
│   └── heap.rs                # Kernel heap allocator
│
├── task/
│   ├── mod.rs                 # Task/process management
│   ├── scheduler.rs           # Task scheduler
│   └── context.rs             # Task context structures
│
└── drivers/                   # Device drivers (Phase 4)
    ├── serial.rs              # Serial port driver
    ├── vga.rs                 # VGA display driver
    └── disk.rs                # Disk driver (Phase 4)
```

## Key Design Decisions

### 1. **Higher-Half Kernel**
- Kernel mapped di high addresses (0xFFFF_8000_0000_0000)
- User space di lower addresses
- Memungkinkan full 2GB user space per process

### 2. **Ring 0/3 Separation**
- GDT dengan separate kernel dan user segments
- TSS untuk context switching
- System calls via INT 0x80 atau SYSCALL

### 3. **Modular Architecture**
- Arch-specific code terpisah di `arch/`
- Generic interfaces untuk portability
- Clean abstraction layers

### 4. **Bitmap Physical Allocator**
- Simple dan efficient untuk Phase 1
- Upgrade path ke buddy allocator
- Track per-frame metadata

## Privilege Levels

| Level | Name | Purpose |
|-------|------|---------|
| Ring 0 | Kernel | Full hardware access, exception handling, I/O |
| Ring 1 | - | Unused |
| Ring 2 | - | Unused |
| Ring 3 | User | Applications, restricted hardware access |

## Interrupt Architecture

```
Hardware Interrupt → PIC/APIC → IDT Entry → Handler

Exception → CPU → IDT Entry → Handler

Software Interrupt (INT) → CPU → IDT Entry → Handler
```

- **0-31**: CPU Exceptions (Page Fault, GP Fault, dll)
- **32-47**: Hardware IRQs (Timer, Keyboard, dll)
- **48+**: Software Interrupts (System Calls, dll)

## Next Phases

**Phase 2**: Exception handlers, interrupt controllers, timer, context switching
**Phase 3**: User mode, system calls, process management
**Phase 4**: Device drivers, filesystem, networking
