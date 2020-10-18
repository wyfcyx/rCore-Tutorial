.altmacro
.set REG_SIZE, 8
.set TRAPFRAME_SIZE, 34

.macro SAVE reg, offset
    sd \reg, \offset * REG_SIZE(sp)
.endm

.macro LOAD reg, offset
    ld \reg, \offset * REG_SIZE(sp)
.endm

.macro SAVE_N n
    SAVE x\n, n
.endm

.macro LOAD_N n
    LOAD x\n, n
.endm

.macro SAVE_ALL_EXCEPT_SP
    addi sp, sp, -TRAPFRAME_SIZE * REG_SIZE
    SAVE_N 1
    .set n, 5
    .rept 27
        SAVE_N %n
        .set n, n + 1
    .endr
    csrr t0, sstatus
    csrr t1, sepc
    SAVE t0, 32
    SAVE t1, 33
.endm

.macro RESTORE_ALL_EXCEPT_SP
    LOAD t0, 32
    LOAD t1, 33
    csrw sstatus, t0
    csrw sepc, t1
    LOAD_N 1
    .set n, 5
    .rept 27
        LOAD_N %n
        .set n, n + 1
    .endr
    addi sp, sp, TRAPFRAME_SIZE * REG_SIZE
.endm

    .section .text
    .align 4
    .globl __trapentry
__trapentry:
    SAVE_ALL_EXCEPT_SP
    mv a0, sp
    jal trap_handler

    .globl __trapret
__trapret:
    RESTORE_ALL_EXCEPT_SP
    sret
