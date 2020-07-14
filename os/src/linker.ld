OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    . = BASE_ADDRESS;

    kernel_start = .;
    text_start = .;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }

    rodata_start = .;
    .data : {
        *(.data .data.*)
    }
    bss_start = .;
    .bss : {
        *(.sbss .bss .bss.*)
    }
    kernel_end = .;
}
