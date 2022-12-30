# boot.S

.option norvc
.section .data

.section .text.init
.global _start

_start:
    # Not every hart should be used for bootstrapping.
    # Make these wait for an IPI

    # read hartID into t0
    # csrr <=> control status register read
    csrr t0, mhartid
    # branch not equal zero
    bnez t0, 4f

    # make sure satp is 0
    # (it should be)
    # satp <=> superviser address translation and protection
    # used to controll MMU eventually! Disabled for now
    # crsw <=> control status registert write
    csrw satp, zero

.option push
.option norelax
    la  gp, _global_pointer

.option pop
    # clear the bss section.
    # a0 = _bss_start
    la  a0, _bss_start
    # a0 = _bss_end
    la  a1, _bss_end
    bgeu a0, a1, 2f

1:
    # *(u64)a0 = 0
    # sd <=> store doubleword
    sd  zero, (a0)
    # a0 += 8
    addi a0, a0, 8
    # when a0 < a1 then jump 1b
    bltu a0, a1, 1b
2:
    la sp, _stack
    li    t0, (0b11<<11) | (1<<7) | (1<<3)
    csrw  mstatus, t0
    la    t1, kmain
    csrw  mepc, t1
    # set mtvec to asm_trap_vector
    la    t2, asm_trap_vector
    csrw mtvec, t2

    li		t3, (1 << 3) | (1 << 7) | (1 << 11)
    csrw	mie, t3
    la		ra, 4f
    mret

4:
    wfi
    j 4b

