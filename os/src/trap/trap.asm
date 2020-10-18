.altmacro
.equ XLENB, 8
.macro LOAD a1, a2
	ld \a1, \a2*XLENB(sp)
.endm

.macro STORE a1, a2
	sd \a1, \a2*XLENB(sp)
.endm

.macro LOAD_N n
    LOAD x\n, \n
.endm

.macro STORE_N n
    STORE x\n, \n
.endm

	.section .text
	.align 4
	.globl __trapentry
__trapentry:
	csrrw sp, sscratch, sp
    bnez sp, __trap_from_user
__trap_from_kernel:
    csrr sp, sscratch
__trap_from_user:
    addi sp, sp, -34*XLENB
    STORE_N 1
    .set n, 5
    .rept 27
        STORE_N %n
        .set n, n + 1
    .endr

    csrrw s0, sscratch, x0
    csrr s1, sstatus
    csrr s2, sepc

    STORE s0, 2
    STORE s1, 32
    STORE s2, 33
	mv a0, sp
	csrr a1, scause
	csrr a2, stval
	jal trap_handler

	.globl __trapret
__trapret:
	LOAD s1, 32
    LOAD s2, 33
    andi s0, s1, 1 << 8
    bnez s0, __trap_ret_kernel
__trap_ret_user:
    addi s0, sp, 34*XLENB
    csrw sscratch, s0
__trap_ret_kernel:
    csrw sstatus, s1
    csrw sepc, s2
    LOAD_N 1
    .set n, 5
    .rept 27
        LOAD_N %n
        .set n, n + 1
    .endr

    LOAD x2, 2
	sret
