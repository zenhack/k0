.global bsp_start
.global mboot_header

# Indicates the presence of the address fields in the multiboot header
.set MBOOT_AOUT_KLUDGE, (1<<16)

.set MBOOT_MAGIC, 0x1badb002
.set MBOOT_FLAGS, MBOOT_AOUT_KLUDGE
.set MBOOT_CHKSUM, (-(MBOOT_MAGIC + MBOOT_FLAGS))

.align 4
.section .mboot
mboot_header:
	.int MBOOT_MAGIC
	.int MBOOT_FLAGS
	.int MBOOT_CHKSUM
	.int mboot_header /* location of mboot header */
	.int code         /* start of text segment */
	.int bss          /* end of data section */
	.int kend         /* end of bss */
	.int bsp_start    /* entry point */
.text
.code32
bsp_start:
	hlt
	jmp bsp_start

.bss
.space 8192 /* exact size is relatively arbitrary. */
boot_stack: /* stack grows down on x86, so this is the base of the stack. */
