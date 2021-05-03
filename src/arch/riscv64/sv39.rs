use crate::{
    Arch,
    MemoryArea,
    PhysicalAddress,
    VirtualAddress,
};

#[derive(Clone, Copy)]
pub struct RiscV64Sv39Arch;

impl Arch for RiscV64Sv39Arch {
    const PAGE_SHIFT: usize = 12; // 4096 bytes
    const PAGE_ENTRY_SHIFT: usize = 9; // 512 entries, 8 bytes each
    const PAGE_LEVELS: usize = 3; // L0, L1, L2

    //TODO
    const ENTRY_ADDRESS_SHIFT: usize = 52;
    const ENTRY_FLAG_DEFAULT_PAGE: usize
        = Self::ENTRY_FLAG_PRESENT
        | 1 << 1 // Read flag
        ;
    const ENTRY_FLAG_DEFAULT_TABLE: usize
        = Self::ENTRY_FLAG_PRESENT
        ;
    const ENTRY_FLAG_PRESENT: usize = 1 << 0;
    const ENTRY_FLAG_READONLY: usize = 0;
    const ENTRY_FLAG_READWRITE: usize = 1 << 2;
    const ENTRY_FLAG_USER: usize = 1 << 4;
    const ENTRY_FLAG_NO_EXEC: usize = 0;
    const ENTRY_FLAG_EXEC: usize = 1 << 3;

    //TODO: adjust to match x86_64?
    const PHYS_OFFSET: usize = 0xfffffe0000000000;

    unsafe fn init() -> &'static [MemoryArea] {
        unimplemented!("RiscV64Sv39Arch::init unimplemented");
    }

    #[inline(always)]
    unsafe fn invalidate(address: VirtualAddress) {
        //TODO: can one address be invalidated?
        Self::invalidate_all();
    }

    #[inline(always)]
    unsafe fn table() -> PhysicalAddress {
        let satp: usize;
        asm!("csrr {0}, satp", out(reg) satp);
        PhysicalAddress::new(
            (satp & 0x0000_0FFF_FFFF_FFFF) << Self::PAGE_SHIFT // Convert from PPN
        )
    }

    #[inline(always)]
    unsafe fn set_table(address: PhysicalAddress) {
        let satp =
            (8 << 60) | // Sv39 MODE
            (address.data() >> Self::PAGE_SHIFT); // Convert to PPN (TODO: ensure alignment)
        asm!("csrw satp, {0}", in(reg) satp);
    }
}

#[cfg(test)]
mod tests {
    use crate::Arch;
    use super::RiscV64Sv39Arch;

    #[test]
    fn constants() {
        assert_eq!(RiscV64Sv39Arch::PAGE_SIZE, 4096);
        assert_eq!(RiscV64Sv39Arch::PAGE_OFFSET_MASK, 0xFFF);
        assert_eq!(RiscV64Sv39Arch::PAGE_ADDRESS_SHIFT, 39);
        assert_eq!(RiscV64Sv39Arch::PAGE_ADDRESS_SIZE, 0x0000_0080_0000_0000);
        assert_eq!(RiscV64Sv39Arch::PAGE_ADDRESS_MASK, 0x0000_007F_FFFF_F000);
        assert_eq!(RiscV64Sv39Arch::PAGE_ENTRY_SIZE, 8);
        assert_eq!(RiscV64Sv39Arch::PAGE_ENTRIES, 512);
        assert_eq!(RiscV64Sv39Arch::PAGE_ENTRY_MASK, 0x1FF);
        assert_eq!(RiscV64Sv39Arch::PAGE_NEGATIVE_MASK, 0xFFFF_FF80_0000_0000);

        assert_eq!(RiscV64Sv39Arch::ENTRY_ADDRESS_SIZE, 0x0010_0000_0000_0000);
        assert_eq!(RiscV64Sv39Arch::ENTRY_ADDRESS_MASK, 0x000F_FFFF_FFFF_F000);
        assert_eq!(RiscV64Sv39Arch::ENTRY_FLAGS_MASK, 0xFFF0_0000_0000_0FFF);

        assert_eq!(RiscV64Sv39Arch::PHYS_OFFSET, 0xFFFF_FE00_0000_0000);
    }
}
