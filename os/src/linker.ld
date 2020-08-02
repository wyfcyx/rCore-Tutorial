
OUTPUT_ARCH(riscv)
ENTRY(_start)

BASE_ADDRESS = 0x80020000;

SECTIONS
{
    /* Load the kernel at this address: "." means the current address */
    . = BASE_ADDRESS;
    start = .;

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

    .bss : {
        sbss = .;
        *(.bss .bss.*)
        ebss = .;
    }

    PROVIDE(end = .);
}
