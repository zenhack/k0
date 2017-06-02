.code32

.macro isr num errcode
.globl isr\num
isr\num:
	.if \errcode
	pushl $0
	.endif
	pushl $\num
	jmp isr_stub
.endm

.include "isr_gen.s"

isr_stub:
	hlt
	jmp isr_stub
