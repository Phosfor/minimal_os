# boot.S
# bootloader for SoS
# Stephen Marz
# 8 February 2019

# Disable generation of compressed instructions.
.option norvc

# Define a .text.init section. The .text.init is put at the
# starting address so that the entry _start is put at the RISC-V
# address 0x8000_0000.
.section .text.init

# Execution starts here.
.global _start
_start:
	# Disable linker instruction relaxation for the `la` instruction below.
	# This disallows the assembler from assuming that `gp` is already initialized.
	# This causes the value stored in `gp` to be calculated from `pc`.
	# The job of the global pointer is to give the linker the ability to address
	# memory relative to GP instead of as an absolute address.
.option push
.option norelax
	la		gp, _global_pointer
.option pop
	# SATP should be zero, but let's make sure. Each HART has its own
	# SATP register.
	//csrw	satp, zero

	# Set all bytes in the BSS section to zero.
# 	la 		t0, _bss_start
# 	la		t1, _bss_end
# 	bgeu	t0, t1, 2f
# 1:
# 	sd		zero, (t0)
# 	addi	t0, t0, 8
# 	bltu	t0, t1, 1b
# 2:
	# The stack grows from bottom to top, so we put the stack pointer
	# to the very end of the stack range.
	la		sp, _stack_end
	# Machine's exception program counter (MEPC) is set to `kinit`.
	jal kinit

.global halt
halt:
	wfi
	j halt
