

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub unsafe fn breakpoint() {
    asm!("xchgw %bx, %bx");
}
