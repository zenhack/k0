
TARGET ?= x86_64-elf

AS ?= $(TARGET)-as
LD ?= $(TARGET)-ld

rust_src := $(shell find src/ -name '*.rs') build.rs
rust_lib := target/x86_64-k0/debug/libk0.a

asm_objects := boot32.o boot64.o isr.o

# "Standard" targets; things which are convential for most Makefiles.
all: k.elf32
clean:
	find -name 'k.elf*' -delete
	rm -f *.o *_gen.s boot.iso
	xargo clean

# (Generated) interrupt handler stubs
isr.o: isr.s isr_gen.s
	$(AS) -o $@ isr.s
isr_gen.s: make_isrs.sh
	./make_isrs.sh > $@


# Boot media #
boot.iso: boot_iso/boot/k.elf32 boot_iso/boot/grub/grub.cfg
	grub-mkrescue -o $@ boot_iso
boot_iso/boot/k.elf32: k.elf32
	cp $< $@

# The kernel proper. Most of the source is rust, which gets handled by xargo;
# all that we have to handle is the assembly source and the linking.
k.elf64: $(asm_objects) link.ld $(rust_lib)
	$(LD) -o $@ $(asm_objects) $(rust_lib) -T link.ld --gc-sections

# The 64-bit elf doesn't play nicely with grub; imperically it places the
# multiboot header *way* after the 8KiB mark in the file (even though it's still
# the first actual thing...) and I'm not sure grub knows enough to load an
# elf64 by itself anyway. To get around this, we copy everything into an elf32,
# which does the job:
k.elf32: k.elf64
	objcopy -O elf32-i386 $< $@

%.o: %.s
	$(AS) -o $@ $<
$(rust_lib): $(rust_src)
	# Set RUST_TARGET_PATH to work around
	# https://github.com/japaric/xargo/issues/44
	RUST_TARGET_PATH=$(PWD) xargo build --target=x86_64-k0

qemu_flags := -serial stdio

# "Run" targets; not building anything, just convenience targets for other
# tasks. qemu-gdb starts qemu with remote debugging. Each of these boots from
# the custom grub ISO.
#
# N.B. There used to be targets that used qemu's -kernel option insead of the
# cd, but it seems to do odd things with the memory map, so we've stopped
# supporting it.
bochs-run: boot.iso
	bochs
qemu-run: boot.iso
	qemu-system-x86_64 $(qemu_flags) -cdrom $<
qemu-gdb: boot.iso
	qemu-system-x86_64 $(qemu_flags) -cdrom $< -s -S


.PHONY: all clean bochs-run qemu-run qemu-gdb qemu-grub-run qemu-grub-gdb
.SUFFIXES:
