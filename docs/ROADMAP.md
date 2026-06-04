# SpringOS Roadmap

Dokumentasi lengkap dari semua fitur yang akan dikembangkan beserta timeline.

## Versioning

- **v0.1.x** - Foundation Phase (Phase 1-2)
- **v0.2.x** - Privilege & Process Phase (Phase 3)
- **v0.3.x** - I/O & Driver Phase (Phase 4)
- **v1.0.0** - Beta Release

## Phase 1: Foundation ✅ COMPLETE

**Status:** Complete (June 4, 2026)

### Completed Features
- [x] GDT (Global Descriptor Table)
- [x] IDT (Interrupt Descriptor Table) - Structure only
- [x] Basic Physical Memory Allocator
- [x] Page Table Structures
- [x] Boot sequence & Kernel entry
- [x] Serial port output

### Deliverables
- Kernel compiles and boots successfully
- GDT with Ring 0/3 segments
- IDT ready for handlers
- ~500 LOC of kernel code

---

## Phase 2: Core Features (IN PROGRESS)

**Estimated Duration:** 1-2 weeks  
**Target Completion:** June 11-18, 2026

### Exception Handling
- [ ] Exception wrapper functions (asm stubs)
- [ ] Unified exception handler
- [ ] Exception dispatcher
- [ ] Stack trace generation
- [ ] Panic handling improvements

**Files:**
- `kernel/src/arch/x86_64/exceptions.rs` (new)

### Interrupt Controller Support
- [ ] PIC (8259) initialization
- [ ] IRQ remapping (32-47)
- [ ] IRQ enable/disable functions
- [ ] Spurious interrupt handling

**Files:**
- `kernel/src/arch/x86_64/pic.rs` (new)

### Timer & Scheduler Foundation
- [ ] Timer interrupt handler
- [ ] Preemptive scheduling infrastructure
- [ ] Task/Process structure
- [ ] Round-robin scheduler (basic)
- [ ] Context switching implementation

**Files:**
- `kernel/src/arch/x86_64/timer.rs` (new)
- `kernel/src/task/mod.rs` (new)
- `kernel/src/task/scheduler.rs` (new)
- `kernel/src/task/context.rs` (new)

### VGA/Console Output
- [ ] VGA text mode driver
- [ ] Print macro for kernel logging
- [ ] Color support

**Files:**
- `kernel/src/drivers/vga.rs` (new)
- `kernel/src/drivers/mod.rs` (new)

### Deliverables
- Kernel handles CPU exceptions gracefully
- Hardware interrupts functional
- Timer-based preemptive scheduling
- Better debugging output

---

## Phase 3: Privilege & Process Management

**Estimated Duration:** 2-3 weeks  
**Target Completion:** June 25 - July 2, 2026

### User Mode Support
- [ ] TSS (Task State Segment) setup
- [ ] Ring 3 mode switching
- [ ] User mode kernel entry
- [ ] Privilege escalation handling

### System Calls
- [ ] System call dispatcher (INT 0x80)
- [ ] Basic system calls:
  - `exit()` - Process termination
  - `write()` - Output to console
  - `read()` - Input from console
  - `fork()` - Process creation
  - `exec()` - Program execution

### Virtual Memory
- [ ] Page table management
- [ ] Memory isolation per process
- [ ] Page fault handling
- [ ] Demand paging (basic)

### Process Management
- [ ] Process table
- [ ] Process states (running, ready, blocked, terminated)
- [ ] Process creation & termination
- [ ] Signal handling (basic)

**Files:**
- `kernel/src/task/process.rs` (new)
- `kernel/src/syscall/mod.rs` (new)
- `kernel/src/memory/paging.rs` (new)

### Deliverables
- User-mode applications can run
- Multiple processes can execute
- Basic system calls functional

---

## Phase 4: I/O & Device Drivers

**Estimated Duration:** 3-4 weeks  
**Target Completion:** July 16-23, 2026

