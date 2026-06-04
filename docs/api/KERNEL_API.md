# API Reference - Kernel Core

## Arch Module (x86_64)

### GDT Functions

```rust
pub struct GlobalDescriptorTable
pub struct GdtPointer

impl GlobalDescriptorTable {
    pub fn new() -> Self
    pub fn get_pointer(&self) -> GdtPointer
}

pub unsafe fn load_gdt(pointer: &GdtPointer)
```

**Example:**
```rust
unsafe {
    let gdt = GlobalDescriptorTable::new();
    let pointer = gdt.get_pointer();
    load_gdt(&pointer);
}
```

### IDT Functions

```rust
pub struct InterruptDescriptorTable
pub struct IdtEntry
pub struct IdtPointer

impl InterruptDescriptorTable {
    pub const fn new() -> Self
    pub fn get_pointer(&self) -> IdtPointer
}

impl IdtEntry {
    pub const fn null() -> Self
    pub fn set_handler(&mut self, handler: u64, selector: u16, flags: u8)
}

pub unsafe fn load_idt(idt_pointer: &IdtPointer)
pub unsafe fn enable_interrupts()
pub unsafe fn disable_interrupts()
```

**Example:**
```rust
unsafe {
    let mut idt = InterruptDescriptorTable::new();
    
    // Setup exception handler
    idt.entries[0].set_handler(
        divide_by_zero_handler as u64,
        0x8,           // Kernel code segment
        0b1000_1110    // Interrupt gate, Present
    );
    
    load_idt(&idt.get_pointer());
    enable_interrupts();
}
```

### Exception Definitions

```rust
pub mod exceptions {
    pub const DIVIDE_BY_ZERO: usize = 0;
    pub const DEBUG: usize = 1;
    pub const NMI: usize = 2;
    pub const BREAKPOINT: usize = 3;
    pub const OVERFLOW: usize = 4;
    pub const BOUND_RANGE_EXCEEDED: usize = 5;
    pub const INVALID_OPCODE: usize = 6;
    pub const DEVICE_NOT_AVAILABLE: usize = 7;
    pub const DOUBLE_FAULT: usize = 8;
    pub const INVALID_TSS: usize = 10;
    pub const SEGMENT_NOT_PRESENT: usize = 11;
    pub const STACK_SEGMENT_FAULT: usize = 12;
    pub const GENERAL_PROTECTION_FAULT: usize = 13;
    pub const PAGE_FAULT: usize = 14;
    pub const X87_FLOATING_POINT: usize = 16;
    pub const ALIGNMENT_CHECK: usize = 17;
    pub const MACHINE_CHECK: usize = 18;
    pub const SIMD_FLOATING_POINT: usize = 19;
    pub const VIRTUALIZATION: usize = 20;
}
```

## Memory Module

### Physical Memory Allocator

```rust
pub struct PhysicalMemoryAllocator

impl PhysicalMemoryAllocator {
    /// Inisialisasi allocator
    /// 
    /// # Arguments
    /// * `start` - Alamat mulai physical memory
    /// * `size` - Total ukuran dalam bytes
    pub unsafe fn new(start: u64, size: u64) -> Self
    
    /// Allocate satu 4KB frame
    pub unsafe fn allocate_frame(&mut self) -> Option<u64>
    
    /// Free satu 4KB frame
    pub unsafe fn free_frame(&mut self, frame_addr: u64)
    
    pub fn get_total_frames(&self) -> usize
    pub fn get_bitmap_size(&self) -> usize
}
```

**Example:**
```rust
unsafe {
    let mut allocator = PhysicalMemoryAllocator::new(0x1000_0000, 0x1000_0000);
    
    if let Some(frame) = allocator.allocate_frame() {
        println!("Allocated frame at: 0x{:x}", frame);
    }
}
```

### Kernel Heap

```rust
pub struct KernelHeap

impl KernelHeap {
    pub const fn new(start: u64, size: u64) -> Self
    pub fn allocate(&mut self, size: usize, align: usize) -> Option<*mut u8>
    pub fn remaining(&self) -> u64
}
```

### Page Table

```rust
pub struct PageTable

impl PageTable {
    pub const fn new() -> Self
    pub fn set_entry(&mut self, index: usize, value: u64)
    pub fn get_entry(&self, index: usize) -> u64
}

pub mod page_flags {
    pub const PRESENT: u64;
    pub const WRITE: u64;
    pub const USER: u64;
    pub const WRITE_THROUGH: u64;
    pub const CACHE_DISABLE: u64;
    pub const ACCESSED: u64;
    pub const DIRTY: u64;
    pub const HUGE_PAGE: u64;
    pub const GLOBAL: u64;
    pub const EXECUTE_DISABLE: u64;
}
```

**Example:**
```rust
let mut page_table = PageTable::new();
page_table.set_entry(0, 0x1000 | page_flags::PRESENT | page_flags::WRITE);
```

## I/O Functions

```rust
pub fn outb(port: u16, value: u8)
pub fn print_serial(s: &str)
```

**Example:**
```rust
// Output 0xAA to port 0x80 (debugging port)
outb(0x80, 0xAA);

// Print to serial
print_serial("Hello, kernel!\n");
```

---

## Type Signatures

### Segment Descriptor

```rust
pub struct SegmentDescriptor {
    limit_low: u16,      // Bits 0-15
    base_low: u16,       // Bits 16-31
    base_mid: u8,        // Bits 32-39
    access: u8,          // Bits 40-47
    limit_and_flags: u8, // Bits 48-55
    base_high: u8,       // Bits 56-63
}
```

### IDT Entry

```rust
pub struct IdtEntry {
    pointer_low: u16,      // Offset 0:15
    gdt_selector: u16,     // Segment selector
    ist: u8,               // IST index
    flags: u8,             // Type and attributes
    pointer_mid: u16,      // Offset 16:31
    pointer_high: u32,     // Offset 32:63
    reserved: u32,         // Must be 0
}
```

---

## Phase 2+ (Planned APIs)

### Exception Handlers (Phase 2)
```rust
pub unsafe fn init_exceptions()
pub extern "C" fn exception_handler(error_code: u64)
```

### Timer Interrupt (Phase 2)
```rust
pub unsafe fn init_timer()
pub fn set_timer_frequency(hz: u32)
```

### Task/Process (Phase 3+)
```rust
pub struct Task { }
pub struct Process { }
pub unsafe fn context_switch(from: &mut Task, to: &Task)
pub fn yield_cpu()
```

### System Calls (Phase 3+)
```rust
pub unsafe fn init_syscalls()
pub extern "C" fn syscall_handler(number: u64, args: &[u64; 6]) -> u64
```

---

## Safety Requirements

### When Using `unsafe`

- Always document SAFETY requirements
- Verify preconditions before calling
- Example:
  ```rust
  // SAFETY: The page table is guaranteed to be valid and
  // properly aligned because it was just allocated
  unsafe { page_table.set_entry(0, address) }
  ```

### Protected Resources

- GDT: Can only be loaded once per CPU
- IDT: Can only be loaded once per CPU
- Page tables: Must be properly aligned
- Physical memory: No double allocation

---

## Examples

### Setup GDT and IDT

```rust
unsafe {
    // Initialize GDT
    let gdt = GlobalDescriptorTable::new();
    load_gdt(&gdt.get_pointer());
    
    // Initialize IDT
    let mut idt = InterruptDescriptorTable::new();
    load_idt(&idt.get_pointer());
    
    // Enable interrupts
    enable_interrupts();
}
```

### Allocate Physical Memory

```rust
unsafe {
    let mut allocator = PhysicalMemoryAllocator::new(0x1000_0000, 0x1000);
    
    match allocator.allocate_frame() {
        Some(frame) => print_serial("Allocated frame\n"),
        None => print_serial("Out of memory\n"),
    }
}
```

---

For more examples, see the kernel source code and test files.
