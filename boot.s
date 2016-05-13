.global bsp_start
.global mboot_header

.set MBOOT_MAGIC, 0x1badb002
.set MBOOT_FLAGS, 0
.set MBOOT_CHKSUM, (-(MBOOT_MAGIC + MBOOT_FLAGS))

.align 4
.section .mboot
mboot_header:
	.long MBOOT_MAGIC
	.long MBOOT_FLAGS
	.long MBOOT_CHKSUM
.text
.code32
bsp_start:
	int $3
	hlt
	jmp bsp_start

.bss
.space 8192 /* exact size is relatively arbitrary. */
boot_stack: /* stack grows down on x86, so this is the base of the stack. */
