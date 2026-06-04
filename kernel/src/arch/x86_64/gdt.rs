// Global Descriptor Table (GDT) - Mengatur memory segmentation dan privilege levels

use core::mem::size_of;

// Struktur untuk GDT entry (descriptor)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct SegmentDescriptor {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    limit_and_flags: u8,
    base_high: u8,
}

impl SegmentDescriptor {
    // Membuat null descriptor (unused)
    pub const fn null() -> Self {
        SegmentDescriptor {
            limit_low: 0,
            base_low: 0,
            base_mid: 0,
            access: 0,
            limit_and_flags: 0,
            base_high: 0,
        }
    }

    // Membuat segment descriptor
    // access_byte: Privilege level, descriptor type, dll
    // flags: Granularity, default operation size, dll
    pub const fn new(base: u32, limit: u32, access: u8, flags: u8) -> Self {
        SegmentDescriptor {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_mid: ((base >> 16) & 0xFF) as u8,
            access,
            limit_and_flags: (((limit >> 16) & 0x0F) as u8) | flags,
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }
}

// GDT Pointer - digunakan untuk LGDT instruction
#[repr(C, packed)]
pub struct GdtPointer {
    pub size: u16,  // Ukuran GDT - 1
    pub addr: u64,  // Address dari GDT
}

// GDT utama dengan entries
pub struct GlobalDescriptorTable {
    entries: [SegmentDescriptor; 5],
}

impl GlobalDescriptorTable {
    pub fn new() -> Self {
        let mut gdt = GlobalDescriptorTable {
            entries: [SegmentDescriptor::null(); 5],
        };

        // Entry 0: NULL descriptor (required)
        gdt.entries[0] = SegmentDescriptor::null();

        // Entry 1: Kernel Code Segment (Ring 0)
        // Access byte: 10011010
        // - Bit 7: Present (1)
        // - Bits 6-5: Privilege level (00 = Ring 0)
        // - Bit 4: Descriptor type (1 = Code/Data segment)
        // - Bit 3: Code segment (1)
        // - Bit 2: Conforming (0)
        // - Bit 1: Readable (1)
        // - Bit 0: Accessed (0, set by CPU)
        gdt.entries[1] = SegmentDescriptor::new(0, 0xFFFFF, 0b10011010, 0b1100);

        // Entry 2: Kernel Data Segment (Ring 0)
        // Access byte: 10010010
        // - Bit 7: Present (1)
        // - Bits 6-5: Privilege level (00 = Ring 0)
        // - Bit 4: Descriptor type (1 = Code/Data segment)
        // - Bit 3: Data segment (0)
        // - Bit 2: Expand down (0)
        // - Bit 1: Writable (1)
        // - Bit 0: Accessed (0)
        gdt.entries[2] = SegmentDescriptor::new(0, 0xFFFFF, 0b10010010, 0b1100);

        // Entry 3: User Code Segment (Ring 3)
        // Sama dengan kernel code tapi privilege level = 11 (Ring 3)
        gdt.entries[3] = SegmentDescriptor::new(0, 0xFFFFF, 0b11111010, 0b1100);

        // Entry 4: User Data Segment (Ring 3)
        // Sama dengan kernel data tapi privilege level = 11 (Ring 3)
        gdt.entries[4] = SegmentDescriptor::new(0, 0xFFFFF, 0b11110010, 0b1100);

        gdt
    }

    // Mendapatkan pointer untuk LGDT instruction
    pub fn get_pointer(&self) -> GdtPointer {
        GdtPointer {
            size: (size_of::<GlobalDescriptorTable>() - 1) as u16,
            addr: self as *const _ as u64,
        }
    }
}

// Load GDT menggunakan LGDT instruction
pub unsafe fn load_gdt(pointer: &GdtPointer) {
    core::arch::asm!(
        "lgdt [{}]",
        in(reg) pointer,
        options(nostack)
    );

    // Reload segment registers
    core::arch::asm!(
        "mov ax, 0x10",          // Kernel data segment (entry 2 * 8 = 0x10)
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        "mov ss, ax",
        options(nostack)
    );

    // Reload CS register menggunakan far jump
    core::arch::asm!(
        "push 0x8",              // Kernel code segment (entry 1 * 8 = 0x8)
        "lea rax, [rip + 2f]",
        "push rax",
        "retfq",
        "2:",
        options(nostack)
    );
}
