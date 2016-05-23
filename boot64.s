.text
.code64
.global bsp_start64
bsp_start64:
	movq $0x2f592f412f4b2f4f, %rax
	movq %rax, 0xb8000
hang:
	hlt
	jmp hang
