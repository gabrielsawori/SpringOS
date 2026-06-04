# GDT (Global Descriptor Table) Specification

## Overview

GDT adalah tabel yang mendefinisikan memory segments dan privilege levels di x86_64. Digunakan untuk:
- Segmentasi memory
- Privilege level management (Ring 0-3)
- Task switching (via TSS)

## GDT Structure

```
┌────────────────────────────────────────┐
│ Entry 0: NULL Descriptor               │
├────────────────────────────────────────┤
│ Entry 1: Kernel Code Segment (Ring 0)  │
├────────────────────────────────────────┤
│ Entry 2: Kernel Data Segment (Ring 0)  │
├────────────────────────────────────────┤
│ Entry 3: User Code Segment (Ring 3)    │
├────────────────────────────────────────┤
│ Entry 4: User Data Segment (Ring 3)    │
├────────────────────────────────────────┤
│ Entry 5: TSS (Task State Segment)      │
│         (Phase 2 - untuk multitasking) │
└────────────────────────────────────────┘
```

## Segment Descriptor Format (64-bit)

```
Bytes 0-7:
┌─────────────────────────────────────────────┐
│ Bits 0-15   │ Bits 16-39  │ Bits 40-47     │
│ Limit Low   │ Base Low    │ Base Mid       │
└─────────────────────────────────────────────┘

Bytes 8-15:
┌──────────────────────────────────────────────┐
│ Bits 48-55  │ Bits 56-63  │ Bits 64-79      │
│ Access Byte │ Limit+Flags │ Base High       │
└──────────────────────────────────────────────┘
```

### Access Byte Format

```
Bit 7:   Present (1 = valid entry)
Bits 6-5: Privilege Level (00=Ring0, 11=Ring3)
Bit 4:   Descriptor Type (1=Code/Data, 0=System)
Bit 3:   Code/Data (1=Code, 0=Data)
Bit 2:   Direction/Conforming
Bit 1:   Readable/Writable
Bit 0:   Accessed
```

### Flags Format

```
Bit 7:   Granularity (0=1byte, 1=4KB)
Bit 6:   Size (0=16-bit, 1=32/64-bit)
Bit 5:   Long Mode (1=64-bit code)
Bits 4-0: Upper limit bits (4-0)
```

## Current Implementation

Located in: `kernel/src/arch/x86_64/gdt.rs`

```rust
pub struct GlobalDescriptorTable {
    entries: [SegmentDescriptor; 5],
}
```

### Entries Breakdown

| Index | Type | Privilege | Selector | Purpose |
|-------|------|-----------|----------|---------|
| 0 | NULL | - | 0x00 | Required null entry |
| 1 | Code | Ring 0 | 0x08 | Kernel code execution |
| 2 | Data | Ring 0 | 0x10 | Kernel data access |
| 3 | Code | Ring 3 | 0x18 | User code execution |
| 4 | Data | Ring 3 | 0x20 | User data access |

**Note:** TSS entry akan ditambahkan di Phase 2 untuk multitasking.

## GDT Loading

```rust
pub unsafe fn load_gdt(pointer: &GdtPointer) {
    // LGDT instruction
    asm!("lgdt [{}]", in(reg) pointer);
    
    // Reload data segments
    asm!("mov ax, 0x10; mov ds, ax; ...");
    
    // Reload code segment via far jump
    asm!("push 0x8; lea rax, [rip + 2f]; push rax; retfq; 2:");
}
```

## Specifications

### Segment Limits
- Limit: 0xFFFFF (1MB * 4KB = 4GB)
- Granularity: 4KB pages
- **Result:** Full 64-bit addressable space

### Base Address
- Code/Data segments: 0x00000000 (flat memory model)
- Same base for all segments in x86_64

### Privilege Levels
- **Ring 0 (Kernel):** Full hardware access
- **Ring 3 (User):** Restricted access, cannot:
  - Execute privileged instructions
  - Access I/O ports directly
  - Modify control registers
  - Access other process memory

## Future Extensions (Phase 3+)

### TSS (Task State Segment)
```
Selector: 0x28 (Index 5)
- For task switching during context switches
- Store kernel stack pointer for ring transitions
- Store interrupt stack table pointers
```

### LDT (Local Descriptor Table)
- Per-process segment tables
- For advanced privilege isolation
- Optional in modern OS

## References

- Intel x86_64 System V AMD64 ABI
- AMD64 Architecture Programmer's Manual
- OSDev GDT Documentation
