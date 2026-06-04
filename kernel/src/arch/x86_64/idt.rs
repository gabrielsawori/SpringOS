// Interrupt Descriptor Table (IDT) - Mengatur exception dan interrupt handlers

use core::mem::size_of;

// Tipe handler untuk exceptions
pub type ExceptionHandler = extern "C" fn();
pub type ExceptionHandlerWithCode = extern "C" fn(u64);

// IDT Entry (Interrupt Gate Descriptor)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IdtEntry {
    pointer_low: u16,      // Offset 0:15 dari handler
    gdt_selector: u16,     // GDT selector untuk code segment
    ist: u8,               // IST (Interrupt Stack Table) index
    flags: u8,             // Type dan attributes
    pointer_mid: u16,      // Offset 16:31 dari handler
    pointer_high: u32,     // Offset 32:63 dari handler
    reserved: u32,         // Reserved, harus 0
}

impl IdtEntry {
    pub const fn null() -> Self {
        IdtEntry {
            pointer_low: 0,
            gdt_selector: 0,
            ist: 0,
            flags: 0,
            pointer_mid: 0,
            pointer_high: 0,
            reserved: 0,
        }
    }

    // Set handler function untuk interrupt/exception
    // selector: GDT selector (biasanya kernel code segment = 0x8)
    // flags: 
    //   - Bit 7: Present (1 = entry valid)
    //   - Bits 6-5: Privilege level (0 = Ring 0, 3 = Ring 3)
    //   - Bits 3-0: Gate type (0xE = Interrupt Gate, 0xF = Trap Gate)
    pub fn set_handler(&mut self, handler: u64, selector: u16, flags: u8) {
        self.pointer_low = (handler & 0xFFFF) as u16;
        self.pointer_mid = ((handler >> 16) & 0xFFFF) as u16;
        self.pointer_high = ((handler >> 32) & 0xFFFFFFFF) as u32;
        self.gdt_selector = selector;
        self.flags = flags | 0x80;  // Set present bit
        self.ist = 0;
        self.reserved = 0;
    }
}

// IDT Pointer untuk LIDT instruction
#[repr(C, packed)]
pub struct IdtPointer {
    pub size: u16,
    pub addr: u64,
}

// Array berisi 256 entries (satu untuk setiap interrupt/exception)
#[repr(align(16))]
pub struct InterruptDescriptorTable {
    pub entries: [IdtEntry; 256],
}

impl InterruptDescriptorTable {
    pub const fn new() -> Self {
        InterruptDescriptorTable {
            entries: [IdtEntry::null(); 256],
        }
    }

    pub fn get_pointer(&self) -> IdtPointer {
        IdtPointer {
            size: (size_of::<InterruptDescriptorTable>() - 1) as u16,
            addr: self as *const _ as u64,
        }
    }
}

// CPU Exceptions (0-31)
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
    pub const COPROCESSOR_SEGMENT_OVERRUN: usize = 9;
    pub const INVALID_TSS: usize = 10;
    pub const SEGMENT_NOT_PRESENT: usize = 11;
    pub const STACK_SEGMENT_FAULT: usize = 12;
    pub const GENERAL_PROTECTION_FAULT: usize = 13;
    pub const PAGE_FAULT: usize = 14;
    pub const RESERVED_1: usize = 15;
    pub const X87_FLOATING_POINT: usize = 16;
    pub const ALIGNMENT_CHECK: usize = 17;
    pub const MACHINE_CHECK: usize = 18;
    pub const SIMD_FLOATING_POINT: usize = 19;
    pub const VIRTUALIZATION: usize = 20;
}

// Load IDT menggunakan LIDT instruction
pub unsafe fn load_idt(idt_pointer: &IdtPointer) {
    core::arch::asm!(
        "lidt [{}]",
        in(reg) idt_pointer,
        options(nostack)
    );
}

// Helper untuk setup exception handlers di IDT
pub fn set_exception_handler(
    idt: &mut InterruptDescriptorTable,
    exception: usize,
    handler: u64,
) {
    if exception < 256 {
        // Flags: Present (0x80) | Ring 0 (0x00) | Interrupt Gate (0x0E)
        idt.entries[exception].set_handler(handler, 0x08, 0x80 | 0x0E);
    }
}

// Enable interrupts
pub unsafe fn enable_interrupts() {
    core::arch::asm!("sti");
}

// Disable interrupts
pub unsafe fn disable_interrupts() {
    core::arch::asm!("cli");
}
