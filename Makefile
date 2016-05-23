
TARGET ?= x86_64-unknown-linux-gnu

AS ?= $(TARGET)-as
LD ?= $(TARGET)-ld

rust_src := $(shell find src/ -name '*.rs')
rust_lib := target/$(TARGET)/debug/libk0.a

# "Standard" targets; things which are convential for most Makefiles.
all: k.elf
clean:
	rm -f *.o k.elf boot_iso/boot/k.elf boot.iso
	cargo clean


# Boot media #
boot.iso: boot_iso/boot/k.elf boot_iso/boot/grub/grub.cfg
	grub-mkrescue -o $@ boot_iso
boot_iso/boot/k.elf: k.elf
	cp $< $@

# The kernel proper. Most of the source is rust, which gets handled by cargo;
# all that we have to handle is the assembly source and the linking.
k.elf: boot.o link.ld $(rust_lib)
	$(LD) -o $@ boot.o $(rust_lib) -T link.ld
boot.o: boot.s
	$(AS) -o $@ $<
$(rust_lib): $(rust_src)
	cargo build --target=$(TARGET)

# "Run" targets; not building anything, just convenience targets for other
# tasks. The *-gdb variants start qemu with remote debugging. The *-grub-*
# variants boot from a cdrom with grub, whereas the others use qemu's built-in
# multiboot support.
qemu-run: k.elf
	qemu-system-x86_64 -kernel $<
qemu-gdb: k.elf
	qemu-system-x86_64 -kernel $< -s -S
qemu-grub-run: boot.iso
	qemu-system-x86_64 -cdrom $<
qemu-grub-gdb: boot.iso
	qemu-system-x86_64 -cdrom $< -s -S


.PHONY: qemu-run qemu-gdb clean
.SUFFIXES:
