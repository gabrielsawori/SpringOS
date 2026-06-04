// x86_64 Architecture Support

pub mod gdt;
pub mod idt;
pub mod exceptions;
pub mod pic;

pub use gdt::{GlobalDescriptorTable, load_gdt};
pub use idt::{InterruptDescriptorTable, load_idt, enable_interrupts, set_exception_handler};
pub use exceptions::{init_exceptions, divide_by_zero_handler, debug_handler, nmi_handler, breakpoint_handler, overflow_handler, bound_range_handler, invalid_opcode_handler, device_not_available_handler, double_fault_handler, coprocessor_segment_overrun_handler, invalid_tss_handler, segment_not_present_handler, stack_segment_fault_handler, general_protection_fault_handler, page_fault_handler, alignment_check_handler, machine_check_handler, x87_floating_point_handler, simd_floating_point_handler, virtualization_handler, reserved_exception_15_handler, reserved_exception_21_handler, reserved_exception_22_handler, reserved_exception_23_handler, reserved_exception_24_handler, reserved_exception_25_handler, reserved_exception_26_handler, reserved_exception_27_handler, reserved_exception_28_handler, reserved_exception_29_handler, reserved_exception_30_handler, reserved_exception_31_handler};
pub use pic::{ProgrammableInterruptController, inb};

// Initialize GDT, IDT and Exception handlers
pub unsafe fn init() {
    // Initialize GDT
    let gdt = GlobalDescriptorTable::new();
    let gdt_pointer = gdt.get_pointer();
    load_gdt(&gdt_pointer);

    // Initialize IDT dengan semua exception handlers
    let mut idt = InterruptDescriptorTable::new();
    
    // Register semua exception handlers
    set_exception_handler(&mut idt, idt::exceptions::DIVIDE_BY_ZERO, divide_by_zero_handler as u64);
    set_exception_handler(&mut idt, idt::exceptions::DEBUG, debug_handler as u64);
    set_exception_handler(&mut idt, idt::exceptions::NMI, nmi_handler as u64);
    set_exception_handler(&mut idt, idt::exceptions::BREAKPOINT, breakpoint_handler as u64);
    set_exception_handler(&mut idt, idt::exceptions::OVERFLOW, overflow_handler as u64);
    set_exception_handler(&mut idt, idt::exceptions::BOUND_RANGE_EXCEEDED, bound_range_handler as u64);
    set_exception_handler(&mut idt, idt::exceptions::INVALID_OPCODE, invalid_opcode_handler as u64);
    set_exception_handler(&mut idt, idt::exceptions::DEVICE_NOT_AVAILABLE, device_not_available_handler as u64);
    set_exception_handler(&mut idt, idt::exceptions::DOUBLE_FAULT, double_fault_handler as u64);
    set_exception_handler(&mut idt, idt::exceptions::COPROCESSOR_SEGMENT_OVERRUN, coprocessor_segment_overrun_handler as u64);
    set_exception_handler(&mut idt, idt::exceptions::INVALID_TSS, invalid_tss_handler as u64);
    set_exception_handler(&mut idt, idt::exceptions::SEGMENT_NOT_PRESENT, segment_not_present_handler as u64);
    set_exception_handler(&mut idt, idt::exceptions::STACK_SEGMENT_FAULT, stack_segment_fault_handler as u64);
    set_exception_handler(&mut idt, idt::exceptions::GENERAL_PROTECTION_FAULT, general_protection_fault_handler as u64);
    set_exception_handler(&mut idt, idt::exceptions::PAGE_FAULT, page_fault_handler as u64);
    set_exception_handler(&mut idt, 15, reserved_exception_15_handler as u64);
    set_exception_handler(&mut idt, idt::exceptions::X87_FLOATING_POINT, x87_floating_point_handler as u64);
    set_exception_handler(&mut idt, idt::exceptions::ALIGNMENT_CHECK, alignment_check_handler as u64);
    set_exception_handler(&mut idt, idt::exceptions::MACHINE_CHECK, machine_check_handler as u64);
    set_exception_handler(&mut idt, idt::exceptions::SIMD_FLOATING_POINT, simd_floating_point_handler as u64);
    set_exception_handler(&mut idt, idt::exceptions::VIRTUALIZATION, virtualization_handler as u64);
    
    // Register reserved exception handlers (21-31)
    set_exception_handler(&mut idt, 21, reserved_exception_21_handler as u64);
    set_exception_handler(&mut idt, 22, reserved_exception_22_handler as u64);
    set_exception_handler(&mut idt, 23, reserved_exception_23_handler as u64);
    set_exception_handler(&mut idt, 24, reserved_exception_24_handler as u64);
    set_exception_handler(&mut idt, 25, reserved_exception_25_handler as u64);
    set_exception_handler(&mut idt, 26, reserved_exception_26_handler as u64);
    set_exception_handler(&mut idt, 27, reserved_exception_27_handler as u64);
    set_exception_handler(&mut idt, 28, reserved_exception_28_handler as u64);
    set_exception_handler(&mut idt, 29, reserved_exception_29_handler as u64);
    set_exception_handler(&mut idt, 30, reserved_exception_30_handler as u64);
    set_exception_handler(&mut idt, 31, reserved_exception_31_handler as u64);
    
    let idt_pointer = idt.get_pointer();
    load_idt(&idt_pointer);
    
    // Initialize exception handlers
    init_exceptions();
    
    // Initialize PIC for hardware interrupts
    ProgrammableInterruptController::init();
    
    // Enable interrupts
    enable_interrupts();
}
