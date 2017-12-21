use core::fmt;

bitflags! {
    #[repr(C)]
    pub struct PageFlags: u64 {
        // Meaningful for all paging structures:
        const PRESENT      = 1 <<  0;
        const RW           = 1 <<  1;
        const USER         = 1 <<  2;
        const PWT          = 1 <<  3;
        const PCD          = 1 <<  4;
        const ACCESSED     = 1 <<  5;
        const EXEC_DISABLE = 1 << 63;

        // Not valid for level 4:
        const BIG_PAGE = 1 << 7; // 2MiB or 1GiB page, depending on level.

        // Only valid for pointers to pages, not subtrees.
        const DIRTY    =      1 <<  6;
        const GLOBAL   =      1 <<  8;
        const PAT      =      1 << 12;
        const PKE_MASK = 0b1111 << 59;


        // TODO: make these a bit prettier; had them detrmined in terms of
        // smaller constants, but wouldn't type check.
        const PG_4K_PHYSADDR_MASK =  (1 << 12) - 1       ;
        const PG_2M_PHYSADDR_MASK = ((1 << 12) - 1) <<  9;
        const PG_1G_PHYSADDR_MASK = ((1 << 12) - 1) << 18;

        const PGTBL_PHYSADDR_MASK = ((1 << 12) - 1) <<  9;
        const PGDIR_PHYSADDR_MASK = ((1 << 12) - 1) << 18;
        const PDPTR_PHYSADDR_MASK = ((1 << 12) - 1) << 27;

        const PML4_PHYSADDR_MASK = !((1 << 63) | ((1 << 12 - 1)));
    }
}

pub struct CR3Value {
    flags: PageFlags
}

pub unsafe fn get_cr3() -> CR3Value {
    let result: CR3Value;
    asm!("movq %cr3, %rax" : "={rax}"(result));
    return result;
}

impl CR3Value {
    pub fn pml4_addr(&self) -> *mut PgStruct {
        (self.flags & PML4_PHYSADDR_MASK).bits() as *mut PgStruct
    }
}

#[repr(C)]
pub struct PgStruct {
    ents: [PageFlags; 512],
}

// We implement this manually, since [Foo; 512] doesn't implement Debug (or any
// array with length > 32
impl fmt::Debug for PgStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(f.write_str("PgStruct{"));
        try!((&self.ents[..]).fmt(f));  // slices *do* impl Debug.
        f.write_str("}")
    }
}
