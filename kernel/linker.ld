/* kernel/linker.ld */
OUTPUT_FORMAT(elf64-x86-64)
ENTRY(_start)

/* Limine bootloader akan handle relocation, jadi kita gunakan lower-half untuk now */
SECTIONS
{
    /* Start dari 0x400000 (standard ELF position) */
    . = 0x400000;
    
    .text : {
        *(.text .text.*)
    }

    . = ALIGN(0x1000);
    .rodata : {
        *(.rodata .rodata.*)
    }

    . = ALIGN(0x1000);
    .data : {
        *(.data .data.*)
    }

    . = ALIGN(0x1000);
    .bss : {
        *(COMMON)
        *(.bss .bss.*)
    }
    
    . = ALIGN(0x1000);
}