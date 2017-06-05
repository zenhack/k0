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
	xchgw %bx, %bx
	jmp isr_stub
