use super::paging::PgStruct;

#[cfg(target_arch = "x86_64")]
pub unsafe fn get_cr3() -> *mut PgStruct {
    let result: *mut PgStruct;
    asm!("movq %cr3, %rax" : "={rax}"(result));
    return result;
}
