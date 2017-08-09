
bitflags! {
    struct PageFlags: u64 {
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


        const PG_4K_PHYSADDR_MASK = (1 << 12) - 1;
        const PG_2M_PHYSADDR_MASK = PG_PHYSADDR_MASK_4K << 9;
        const PG_1G_PHYSADDR_MASK = PG_PHYSADDR_MASK_2M << 9;

        const PGTBL_PHYSADDR_MASK = PG_2M_PHYSADDR_MASK;
        const PGDIR_PHYSADDR_MASK = PG_1G_PHYSADDR_MASK;
        const PDPTR_PHYSADDR_MASK = PD_PHYSADDR_MASK << 9;
    }
}

#[repr(C)]
struct PgStruct<T> {
    ents: [PageFlags; 512],
}
