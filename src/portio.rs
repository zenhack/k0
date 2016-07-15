// Port-IO instruction wrappers

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub unsafe fn outb(port : u16, data : u8) {
  asm!("outb $0, $1"
       :: "{al}"(data), "{dx}"(port)
       :: "volatile"
       );
}


#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub unsafe fn inb(port : u16) -> u8 {
  let result: u8;
  asm!("inb $1, $0"
       : "={al}"(result)
       : "{dx}"(port)
       :: "volatile"
       );
  return result
}