### Storage & Filesystem
- [ ] IDE/SATA disk driver
- [ ] Filesystem abstraction layer
- [ ] FAT32 filesystem support
- [ ] File operations (open, read, write, close)
- [ ] Directory support

### Input Devices
- [ ] Keyboard driver
- [ ] Input buffering
- [ ] Keyboard interrupt handling

### System Calls Extension
- [ ] File I/O syscalls:
  - `open()`, `close()`
  - `read()`, `write()`
  - `lseek()`
  - `stat()`, `fstat()`

### Deliverables
- Can read/write files on disk
- Multi-threaded file access
- Filesystem operations functional

---

## Phase 5: Advanced Features (Post v1.0)

### Networking
- [ ] Network drivers (Ethernet)
- [ ] TCP/IP stack
- [ ] Socket programming

### Dynamic Library Loading
- [ ] Shared library support
- [ ] Dynamic linking loader

### Memory Protection
- [ ] Memory protection unit integration
- [ ] Address space randomization (ASLR)

### Security
- [ ] Process capability system
- [ ] User/group management
- [ ] File permissions

---

## Known Limitations & Trade-offs

### Current
1. **Single CPU Support** - No SMP/multicore support yet
2. **Limited Exception Handling** - Basic handlers only
3. **Simple Scheduler** - No priority levels
4. **No Virtual Filesystem** - Direct filesystem access only
5. **No Memory Protection** - Flat memory model initially

### Phase 2
- Will add preemptive multitasking
- Better exception handling
- Still single CPU

### Phase 3
- Memory isolation per process (via page tables)
- Ring 0/3 separation enforcement
- Still single CPU

### Phase 4+
- SMP support
- Advanced filesystem features
- Networking stack

---

## Testing Strategy

### Phase 1
- [x] Compilation test
- [x] Boot test (kernel enters)

### Phase 2
- [ ] Exception handler test
- [ ] Interrupt delivery test
- [ ] Timer accuracy test
- [ ] Context switching test

### Phase 3
- [ ] User mode transition test
- [ ] System call test
- [ ] Multi-process test
- [ ] Memory isolation test

### Phase 4
- [ ] File I/O test
- [ ] Filesystem test
- [ ] Input/output test

---

## Success Criteria per Phase

### Phase 1
- ✅ Kernel boots without errors
- ✅ GDT properly loaded
- ✅ IDT structure ready
- ✅ Code compiles with no errors

### Phase 2
- [ ] Can receive and handle all CPU exceptions
- [ ] Timer interrupt working
- [ ] Multiple tasks can run
- [ ] Switching visible in output

### Phase 3
- [ ] User programs can execute
- [ ] System calls functional
- [ ] Multiple processes isolated
- [ ] Process termination works

### Phase 4
- [ ] Files readable/writable
- [ ] Directory navigation works
- [ ] Keyboard input captured
- [ ] Disk I/O functional

---

## Contributing Areas

### For Contributors
1. **Drivers** - Implement more device drivers
2. **Filesystem** - Add ext2, ext4 support
3. **Networking** - Implement network stack
4. **Tools** - Build bootloader, utilities
5. **Documentation** - Expand and clarify docs

### Priority Needs
1. Exception handler implementations
2. Timer interrupt setup
3. Context switching code
4. VGA driver

---

## Milestones

| Milestone | Target Date | Status |
|-----------|------------|--------|
| Phase 1 Complete | June 4, 2026 | ✅ Done |
| Phase 2 Complete | June 18, 2026 | 🔄 In Progress |
| Phase 3 Complete | July 2, 2026 | ⏳ Upcoming |
| Phase 4 Complete | July 23, 2026 | ⏳ Upcoming |
| v1.0 Beta Release | August 1, 2026 | ⏳ Planned |

---

## References

- [OSDev Tutorials](https://wiki.osdev.org/Main_Page)
- [Linux Kernel Development](https://www.kernel.org/doc/html/latest/)
- [Xv6 OS](https://github.com/mit-pdos/xv6-public)
- [Intel SDM](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)
