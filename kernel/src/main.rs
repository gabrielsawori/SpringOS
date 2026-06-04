#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

mod arch;
mod memory;

// Fungsi tingkat rendah untuk mengirim data 1-byte ke port I/O perangkat keras
#[inline]
pub fn outb(port: u16, value: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }
}

// Fungsi sederhana untuk mencetak teks ke Serial Port (COM1)
pub fn print_serial(s: &str) {
    for byte in s.bytes() {
        outb(0x3F8, byte); // 0x3F8 adalah alamat standar untuk COM1
    }
}

// Handler jika terjadi error fatal
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print_serial("====================================\n");
    print_serial("KERNEL PANIC!\n");
    if let Some(location) = info.location() {
        print_serial("File: ");
        print_serial(location.file());
        print_serial(" Line: ");
        // Simplified line printing
        print_serial("\n");
    }
    print_serial("====================================\n");
    loop {}
}

// Entry point kernel kita
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // CRITICAL: Output message IMMEDIATELY sebelum apapun
    // Jika ini tidak keluar, ada problem dengan serial port setup
    print_serial("KERNEL STARTING...\n");
    print_serial("Kernel entry point reached!\n");
    
    // Pesan boot
    print_serial("====================================\n");
    print_serial("Booting SpringOS (Phase 1)...\n");
    print_serial("====================================\n");

    // Initialize GDT and IDT
    unsafe {
        print_serial("[*] Initializing GDT...\n");
        arch::init();
    }
    
    print_serial("[+] GDT Initialized\n");
    print_serial("[+] IDT Initialized\n");
    print_serial("[+] PIC Initialized\n");
    print_serial("[+] Interrupts Enabled\n");

    // Success message
    print_serial("====================================\n");
    print_serial("[+] Halo Arsitek! SpringOS berhasil hidup!\n");
    print_serial("[+] Sistem berjalan stabil di arsitektur x86_64.\n");
    print_serial("[+] Phase 2 (Exception & Interrupt) Loaded\n");
    print_serial("====================================\n");
    print_serial("[+] Kernel running, waiting for interrupts...\n");

    // Tahan CPU agar sistem tidak mati
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}