
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Info {
    pub flags: InfoFlags,
    mem_lower: u32,
    mem_upper: u32,
    boot_dev: u32,
    cmdline: u32,
    mods_count: u32,
    mods_addr: u32,
    mmap_length: u32,
    mmap_addr: u32,
}

bitflags! {
    pub struct InfoFlags: u32 {
        const HAVE_MEM          = 0b00000000000000001;
        const HAVE_BOOTDEV      = 0b00000000000000010;
        const HAVE_CMDLINE      = 0b00000000000000100;
        const HAVE_MODS         = 0b00000000000001000;
        const HAVE_SYMS         = 0b00000000000110000;
        const HAVE_MMAP         = 0b00000000001000000;
        const HAVE_DRIVES       = 0b00000000010000000;
        const HAVE_CONFIG_TBL   = 0b00000000100000000;
        const HAVE_BOOTLDR_NAME = 0b00000001000000000;
        const HAVE_APM_TBL      = 0b00000010000000000;
        const HAVE_VBE          = 0b00000100000000000;
    }
}

impl Info {

    /// Return Some(mem_lower, mem_upper), or None if the mem_*
    /// fields of the info structure are not present.
    pub fn mem_info(&self) -> Option<(u32, u32)> {
        if self.flags.contains(HAVE_MEM) {
            Some((self.mem_lower, self.mem_upper))
        } else {
            None
        }
    }
}
