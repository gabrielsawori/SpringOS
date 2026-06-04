// Exception handler wrappers dan dispatcher
// Setiap exception memiliki wrapper asm yang menyimpan context ke stack

use crate::arch::x86_64::idt::exceptions;

// Exception frame yang di-push oleh CPU saat terjadi interrupt/exception
#[repr(C)]
pub struct ExceptionFrame {
    // Pushed by wrapper (jika perlu error code)
    pub error_code: u64,
    
    // Pushed oleh CPU (untuk ring 0 -> ring 0)
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    
    // Hanya jika ring 3 -> ring 0
    pub rsp: u64,
    pub ss: u64,
}

// Handler type untuk exception handlers
pub type ExceptionHandler = fn(&ExceptionFrame);

// Global exception handlers array
static mut EXCEPTION_HANDLERS: [Option<ExceptionHandler>; 32] = [None; 32];

/// Register custom exception handler
pub fn register_handler(exception: usize, handler: ExceptionHandler) {
    if exception < 32 {
        unsafe {
            EXCEPTION_HANDLERS[exception] = Some(handler);
        }
    }
}

/// Dispatcher untuk exception - dipanggil oleh asm wrapper
#[no_mangle]
pub extern "C" fn dispatch_exception(exc_num: u64, frame: &ExceptionFrame) {
    let exception_num = exc_num as usize;
    
    // Try custom handler dulu
    unsafe {
        if let Some(handler) = EXCEPTION_HANDLERS[exception_num] {
            handler(frame);
            return;
        }
    }
    
    // Default handler
    default_exception_handler(exception_num, frame);
}

/// Default handler untuk exceptions yang tidak di-handle
fn default_exception_handler(exc_num: usize, _frame: &ExceptionFrame) {
    crate::print_serial("====================================\n");
    crate::print_serial("CPU EXCEPTION RECEIVED!\n");
    crate::print_serial("Exception: ");
    
    match exc_num {
        exceptions::DIVIDE_BY_ZERO => crate::print_serial("Divide by Zero (0)"),
        exceptions::DEBUG => crate::print_serial("Debug Exception (1)"),
        exceptions::NMI => crate::print_serial("NMI (2)"),
        exceptions::BREAKPOINT => crate::print_serial("Breakpoint (3)"),
        exceptions::OVERFLOW => crate::print_serial("Overflow (4)"),
        exceptions::BOUND_RANGE_EXCEEDED => crate::print_serial("Bound Range Exceeded (5)"),
        exceptions::INVALID_OPCODE => crate::print_serial("Invalid Opcode (6)"),
        exceptions::DEVICE_NOT_AVAILABLE => crate::print_serial("Device Not Available (7)"),
        exceptions::DOUBLE_FAULT => crate::print_serial("Double Fault (8)"),
        exceptions::COPROCESSOR_SEGMENT_OVERRUN => {
            crate::print_serial("Coprocessor Segment Overrun (9)")
        }
        exceptions::INVALID_TSS => crate::print_serial("Invalid TSS (10)"),
        exceptions::SEGMENT_NOT_PRESENT => crate::print_serial("Segment Not Present (11)"),
        exceptions::STACK_SEGMENT_FAULT => crate::print_serial("Stack Segment Fault (12)"),
        exceptions::GENERAL_PROTECTION_FAULT => crate::print_serial("General Protection Fault (13)"),
        exceptions::PAGE_FAULT => crate::print_serial("Page Fault (14)"),
        exceptions::X87_FLOATING_POINT => crate::print_serial("x87 Floating Point (16)"),
        exceptions::ALIGNMENT_CHECK => crate::print_serial("Alignment Check (17)"),
        exceptions::MACHINE_CHECK => crate::print_serial("Machine Check (18)"),
        exceptions::SIMD_FLOATING_POINT => crate::print_serial("SIMD Floating Point (19)"),
        exceptions::VIRTUALIZATION => crate::print_serial("Virtualization (20)"),
        _ => crate::print_serial("Unknown Exception"),
    }
    
    crate::print_serial("\n");
    crate::print_serial("RIP: 0x");
    // Would need hex printer here
    crate::print_serial("\n");
    crate::print_serial("====================================\n");
    
    // Hang sistem
    loop {}
}

/// Initialize exception handlers
pub unsafe fn init_exceptions() {
    // Register default handlers untuk exceptions penting
    
    // Double fault harus selalu di-handle karena critical
    // (Will be implemented in IDT setup)
    
    crate::print_serial("[*] Exception handlers initialized\n");
}

// ============================================================================
// ASSEMBLY STUBS untuk semua CPU exceptions
// ============================================================================
// Setiap exception memiliki wrapper yang:
// 1. Push error code (0 jika exception tidak punya error code)
// 2. Push exception number
// 3. Call dispatch_exception

// Define handler functions dengan proper linking
extern "C" {
    // Declare these handlers that will be defined in assembly in separate file
}

// Placeholder handler functions
// Actual implementations akan di-inline via macros

macro_rules! define_exception_handler {
    ($name:ident) => {
        pub fn $name() {
            // Stub - actual implementation in asm
        }
    };
}

// Define semua handler stubs
define_exception_handler!(divide_by_zero_handler);
define_exception_handler!(debug_handler);
define_exception_handler!(nmi_handler);
define_exception_handler!(breakpoint_handler);
define_exception_handler!(overflow_handler);
define_exception_handler!(bound_range_handler);
define_exception_handler!(invalid_opcode_handler);
define_exception_handler!(device_not_available_handler);
define_exception_handler!(double_fault_handler);
define_exception_handler!(coprocessor_segment_overrun_handler);
define_exception_handler!(invalid_tss_handler);
define_exception_handler!(segment_not_present_handler);
define_exception_handler!(stack_segment_fault_handler);
define_exception_handler!(general_protection_fault_handler);
define_exception_handler!(page_fault_handler);
define_exception_handler!(reserved_exception_15_handler);
define_exception_handler!(x87_floating_point_handler);
define_exception_handler!(alignment_check_handler);
define_exception_handler!(machine_check_handler);
define_exception_handler!(simd_floating_point_handler);
define_exception_handler!(virtualization_handler);
define_exception_handler!(reserved_exception_21_handler);
define_exception_handler!(reserved_exception_22_handler);
define_exception_handler!(reserved_exception_23_handler);
define_exception_handler!(reserved_exception_24_handler);
define_exception_handler!(reserved_exception_25_handler);
define_exception_handler!(reserved_exception_26_handler);
define_exception_handler!(reserved_exception_27_handler);
define_exception_handler!(reserved_exception_28_handler);
define_exception_handler!(reserved_exception_29_handler);
define_exception_handler!(reserved_exception_30_handler);
define_exception_handler!(reserved_exception_31_handler);
