// Programmable Interrupt Controller (8259)
// Mengatur hardware interrupts (IRQ 0-15)

use crate::outb;

// I/O port constants
const PIC_MASTER_COMMAND: u16 = 0x20;
const PIC_MASTER_DATA: u16 = 0x21;
const PIC_SLAVE_COMMAND: u16 = 0xA0;
const PIC_SLAVE_DATA: u16 = 0xA1;

// ICW (Initialization Command Words)
const ICW1_ICW4: u8 = 0x01;
const ICW1_INIT: u8 = 0x10;

// ICW4 flags
const ICW4_8086: u8 = 0x01;

// OCW1 - Interrupt Mask Register
const OCW1_MASK: u8 = 0xFF;

// OCW2 - End Of Interrupt
const OCW2_EOI: u8 = 0x20;

/// PIC configuration untuk SpringOS
/// Master PIC: IRQ 0-7 → Interrupt 32-39
/// Slave PIC: IRQ 8-15 → Interrupt 40-47
pub struct ProgrammableInterruptController;

impl ProgrammableInterruptController {
    /// Initialize kedua PIC (master dan slave)
    /// 
    /// # Safety
    /// Must be called only once during kernel initialization
    pub unsafe fn init() {
        // ICW1 - Start initialization
        Self::init_command(PIC_MASTER_COMMAND, ICW1_INIT | ICW1_ICW4);
        Self::init_command(PIC_SLAVE_COMMAND, ICW1_INIT | ICW1_ICW4);
        
        // ICW2 - Set interrupt vector offsets
        // Master: IRQ 0-7 → Interrupts 32-39
        Self::init_command(PIC_MASTER_DATA, 32);
        
        // Slave: IRQ 8-15 → Interrupts 40-47
        Self::init_command(PIC_SLAVE_DATA, 40);
        
        // ICW3 - Setup cascading
        // Master: Slave is on IRQ2
        Self::init_command(PIC_MASTER_DATA, 0x04);
        
        // Slave: Connected to master IRQ2
        Self::init_command(PIC_SLAVE_DATA, 0x02);
        
        // ICW4 - 8086 mode
        Self::init_command(PIC_MASTER_DATA, ICW4_8086);
        Self::init_command(PIC_SLAVE_DATA, ICW4_8086);
        
        // OCW1 - Unmask all IRQs
        // (Set mask register to 0 = all IRQs enabled)
        outb(PIC_MASTER_DATA, 0x00);
        outb(PIC_SLAVE_DATA, 0x00);
        
        crate::print_serial("[+] PIC Initialized\n");
        crate::print_serial("    Master: IRQ 0-7 -> Vectors 32-39\n");
        crate::print_serial("    Slave:  IRQ 8-15 -> Vectors 40-47\n");
    }
    
    /// Send End Of Interrupt (EOI) signal
    /// 
    /// # Arguments
    /// * `irq` - IRQ number (0-15)
    pub unsafe fn end_of_interrupt(irq: u8) {
        if irq >= 8 {
            // Slave IRQ - send EOI to both master and slave
            outb(PIC_SLAVE_COMMAND, OCW2_EOI);
        }
        // Always send EOI to master
        outb(PIC_MASTER_COMMAND, OCW2_EOI);
    }
    
    /// Mask (disable) specific IRQ
    /// 
    /// # Arguments
    /// * `irq` - IRQ number (0-15)
    pub unsafe fn mask_irq(irq: u8) {
        if irq < 8 {
            // Master PIC
            let mask = inb(PIC_MASTER_DATA);
            outb(PIC_MASTER_DATA, mask | (1 << irq));
        } else if irq < 16 {
            // Slave PIC
            let mask = inb(PIC_SLAVE_DATA);
            outb(PIC_SLAVE_DATA, mask | (1 << (irq - 8)));
        }
    }
    
    /// Unmask (enable) specific IRQ
    /// 
    /// # Arguments
    /// * `irq` - IRQ number (0-15)
    pub unsafe fn unmask_irq(irq: u8) {
        if irq < 8 {
            // Master PIC
            let mask = inb(PIC_MASTER_DATA);
            outb(PIC_MASTER_DATA, mask & !(1 << irq));
        } else if irq < 16 {
            // Slave PIC
            let mask = inb(PIC_SLAVE_DATA);
            outb(PIC_SLAVE_DATA, mask & !(1 << (irq - 8)));
        }
    }
    
    /// Disable all IRQs
    pub unsafe fn disable_all() {
        outb(PIC_MASTER_DATA, 0xFF);
        outb(PIC_SLAVE_DATA, 0xFF);
    }
    
    /// Helper untuk initialization command
    fn init_command(port: u16, value: u8) {
        unsafe {
            outb(port, value);
        }
    }
}

/// Read byte from I/O port
/// 
/// # Safety
/// Must only be used for valid I/O ports
pub unsafe fn inb(port: u16) -> u8 {
    let result: u8;
    core::arch::asm!(
        "in al, dx",
        out("al") result,
        in("dx") port,
        options(nomem, nostack, preserves_flags)
    );
    result
}

// IRQ definitions
pub mod irq {
    pub const TIMER: u8 = 0;
    pub const KEYBOARD: u8 = 1;
    pub const CASCADE: u8 = 2;
    pub const SERIAL_PORT_2: u8 = 3;
    pub const SERIAL_PORT_1: u8 = 4;
    pub const PARALLEL_PORT_2: u8 = 5;
    pub const FLOPPY_DISK: u8 = 6;
    pub const PARALLEL_PORT_1: u8 = 7;
    pub const RTC: u8 = 8;
    pub const PERIPHERAL_1: u8 = 9;
    pub const PERIPHERAL_2: u8 = 10;
    pub const PERIPHERAL_3: u8 = 11;
    pub const MOUSE: u8 = 12;
    pub const FPU: u8 = 13;
    pub const PRIMARY_ATA: u8 = 14;
    pub const SECONDARY_ATA: u8 = 15;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pic_init() {
        unsafe {
            ProgrammableInterruptController::init();
        }
        // Jika tidak panic, initialization berhasil
    }
}
