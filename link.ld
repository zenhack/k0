/* This file is copypasta from [zero][1], so some of the comments it refer to
 * features and code that doesn't exist yet. It's planned, so I haven't
 * removed the comments.
 *
 * [1]: https://github.com/zenhack/zero
 */


/* The entry point as marked in the executable is called by the bootloader.
 * There's another entry point, ap_start, which the application processors
 * (processors started after the first one) jump to, but this is handled by the
 * already running kernel. */
ENTRY(bsp_start32)

/* Make sure we don't overflow the region of memory we're loading into.
 * The region is 408.5K, but ld's syntax won't do fractional values, hence the
 * awkward specification: */
MEMORY { kimage (rwx): ORIGIN = 0x7e00, LENGTH = 408K + 512 }

SECTIONS
{

	/* There's an area of ram guaranteed to be present and free for
	 * use starting at 0x7e00, which is 480.5 KiB in size
	 * [osdev/Memory_Map]. We load our kernel at the first page boundary
	 * within that region.
	 *
	 * The kernel needs to be loaded below the 1 MiB mark, so that
	 * additional CPUs can jump to something inside the kernel image when
	 * they are started. */
	. = 0x7e00;
	.mboot :
	{
		. = ALIGN(4096);
		KEEP(*(.mboot))
	} >kimage
	.text :
	{
		. = ALIGN(4096);
		code = .; _code = .; __code = .;
		*(.text)
		/* rust puts each of it's modules in it's own section like
		 * `.text.<C++-style-mangled-module-name>`; as such *(.text)
		 * won't get much on its own (just asm). So we do *(.text.*),
		 * *(.data.*), etc. as well. */
		*(.text.*)
	} >kimage
	.data :
	{
		. = ALIGN(4096);
		data = .; _data = .; __data = .;
		*(.data)
		*(.data.*)
		*(.rodata)
		*(.rodata.*)
	} >kimage
	.bss :
	{
		. = ALIGN(4096);
		bss = .; _bss = .; __bss = .;
		*(.bss)
		*(.bss.*)
		. = ALIGN(4096);
	} >kimage
	kend = .; _kend = .; __kend = .;
}

