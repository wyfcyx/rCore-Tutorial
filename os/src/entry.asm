    .section .text.entry
    .globl _start
# 目前 _start 的功能：将预留的栈空间写入 $sp，然后跳转至 rust_main
_start:
    # 计算 boot_page_table 的物理页号
    //sfence.vma

    # 加载栈地址
    add t0, a0, 1
    slli t0, t0, 14
    lui sp, %hi(boot_stack)
    add sp, sp, t0

    # 跳转至 rust_main
    lui t0, %hi(rust_main)
    addi t0, t0, %lo(rust_main)
    jr t0

    # 回忆：bss 段是 ELF 文件中只记录长度，而全部初始化为 0 的一段内存空间
    # 这里声明字段 .bss.stack 作为操作系统启动时的栈
    .section .bss.stack
    .global boot_stack
boot_stack:
    # 16K 启动栈大小
    .space 4096 * 4 * 2
    .global boot_stack_top
boot_stack_top:
    # 栈结尾

