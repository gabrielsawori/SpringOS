# IDT (Interrupt Descriptor Table) Specification

## Overview

IDT adalah tabel yang mendefinisikan interrupt dan exception handlers. Digunakan untuk:
- CPU exception handling (divide by zero, page fault, dll)
- Hardware interrupt handling (timer, keyboard, dll)
- Software interrupt handling (system calls)

## IDT Structure

```
┌─────────────────────────────────────┐
│  0: Divide by Zero Exception        │
├─────────────────────────────────────┤
│  1: Debug Exception                 │
├─────────────────────────────────────┤
│  ...                                │
├─────────────────────────────────────┤
│  14: Page Fault Exception           │
├─────────────────────────────────────┤
│  ...                                │
├─────────────────────────────────────┤
│  31: Floating Point Exception       │
├─────────────────────────────────────┤
│  32-47: Hardware IRQs               │
│         (Remapped from PIC)         │
├─────────────────────────────────────┤
│  48+: Software Interrupts           │
│       (System Calls, etc)           │
├─────────────────────────────────────┤
│  255: Reserved                      │
└─────────────────────────────────────┘
```

## IDT Entry Format (Interrupt Gate)

```
Bytes 0-15:
┌──────────────────────────────────────┐
│ Bits 0-15   │ Bits 16-31             │
│ Offset 0:15 │ Segment Selector       │
└──────────────────────────────────────┘

Bytes 16-31:
┌──────────────────────────────────────┐
│ Bits 32-39  │ Bits 40-47 │ Bits 48-63│
│ IST+Flags   │ Type       │ Offset... │
└──────────────────────────────────────┘

Bytes 32-63:
┌──────────────────────────────────────┐
│ Bits 64-95  │ Bits 96-127            │
│ Offset 32:63│ Reserved (0)           │
└──────────────────────────────────────┘
```

### Flags Byte Format

```
Bit 7:     Present (1 = valid entry)
Bits 6-5:  Privilege Level (00=Ring0, 11=Ring3)
Bit 4:     0 (Reserved)
Bits 3-0:  Gate Type:
           0xE = Interrupt Gate
           0xF = Trap Gate
```

## CPU Exceptions (0-31)

```
Vector  Name                        Error Code  Handler Type
──────  ──────────────────────────  ──────────  ────────────
0       Divide by Zero              No          Fault
1       Debug                       No          Fault/Trap
2       NMI                         No          Interrupt
3       Breakpoint                  No          Trap
4       Overflow                    No          Trap
5       Bound Range Exceeded        No          Fault
6       Invalid Opcode              No          Fault
7       Device Not Available        No          Fault
8       Double Fault                Yes (0)     Abort
9       Coprocessor Segment Overrun No          Fault
10      Invalid TSS                 Yes         Fault
11      Segment Not Present         Yes         Fault
12      Stack Segment Fault         Yes         Fault
13      General Protection Fault    Yes         Fault
14      Page Fault                  Yes         Fault
15      Reserved                    -           -
16      x87 Floating Point          No          Fault
17      Alignment Check             Yes         Fault
18      Machine Check               No          Abort
19      SIMD Floating Point         No          Fault
20      Virtualization              No          Fault
21-31   Reserved                    -           -
```

### Exception Handler Types

**Fault:** CPU state before faulting instruction is saved  
**Trap:** CPU state after instruction is saved  
**Abort:** Cannot determine precise location

## Hardware Interrupts (32-47)

Mapped oleh PIC (8259 Programmable Interrupt Controller):

```
IRQ  Source                      Vector (after remap)
───  ──────────────────────────  ────────────────────
0    Timer (PIT)                 32
1    Keyboard                    33
2    Cascaded Controller         34
3    Serial Port 2               35
4    Serial Port 1               36
5    Parallel Port 2             37
6    Floppy Disk                 38
7    Parallel Port 1             39
8    RTC                         40
9-15 Various peripherals         41-47
```

## Current Implementation

Located in: `kernel/src/arch/x86_64/idt.rs`

```rust
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IdtEntry {
    pointer_low: u16,
    gdt_selector: u16,
    ist: u8,
    flags: u8,
    pointer_mid: u16,
    pointer_high: u32,
    reserved: u32,
}

pub struct InterruptDescriptorTable {
    entries: [IdtEntry; 256],
}
```

## IDT Loading

```rust
pub unsafe fn load_idt(idt_pointer: &IdtPointer) {
    asm!("lidt [{}]", in(reg) idt_pointer);
}

pub unsafe fn enable_interrupts() {
    asm!("sti");
}

pub unsafe fn disable_interrupts() {
    asm!("cli");
}
```

## Exception Handler Structure

Phase 2 akan mengimplementasikan exception handlers dengan format:

```rust
#[no_mangle]
extern "C" fn exception_handler(frame: &ExceptionFrame) {
    // Handle exception
    // Frame contains: RIP, CS, EFLAGS, RSP, SS (ring change)
}
```

### ExceptionFrame (Stack on Exception)

```
Ring 0 → Ring 0:
┌─────────────────┐
│ RIP             │
│ CS              │
│ EFLAGS          │
└─────────────────┘

Ring 3 → Ring 0:
┌─────────────────┐
│ SS (Ring 3)     │
│ RSP (Ring 3)    │
│ EFLAGS          │
│ CS (Ring 3)     │
│ RIP (Ring 3)    │
└─────────────────┘

With Error Code:
├─────────────────┤
│ Error Code      │ (some exceptions)
└─────────────────┘
```

## Interrupt Descriptor Types

### Interrupt Gate
- Disables interrupts (clears IF flag)
- Used for: Hardware interrupts, most exceptions
- CPU executes: CLI (clear interrupt flag)

### Trap Gate
- Preserves interrupt state
- Used for: Breakpoint, overflow
- CPU does NOT execute CLI

## Specifications

### Total Entries: 256
- 0-31: CPU Exceptions
- 32-47: Hardware IRQs (PIC remapped)
- 48-127: Available for software interrupts
- 128: System call vector (INT 0x80)
- 129-255: Available

### Alignment: 16 bytes (cache-optimized)

### Privilege Levels
- **Ring 0 only:** Most CPU exceptions, hardware IRQs
- **Ring 3 accessible:** Breakpoint (Vector 3), Overflow (Vector 4)
- **System call:** Usually INT 0x80 (Vector 128) - Ring 3 accessible

## Phase 2 Implementation Plan

1. Exception wrapper functions (asm stubs untuk semua 32 exceptions)
2. Unified exception handler (Rust function)
3. Exception dispatcher (routing ke specific handlers)
4. PIC initialization dan IRQ remapping
5. Timer interrupt handler
6. Keyboard interrupt handler

## References

- Intel 64 and IA-32 Architectures Software Developer Manuals
- x86-64 Instruction Set Architecture
- OSDev IDT Documentation
