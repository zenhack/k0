.code64

.macro isr num errcode
.globl isr\num
isr\num:
	.if \errcode
	pushq $0
	.endif
	pushq $\num
	jmp isr_stub
.endm

.include "isr_gen.s"

isr_stub:
	pushq %rax
	pushq %rdi
	pushq %rsi
	pushq %rdx
	pushq %rcx
	pushq %r8
	pushq %r9
	pushq %r10
	pushq %r11

	movq %rsp, %rdi

	call int_handler_main

	popq %r11
	popq %r10
	popq %r9
	popq %r8
	popq %rcx
	popq %rdx
	popq %rsi
	popq %rdi
	popq %rax

	/* Pop the interrupt number and the error code. */
	addq $16, %rsp

	iretq
