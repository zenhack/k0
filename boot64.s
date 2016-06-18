.text
.code64
.global bsp_start64
bsp_start64:
	movq $_stack_base-8, %rsp
	movq $0, %rbp
	call bsp_main

hang:
	hlt
	jmp hang
.bss
.space 4096
_stack_base:
