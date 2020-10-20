OUTPUT_ARCH(riscv)
ENTRY(_start)

BASE_ADDRESS = 0x0;

SECTIONS
{
    /* Load the kernel at this address: "." means the current address */
    . = BASE_ADDRESS;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }
    . = ALIGN(4K);
    .rodata : {
        *(.rodata .rodata.*)
    }
    . = ALIGN(4K);
    .data : {
        *(.data .data.*)
        *(.eh_frame)
    }
    . = ALIGN(4K);
    .bss : {
        *(.sbss .bss .bss.*)
    }
    /DISCARD/ : {
        *(.debug*)
    }
}
