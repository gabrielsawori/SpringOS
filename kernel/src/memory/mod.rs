// Basic Memory Management

// Simple physical memory allocator menggunakan bitmap
// Ini adalah basic allocator untuk Phase 1
pub struct PhysicalMemoryAllocator {
    bitmap: *mut u8,
    bitmap_size: usize,
    frame_count: usize,
}

impl PhysicalMemoryAllocator {
    // Inisialisasi allocator dengan memory range
    // start: Alamat mulai physical memory
    // size: Total ukuran memory dalam bytes
    pub unsafe fn new(start: u64, size: u64) -> Self {
        let frame_size: u64 = 4096; // 4KB per frame
        let frame_count = (size / frame_size) as usize;
        let bitmap_size = (frame_count + 7) / 8; // Round up ke byte terdekat

        // Bitmap ditempatkan di awal memory yang dialokasikan
        let bitmap = start as *mut u8;

        // Clear bitmap (semua memory free)
        for i in 0..bitmap_size {
            *bitmap.add(i) = 0;
        }

        let allocator = PhysicalMemoryAllocator {
            bitmap,
            bitmap_size,
            frame_count,
        };
        allocator
    }

    // Allocate satu frame physical memory
    pub unsafe fn allocate_frame(&mut self) -> Option<u64> {
        for byte_idx in 0..self.bitmap_size {
            let byte = *self.bitmap.add(byte_idx);

            if byte != 0xFF {
                // Ada bit yang 0 (free frame)
                for bit_idx in 0..8 {
                    if (byte & (1 << bit_idx)) == 0 {
                        // Tandai sebagai allocated
                        *self.bitmap.add(byte_idx) |= 1 << bit_idx;

                        let frame_index = byte_idx * 8 + bit_idx;
                        let frame_addr = (frame_index as u64) * 4096;
                        return Some(frame_addr);
                    }
                }
            }
        }
        None
    }

    // Free satu frame physical memory
    pub unsafe fn free_frame(&mut self, frame_addr: u64) {
        let frame_index = (frame_addr / 4096) as usize;
        let byte_idx = frame_index / 8;
        let bit_idx = frame_index % 8;

        if byte_idx < self.bitmap_size {
            *self.bitmap.add(byte_idx) &= !(1 << bit_idx);
        }
    }

    pub fn get_total_frames(&self) -> usize {
        self.frame_count
    }

    pub fn get_bitmap_size(&self) -> usize {
        self.bitmap_size
    }
}

// Heap allocator sederhana untuk kernel
pub struct KernelHeap {
    start: u64,
    end: u64,
    current: u64,
}

impl KernelHeap {
    pub const fn new(start: u64, size: u64) -> Self {
        KernelHeap {
            start,
            end: start + size,
            current: start,
        }
    }

    // Simple linear allocator (bisa di-improve dengan buddy allocator nanti)
    pub fn allocate(&mut self, size: usize, align: usize) -> Option<*mut u8> {
        // Align current pointer
        let aligned_current = ((self.current + align as u64 - 1) / align as u64) * align as u64;

        if aligned_current + size as u64 > self.end {
            return None; // Out of memory
        }

        let ptr = aligned_current as *mut u8;
        self.current = aligned_current + size as u64;

        Some(ptr)
    }

    pub fn remaining(&self) -> u64 {
        if self.current < self.end {
            self.end - self.current
        } else {
            0
        }
    }
}

// Virtual memory structures
#[repr(C)]
#[repr(C, align(4096))]
pub struct PageTable {
    entries: [u64; 512], // 512 entries per page table (9 bits untuk addressing)
}

impl PageTable {
    #[allow(dead_code)]
    pub const fn new() -> Self {
        PageTable { entries: [0; 512] }
    }

    #[allow(dead_code)]
    pub fn set_entry(&mut self, index: usize, value: u64) {
        if index < 512 {
            self.entries[index] = value;
        }
    }

    #[allow(dead_code)]
    pub fn get_entry(&self, index: usize) -> u64 {
        if index < 512 {
            self.entries[index]
        } else {
            0
        }
    }
}

// Page table flags
#[allow(dead_code)]
pub mod page_flags {
    pub const PRESENT: u64 = 1 << 0;
    pub const WRITE: u64 = 1 << 1;
    pub const USER: u64 = 1 << 2;
    pub const WRITE_THROUGH: u64 = 1 << 3;
    pub const CACHE_DISABLE: u64 = 1 << 4;
    pub const ACCESSED: u64 = 1 << 5;
    pub const DIRTY: u64 = 1 << 6;
    pub const HUGE_PAGE: u64 = 1 << 7;
    pub const GLOBAL: u64 = 1 << 8;
    pub const EXECUTE_DISABLE: u64 = 1 << 63;
}
