/* Initial bringup; grub -> 64-bit mode.
 *
 * Much of this is based on [rust-longmode], though we do a few things
 * differently.
 */
.global bsp_start32
.global mboot_header

.extern bsp_start64

# Indicates the presence of the address fields in the multiboot header
.set MBOOT_AOUT_KLUDGE, (1<<16)

.set MBOOT_MAGIC, 0x1badb002
.set MBOOT_FLAGS, MBOOT_AOUT_KLUDGE
.set MBOOT_CHKSUM, (-(MBOOT_MAGIC + MBOOT_FLAGS))

.set PG_PRESENT, (1<<0)
.set PG_RW, (1<<1)
.set PG_HUGE, (1<<7)

.set CR0_PAGING_ENABLED, (1<<31)
.set CR4_PAE, (1<<5)

.set EFER_MSR, 0xc0000080
.set EFER_LONGMODE, (1<<8)

.set GDT_RW, (1<<41)
.set GDT_EXEC, (1<<43)
.set GDT_DESC_TYPE_CODE_DATA, (1<<44)
.set GDT_PRESENT, (1<<47)
.set GDT_64, (1<<53)

.set GDT_MAX_LIMIT, 0xffff | (0xf<<48)

.set GDT_OFFSET_CODE, 8
.set GDT_OFFSET_DATA, 16

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
	.int bsp_start32  /* entry point */
.text
.code32
bsp_start32:
	/* set up a stack: */
	mov $boot_stack, %esp
	mov 0, %ebp


build_page_tables:
	/* Identity map the first 2 MiB via a single huge page. That's enough
         * to cover our kernel and then some; we'll set something nicer up when
         * we're not so constrained. */
	movl $boot_pml3, %eax
	orl $(PG_PRESENT | PG_RW), %eax
	movl %eax, boot_pml4

	movl $boot_pml2, %eax
	orl $(PG_PRESENT | PG_RW), %eax
	movl %eax, boot_pml3

	movl $(PG_PRESENT | PG_RW | PG_HUGE), %eax
	movl %eax, boot_pml2

load_page_table:
	movl $boot_pml4, %eax
	movl %eax, %cr3
enable_pae:
	movl %cr4, %eax
	orl $CR4_PAE, %eax
	movl %eax, %cr4
enable_long_mode:
	movl $EFER_MSR, %ecx
	rdmsr
	orl $EFER_LONGMODE, %eax
	wrmsr

enable_paging:
	mov %cr0, %eax
	orl $CR0_PAGING_ENABLED, %eax
	movl %eax, %cr0

reload_gdt:
	lgdt gdt_ptr

	/* update segment selectors: */
	movw $GDT_OFFSET_DATA, %ax
	movw %ax, %ss
	movw %ax, %ds
	movw %ax, %es
	movw %ax, %fs
	movw %ax, %gs


	jmpl $GDT_OFFSET_CODE, $bsp_start64


.bss
.align 4096
boot_pml4:
	.space 4096
boot_pml3:
	.space 4096
boot_pml2:
	.space 4096
.space 8192 /* exact size is relatively arbitrary. */
boot_stack: /* stack grows down on x86, so this is the base of the stack. */
.section .rodata
gdt:
	.quad 0
	.quad GDT_DESC_TYPE_CODE_DATA | GDT_MAX_LIMIT | GDT_PRESENT | GDT_RW | GDT_EXEC | GDT_64
	.quad GDT_DESC_TYPE_CODE_DATA | GDT_MAX_LIMIT | GDT_PRESENT | GDT_RW | GDT_64
gdt_ptr:
	.word ((3*8) - 1) /* 3 entries at 8 bytes each, -1 because gdt weirdness */
	.quad gdt
