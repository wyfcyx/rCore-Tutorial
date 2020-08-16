OUTPUT_ARCH(riscv)
ENTRY(_start)

BASE_ADDRESS = 0x80200000;

SECTIONS
{
    /* Load the kernel at this address: "." means the current address */
    . = BASE_ADDRESS;
    kernel_start = .;

    . = ALIGN(4K);

    .text : {
        stext = .;
        *(.text.entry)
        *(.text .text.*)
        etext = .;
    }

    . = ALIGN(4K);

    .rodata : {
        srodata = .;
        *(.rodata .rodata.*)
        erodata = .;
    }

    . = ALIGN(4K);

    .data : {
        sdata = .;
        *(.data .data.*)
        edata = .;
    }

    . = ALIGN(4K);

    .stack : {
        *(.bss.stack)
    }

    . = ALIGN(4K);

    .bss : {
        sbss = .;
        *(.sbss .bss .bss.*)
        ebss = .;
    }

    . = ALIGN(4K);

    PROVIDE(kernel_end = .);
}
