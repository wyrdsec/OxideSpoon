# boot.S
# bootloader for SoS
# Stephen Marz
# 8 February 2019

# Note most comments are for me because
# ASM code makes my brain smooth

# Reference links I used
# 1. https://msyksphinz-self.github.io/riscv-isadoc/html/index.html
# 2. https://sourceware.org/binutils/docs-2.31/as/RISC_002dV_002dDirectives.html#RISC_002dV_002dDirectives
# 3. https://github.com/riscv/riscv-isa-manual/


# disables the generation of compressed instructions, probably because booting is fragile
.option norvc
.section .data

# _start here dumbass, not you the emulator
.section .text.init
.global _start

_start:
    # Read mhartid, reference the RiscV manual section 3.1.5 for better info
    # mhartid contains the integer ID of the hardware thread running the code.
    csrr	t0, mhartid

    # If this code is not running on mhartid == 0, jump to label 3
    bnez    t0, 3f

    # Supervisor Address Translation and Protection (satp) Register
    # Section 5.1.11
    # controls supervisor-mode address translation and protection
    # Should already be zero, but set it anyway
    csrw	satp, zero
.option push
# norelax, disabled linker relaxation
.option norelax
    # pseudo instruction, load address _global_pointer into register gp
    la      gp, _global_pointer
.option pop

    la      a0, _bss_start
    la      a1, _bss_end

    # If    _bss_start >= _bss_end
    bgeu    a0, a1, 2f

1:
    # sd, store doubleword (8 bytes)
    sd      zero, (a0)

    # Continue moving down BSS until all bytes are 0
    addi    a0, a0, 8
    bltu    a0, a1, 1b

2:
    # Control registers, set the stack, mstatus, mepc,
    # and mtvec to return to the main function.
    # li    t5, 0xffff;
    #csrw    medeleg, t5
    #csrw    mideleg, t5
    la		sp, _stack_start

    # li, load immediate
    # Set bits [11,12] to 1, which sets mstatus register MPP to 3
    #   This sets the CPU to machine mode
    # Set bit 7 to 1, which sets the mstatus register MPIE to 1
    # Set bit 3 to 1, which set the mstatus register MIE to 1
    #   Allows interrupts at course level 
    li      t0, (0b11 << 11) | (1 << 7) | (1 << 3)
    csrw    mstatus, t0

    # Load our rust kernel main
    la      t1, kmain
    # mepc, Machine Exception Program Counter
    csrw	mepc, t1

    # Loads rust trap function
    la		t2, asm_trap_vector
    # mtvec, Machine Trap-Vector Base-Address Register
    csrw	mtvec, t2

    # MIP values MSIE, MTIE, and MEIE are all set to 1
    # Enables machine level software, timer, and hardware interrupts
    li		t3, (1 << 3) | (1 << 7) | (1 << 11)
    csrw	mie, t3

    la		ra, 4f

    # use mret so mstatus is set properly
    mret

3:
    # wait for interrupt
    wfi
    j 3b

4:
    wfi
    j   4b