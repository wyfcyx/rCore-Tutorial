    .section .text.entry
    .globl _start
_start:
    la sp, boot_stack
    mv a1, sp
    call rust_main

    .section .bss.stack
    .align 12
    .globl boot_stack
boot_stack:
    .space 4096 * 4 * 8
    .globl boot_stack_top
boot_stack_top:
