
TARGET ?= x86_64-unknown-linux-gnu

AS ?= $(TARGET)-as
LD ?= $(TARGET)-ld

rust_src := $(shell find src/ -name '*.rs')
rust_lib := target/$(TARGET)/release/libk0.a

asm_objects := boot32.o boot64.o

# "Standard" targets; things which are convential for most Makefiles.
all: k.elf32
clean:
	find -name 'k.elf*' -delete
	rm -f *.o boot.iso
	cargo clean


# Boot media #
boot.iso: boot_iso/boot/k.elf32 boot_iso/boot/grub/grub.cfg
	grub-mkrescue -o $@ boot_iso
boot_iso/boot/k.elf32: k.elf32
	cp $< $@

# The kernel proper. Most of the source is rust, which gets handled by cargo;
# all that we have to handle is the assembly source and the linking.
k.elf64: $(asm_objects) link.ld $(rust_lib)
	$(LD) -o $@ $(asm_objects) $(rust_lib) -T link.ld

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
	cargo build --release --target=$(TARGET)

# "Run" targets; not building anything, just convenience targets for other
# tasks. The *-gdb variants start qemu with remote debugging. The qemu-grub-*
# variants boot qemu from a cdrom with grub, whereas the other qemu targets
# qemu's built-in multiboot support. bochs-run boots bochs from the cdrom.
bochs-run: boot.iso
	bochs

qemu_flags := -serial stdio

qemu-run: k.elf32
	qemu-system-x86_64 $(qemu_flags) -kernel $<
qemu-gdb: k.elf32
	qemu-system-x86_64 $(qemu_flags) -kernel $< -s -S
qemu-grub-run: boot.iso
	qemu-system-x86_64 $(qemu_flags) -cdrom $<
qemu-grub-gdb: boot.iso
	qemu-system-x86_64 $(qemu_flags) -cdrom $< -s -S


.PHONY: qemu-run qemu-gdb clean
.SUFFIXES:
