.text
.code64
.global bsp_start64
bsp_start64:
	movq $_stack_base-8, %rsp
	movq $0, %rbp

	/* Pass the multiboot info structure: */
	movq $0, %rdi
	movl %ebx, %edi

	call bsp_main

hang:
	hlt
	jmp hang
.bss
.space 4096
_stack_base:
